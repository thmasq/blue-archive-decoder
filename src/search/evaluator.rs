use crate::search::ast::{BinaryOp, CompiledPattern, Expr, FunctionId, UnaryOp};
use chrono::NaiveDate;
use sqlite_wasm_reader::Value;
use std::collections::HashSet;

pub type EvalResult = Result<Value, String>;

/// Resolves a string name to a specific FunctionId.
fn resolve_function_id(id: &FunctionId) -> FunctionId {
    if let FunctionId::Unknown(name) = id {
        match name.to_lowercase().as_str() {
            "int" => FunctionId::Int,
            "float" => FunctionId::Float,
            "str" | "string" => FunctionId::String,
            "date" => FunctionId::Date,
            "len" => FunctionId::Len,
            "lower" => FunctionId::Lower,
            "upper" => FunctionId::Upper,
            "contains" => FunctionId::Contains,
            "is_null" => FunctionId::IsNull,
            _ => FunctionId::Unknown(name.clone()),
        }
    } else {
        id.clone()
    }
}

/// Checks if an expression is static (contains no column references or row indices).
fn is_constant(expr: &Expr) -> bool {
    match expr {
        Expr::Null | Expr::Bool(_) | Expr::Integer(_) | Expr::Float(_) | Expr::String(_) => true,
        Expr::Column(_) | Expr::ColumnIndex(_) | Expr::RowIndex => false,
        Expr::Binary(lhs, _, rhs) => is_constant(lhs) && is_constant(rhs),
        Expr::Unary(_, inner) => is_constant(inner),
        Expr::Call(_, args) => args.iter().all(is_constant),
        Expr::List(items) => items.iter().all(is_constant),
        Expr::Range(start, end) => is_constant(start) && is_constant(end),
        Expr::RegexMatch(lhs, _) => is_constant(lhs),
        Expr::SetMatch(lhs, _) => is_constant(lhs),
    }
}

pub fn optimize(expr: Expr) -> Result<Expr, String> {
    let mut expr = match expr {
        Expr::Binary(lhs, op, rhs) => {
            let lhs_opt = optimize(*lhs)?;
            let rhs_opt = optimize(*rhs)?;

            if op == BinaryOp::Regex {
                if let Expr::String(ref pat) = rhs_opt {
                    let re = regex::Regex::new(pat).map_err(|e| format!("Invalid Regex: {e}"))?;
                    return Ok(Expr::RegexMatch(
                        Box::new(lhs_opt),
                        CompiledPattern {
                            re,
                            original: pat.clone(),
                        },
                    ));
                }
            }

            if matches!(op, BinaryOp::In | BinaryOp::NotIn) {
                if let Expr::List(items) = &rhs_opt {
                    let has_range = items.iter().any(|i| matches!(i, Expr::Range(..)));

                    if items.iter().all(is_constant) && !has_range {
                        let mut set = HashSet::new();
                        for item in items {
                            if let Ok(val) = evaluate(item, &[], &[], 0) {
                                let s = match val {
                                    Value::Text(t) => t,
                                    Value::Integer(i) => i.to_string(),
                                    Value::Real(f) => f.to_string(),
                                    Value::Blob(_) => "<Blob>".to_string(),
                                    Value::Null => "null".to_string(),
                                };
                                set.insert(s);
                            }
                        }

                        let match_expr = Expr::SetMatch(Box::new(lhs_opt.clone()), set);

                        return Ok(if matches!(op, BinaryOp::NotIn) {
                            Expr::Unary(UnaryOp::Not, Box::new(match_expr))
                        } else {
                            match_expr
                        });
                    }
                }
            }

            match op {
                BinaryOp::And => match (&lhs_opt, &rhs_opt) {
                    (Expr::Bool(true), _) => return Ok(rhs_opt),
                    (Expr::Bool(false), _) => return Ok(Expr::Bool(false)),
                    (_, Expr::Bool(true)) => return Ok(lhs_opt),
                    _ => {}
                },
                BinaryOp::Or => match (&lhs_opt, &rhs_opt) {
                    (Expr::Bool(true), _) => return Ok(Expr::Bool(true)),
                    (Expr::Bool(false), _) => return Ok(rhs_opt),
                    _ => {}
                },
                _ => {}
            }

            Expr::Binary(Box::new(lhs_opt), op, Box::new(rhs_opt))
        }

        Expr::Unary(op, inner) => Expr::Unary(op, Box::new(optimize(*inner)?)),

        Expr::Range(start, end) => {
            Expr::Range(Box::new(optimize(*start)?), Box::new(optimize(*end)?))
        }

        Expr::Call(func_id, args) => {
            let resolved_id = resolve_function_id(&func_id);
            let mut new_args = Vec::with_capacity(args.len());
            for arg in args {
                new_args.push(optimize(arg)?);
            }
            Expr::Call(resolved_id, new_args)
        }

        Expr::List(items) => {
            let mut new_items = Vec::with_capacity(items.len());
            for item in items {
                new_items.push(optimize(item)?);
            }
            Expr::List(new_items)
        }

        e => e,
    };

    if is_constant(&expr) {
        if let Ok(val) = evaluate(&expr, &[], &[], 0) {
            expr = match val {
                Value::Integer(i) => Expr::Integer(i),
                Value::Real(f) => Expr::Float(f),
                Value::Text(s) => Expr::String(s),
                Value::Null => Expr::Null,
                _ => expr,
            };
        }
    }

    Ok(expr)
}

/// Resolves column names to indices relative to the provided schema.
pub fn bind(expr: Expr, columns: &[String]) -> Result<Expr, String> {
    match expr {
        Expr::Column(name) => {
            let idx = columns
                .iter()
                .position(|c| c.eq_ignore_ascii_case(&name))
                .ok_or_else(|| format!("Column not found: '{name}'"))?;
            Ok(Expr::ColumnIndex(idx))
        }

        Expr::Binary(lhs, op, rhs) => Ok(Expr::Binary(
            Box::new(bind(*lhs, columns)?),
            op,
            Box::new(bind(*rhs, columns)?),
        )),
        Expr::Unary(op, inner) => Ok(Expr::Unary(op, Box::new(bind(*inner, columns)?))),
        Expr::Range(start, end) => Ok(Expr::Range(
            Box::new(bind(*start, columns)?),
            Box::new(bind(*end, columns)?),
        )),
        Expr::RegexMatch(lhs, pat) => Ok(Expr::RegexMatch(Box::new(bind(*lhs, columns)?), pat)),
        Expr::SetMatch(lhs, set) => Ok(Expr::SetMatch(Box::new(bind(*lhs, columns)?), set)),

        Expr::Call(func_id, args) => {
            let resolved_id = resolve_function_id(&func_id);
            let mut new_args = Vec::with_capacity(args.len());
            for arg in args {
                new_args.push(bind(arg, columns)?);
            }
            Ok(Expr::Call(resolved_id, new_args))
        }

        Expr::List(items) => {
            let mut new_items = Vec::with_capacity(items.len());
            for item in items {
                new_items.push(bind(item, columns)?);
            }
            Ok(Expr::List(new_items))
        }

        e => Ok(e),
    }
}

/// Evaluates an expression against a specific row.
///
/// * `expr`: The AST node to evaluate.
/// * `row`: The current row of data from the table.
/// * `columns`: The list of column names (used to resolve `Expr::Column`).
/// * `row_index`: The index of the current row (used for `@row`).
pub fn evaluate(expr: &Expr, row: &[Value], columns: &[String], row_index: usize) -> EvalResult {
    match expr {
        // --- Literals ---
        Expr::Null => Ok(Value::Null),
        Expr::Bool(b) => Ok(Value::Integer(if *b { 1 } else { 0 })),
        Expr::Integer(i) => Ok(Value::Integer(*i)),
        Expr::Float(f) => Ok(Value::Real(*f)),
        Expr::String(s) => Ok(Value::Text(s.clone())),

        // --- References ---
        Expr::List(_) => Err("Lists can only be used in 'in' checks".to_string()),
        Expr::Range(_, _) => {
            Err("Ranges can only be used inside lists for 'in' checks".to_string())
        }

        Expr::RowIndex => Ok(Value::Integer((row_index + 1) as i64)),

        Expr::ColumnIndex(idx) => Ok(row.get(*idx).cloned().unwrap_or(Value::Null)),

        Expr::Column(name) => {
            let idx = columns
                .iter()
                .position(|c| c.eq_ignore_ascii_case(name))
                .ok_or_else(|| format!("Column not found: '{name}'"))?;

            Ok(row.get(idx).cloned().unwrap_or(Value::Null))
        }

        // --- Unary Operations ---
        Expr::Unary(op, rhs) => {
            let val = evaluate(rhs, row, columns, row_index)?;

            if let Value::Null = val {
                return Ok(Value::Null);
            }

            match op {
                UnaryOp::Not => Ok(Value::Integer(if !is_truthy(&val) { 1 } else { 0 })),
                UnaryOp::Neg => match val {
                    Value::Integer(i) => Ok(Value::Integer(-i)),
                    Value::Real(f) => Ok(Value::Real(-f)),
                    _ => Err(format!("Cannot negate type {}", val_type(&val))),
                },
            }
        }

        Expr::RegexMatch(lhs, pat) => {
            let val = evaluate(lhs, row, columns, row_index)?;

            if let Value::Null = val {
                return Ok(Value::Null);
            }

            if let Value::Text(t) = val {
                Ok(Value::Integer(if pat.re.is_match(&t) { 1 } else { 0 }))
            } else {
                Err(format!("Regex requires (Text), found ({})", val_type(&val)))
            }
        }

        Expr::SetMatch(lhs, set) => {
            let val = evaluate(lhs, row, columns, row_index)?;
            if let Value::Null = val {
                return Ok(Value::Null);
            }
            let s = match val {
                Value::Text(t) => t,
                Value::Integer(i) => i.to_string(),
                Value::Real(f) => f.to_string(),
                Value::Blob(_) => "<Blob>".to_string(),
                Value::Null => "null".to_string(),
            };
            Ok(Value::Integer(if set.contains(&s) { 1 } else { 0 }))
        }

        // --- Binary Operations ---
        Expr::Binary(lhs, op, rhs) => {
            // Short-circuiting logic for AND / OR
            if matches!(op, BinaryOp::And | BinaryOp::Or) {
                let left_val = evaluate(lhs, row, columns, row_index)?;
                let left_truthy = is_truthy(&left_val);

                if *op == BinaryOp::And {
                    if !left_truthy {
                        return Ok(left_val);
                    }
                } else {
                    if left_truthy {
                        return Ok(left_val);
                    }
                }

                return evaluate(rhs, row, columns, row_index);
            }

            if matches!(op, BinaryOp::In | BinaryOp::NotIn) {
                let l_val = evaluate(lhs, row, columns, row_index)?;

                if let Value::Null = l_val {
                    return Ok(Value::Null);
                }

                let is_in = if let Expr::Range(start, end) = &**rhs {
                    let start_val = evaluate(start, row, columns, row_index)?;
                    let end_val = evaluate(end, row, columns, row_index)?;

                    let ge = compare_values(&l_val, &start_val, &BinaryOp::Gte)?;
                    let le = compare_values(&l_val, &end_val, &BinaryOp::Lte)?;

                    is_truthy(&ge) && is_truthy(&le)
                } else if let Expr::List(items) = &**rhs {
                    let mut found = false;
                    for item in items {
                        if let Expr::Range(start, end) = item {
                            let start_val = evaluate(start, row, columns, row_index)?;
                            let end_val = evaluate(end, row, columns, row_index)?;

                            let ge = compare_values(&l_val, &start_val, &BinaryOp::Gte)?;
                            let le = compare_values(&l_val, &end_val, &BinaryOp::Lte)?;

                            if is_truthy(&ge) && is_truthy(&le) {
                                found = true;
                                break;
                            }
                        } else {
                            let item_val = evaluate(item, row, columns, row_index)?;
                            if values_equal(&l_val, &item_val) {
                                found = true;
                                break;
                            }
                        }
                    }
                    found
                } else {
                    return Err("Right side of 'in' must be a list [...] or range".to_string());
                };

                let result = if is_in { 1 } else { 0 };

                return Ok(Value::Integer(if matches!(op, BinaryOp::NotIn) {
                    1 - result
                } else {
                    result
                }));
            }

            let l_val = evaluate(lhs, row, columns, row_index)?;
            let r_val = evaluate(rhs, row, columns, row_index)?;

            if matches!(l_val, Value::Null) || matches!(r_val, Value::Null) {
                return Ok(Value::Null);
            }

            match op {
                BinaryOp::Eq => Ok(Value::Integer(if values_equal(&l_val, &r_val) {
                    1
                } else {
                    0
                })),
                BinaryOp::NotEq => Ok(Value::Integer(if !values_equal(&l_val, &r_val) {
                    1
                } else {
                    0
                })),

                BinaryOp::Regex => match (&l_val, &r_val) {
                    (Value::Text(t), Value::Text(pat)) => {
                        let re =
                            regex::Regex::new(pat).map_err(|e| format!("Invalid Regex: {e}"))?;
                        Ok(Value::Integer(if re.is_match(t) { 1 } else { 0 }))
                    }
                    _ => Err(format!(
                        "Regex requires (Text ~= Text), found ({} ~= {})",
                        val_type(&l_val),
                        val_type(&r_val)
                    )),
                },

                BinaryOp::Gt | BinaryOp::Lt | BinaryOp::Gte | BinaryOp::Lte => {
                    compare_values(&l_val, &r_val, op)
                }

                _ => Err(format!("Unknown binary operator {:?}", op)),
            }
        }

        // --- Function Calls ---
        Expr::Call(func_id, args) => {
            let mut evaluated_args = Vec::with_capacity(args.len());
            for arg in args {
                evaluated_args.push(evaluate(arg, row, columns, row_index)?);
            }
            call_function(func_id, &evaluated_args)
        }
    }
}

// --- Helpers ---

fn val_type(v: &Value) -> &'static str {
    match v {
        Value::Integer(_) => "Integer",
        Value::Real(_) => "Float",
        Value::Text(_) => "Text",
        Value::Blob(_) => "Blob",
        Value::Null => "Null",
    }
}

pub fn is_truthy(v: &Value) -> bool {
    match v {
        Value::Integer(i) => *i != 0,
        Value::Real(f) => *f != 0.0,
        Value::Text(s) => !s.is_empty(),
        Value::Blob(b) => !b.is_empty(),
        Value::Null => false,
    }
}

fn values_equal(lhs: &Value, rhs: &Value) -> bool {
    match (lhs, rhs) {
        (Value::Integer(l), Value::Integer(r)) => l == r,
        (Value::Real(l), Value::Real(r)) => (l - r).abs() < f64::EPSILON,
        (Value::Text(l), Value::Text(r)) => l == r,
        (Value::Blob(l), Value::Blob(r)) => l == r,
        (Value::Null, Value::Null) => true,

        (Value::Integer(l), Value::Real(r)) => (*l as f64 - r).abs() < f64::EPSILON,
        (Value::Real(l), Value::Integer(r)) => (l - *r as f64).abs() < f64::EPSILON,

        _ => false,
    }
}

fn compare_values(lhs: &Value, rhs: &Value, op: &BinaryOp) -> EvalResult {
    let matches = match (lhs, rhs) {
        (Value::Integer(l), Value::Integer(r)) => apply_ord(l, r, op),
        (Value::Real(l), Value::Real(r)) => apply_ord(l, r, op),
        (Value::Text(l), Value::Text(r)) => apply_ord(l, r, op),

        (Value::Integer(l), Value::Real(r)) => apply_ord(&(*l as f64), r, op),
        (Value::Real(l), Value::Integer(r)) => apply_ord(l, &(*r as f64), op),

        _ => {
            return Err(format!(
                "Type mismatch: Cannot compare {} {} {}",
                val_type(lhs),
                match op {
                    BinaryOp::Gt => ">",
                    BinaryOp::Lt => "<",
                    _ => "?",
                },
                val_type(rhs)
            ));
        }
    };

    Ok(Value::Integer(if matches { 1 } else { 0 }))
}

fn apply_ord<T: PartialOrd>(l: &T, r: &T, op: &BinaryOp) -> bool {
    match op {
        BinaryOp::Gt => l > r,
        BinaryOp::Lt => l < r,
        BinaryOp::Gte => l >= r,
        BinaryOp::Lte => l <= r,
        _ => false,
    }
}

// --- Standard Library ---

fn call_function(func_id: &FunctionId, args: &[Value]) -> EvalResult {
    match func_id {
        // int(val)
        FunctionId::Int => {
            check_arg_count("int", args, 1)?;
            match &args[0] {
                Value::Integer(i) => Ok(Value::Integer(*i)),
                Value::Real(f) => Ok(Value::Integer(*f as i64)),
                Value::Text(s) => s
                    .parse::<i64>()
                    .map(Value::Integer)
                    .map_err(|_| format!("int(): Cannot parse '{s}'")),
                Value::Null => Ok(Value::Null),
                _ => Err(format!("int(): Cannot cast {}", val_type(&args[0]))),
            }
        }

        // float(val)
        FunctionId::Float => {
            check_arg_count("float", args, 1)?;
            match &args[0] {
                Value::Real(f) => Ok(Value::Real(*f)),
                Value::Integer(i) => Ok(Value::Real(*i as f64)),
                Value::Text(s) => s
                    .parse::<f64>()
                    .map(Value::Real)
                    .map_err(|_| format!("float(): Cannot parse '{s}'")),
                Value::Null => Ok(Value::Null),
                _ => Err(format!("float(): Cannot cast {}", val_type(&args[0]))),
            }
        }

        // str(val)
        FunctionId::String => {
            check_arg_count("str", args, 1)?;
            match &args[0] {
                Value::Text(s) => Ok(Value::Text(s.clone())),
                Value::Integer(i) => Ok(Value::Text(i.to_string())),
                Value::Real(f) => Ok(Value::Text(f.to_string())),
                Value::Blob(_) => Ok(Value::Text("<Blob>".to_string())),
                Value::Null => Ok(Value::Text("null".to_string())),
            }
        }

        // date(val) -> timestamp
        FunctionId::Date => {
            check_arg_count("date", args, 1)?;
            match &args[0] {
                Value::Text(s) => {
                    // Uses Chrono for parsing
                    let date = NaiveDate::parse_from_str(s, "%Y-%m-%d")
                        .map_err(|e| format!("date(): {e}"))?;

                    // Convert to timestamp (seconds)
                    let ts = date
                        .and_hms_opt(0, 0, 0)
                        .ok_or("date(): Invalid time")?
                        .and_utc()
                        .timestamp();

                    Ok(Value::Integer(ts))
                }
                Value::Null => Ok(Value::Null),
                v => Err(format!("date(): Expected String, found {}", val_type(v))),
            }
        }

        // len(val)
        FunctionId::Len => {
            check_arg_count("len", args, 1)?;
            match &args[0] {
                Value::Text(s) => Ok(Value::Integer(s.len() as i64)),
                Value::Blob(b) => Ok(Value::Integer(b.len() as i64)),
                Value::Null => Ok(Value::Null),
                v => Err(format!("len(): Expected Text/Blob, found {}", val_type(v))),
            }
        }

        // lower(val)
        FunctionId::Lower => {
            check_arg_count("lower", args, 1)?;
            match &args[0] {
                Value::Text(s) => Ok(Value::Text(s.to_lowercase())),
                Value::Null => Ok(Value::Null),
                v => Err(format!("lower(): Expected Text, found {}", val_type(v))),
            }
        }

        // upper(val)
        FunctionId::Upper => {
            check_arg_count("upper", args, 1)?;
            match &args[0] {
                Value::Text(s) => Ok(Value::Text(s.to_uppercase())),
                Value::Null => Ok(Value::Null),
                v => Err(format!("upper(): Expected Text, found {}", val_type(v))),
            }
        }

        // contains(haystack, needle)
        FunctionId::Contains => {
            check_arg_count("contains", args, 2)?;
            match (&args[0], &args[1]) {
                (Value::Text(h), Value::Text(n)) => {
                    Ok(Value::Integer(if h.contains(n) { 1 } else { 0 }))
                }
                (Value::Null, _) | (_, Value::Null) => Ok(Value::Null),
                (h, n) => Err(format!(
                    "contains(): Expected (Text, Text), found ({}, {})",
                    val_type(h),
                    val_type(n)
                )),
            }
        }

        // is_null(val)
        FunctionId::IsNull => {
            check_arg_count("is_null", args, 1)?;
            Ok(Value::Integer(if matches!(args[0], Value::Null) {
                1
            } else {
                0
            }))
        }

        FunctionId::Unknown(name) => Err(format!("Unknown function: '{name}'")),
    }
}

fn check_arg_count(name: &str, args: &[Value], expected: usize) -> Result<(), String> {
    if args.len() == expected {
        Ok(())
    } else {
        Err(format!(
            "{name}(): Expected {expected} argument(s), found {}",
            args.len()
        ))
    }
}
