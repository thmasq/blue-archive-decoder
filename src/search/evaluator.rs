use crate::search::ast::{BinaryOp, CompiledPattern, Expr, UnaryOp};
use chrono::NaiveDate;
use sqlite_wasm_reader::Value;

// We use a custom Result type to propagate "Type Errors" up to the UI.
pub type EvalResult = Result<Value, String>;

pub fn optimize(expr: Expr) -> Result<Expr, String> {
    match expr {
        Expr::Binary(lhs, op, rhs) => {
            if op == BinaryOp::Regex {
                if let Expr::String(ref pat) = *rhs {
                    let re = regex::Regex::new(pat).map_err(|e| format!("Invalid Regex: {e}"))?;

                    let lhs_optimized = optimize(*lhs)?;

                    return Ok(Expr::RegexMatch(
                        Box::new(lhs_optimized),
                        CompiledPattern {
                            re,
                            original: pat.clone(),
                        },
                    ));
                }
            }

            Ok(Expr::Binary(
                Box::new(optimize(*lhs)?),
                op,
                Box::new(optimize(*rhs)?),
            ))
        }
        Expr::Unary(op, inner) => Ok(Expr::Unary(op, Box::new(optimize(*inner)?))),
        Expr::Call(name, args) => {
            let mut new_args = Vec::with_capacity(args.len());
            for arg in args {
                new_args.push(optimize(arg)?);
            }
            Ok(Expr::Call(name, new_args))
        }
        Expr::List(items) => {
            let mut new_items = Vec::with_capacity(items.len());
            for item in items {
                new_items.push(optimize(item)?);
            }
            Ok(Expr::List(new_items))
        }
        e => Ok(e),
    }
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
        Expr::RegexMatch(lhs, pat) => Ok(Expr::RegexMatch(Box::new(bind(*lhs, columns)?), pat)),

        Expr::Call(name, args) => {
            let mut new_args = Vec::with_capacity(args.len());
            for arg in args {
                new_args.push(bind(arg, columns)?);
            }
            Ok(Expr::Call(name, new_args))
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

        // --- Binary Operations ---
        Expr::Binary(lhs, op, rhs) => {
            // Short-circuiting logic
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

                BinaryOp::In => {
                    if let Expr::List(items) = &**rhs {
                        for item in items {
                            let item_val = evaluate(item, row, columns, row_index)?;
                            if values_equal(&l_val, &item_val) {
                                return Ok(Value::Integer(1));
                            }
                        }
                        Ok(Value::Integer(0))
                    } else {
                        Err("Right side of 'in' must be a list [...]".to_string())
                    }
                }

                BinaryOp::Gt | BinaryOp::Lt | BinaryOp::Gte | BinaryOp::Lte => {
                    compare_values(&l_val, &r_val, op)
                }

                _ => Err(format!("Unknown binary operator {:?}", op)),
            }
        }

        // --- Function Calls ---
        Expr::Call(name, args) => {
            let mut evaluated_args = Vec::with_capacity(args.len());
            for arg in args {
                evaluated_args.push(evaluate(arg, row, columns, row_index)?);
            }
            call_function(name, &evaluated_args)
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

fn call_function(name: &str, args: &[Value]) -> EvalResult {
    match name {
        // int(val)
        "int" => {
            check_arg_count(name, args, 1)?;
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
        "float" => {
            check_arg_count(name, args, 1)?;
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
        "str" | "string" => {
            check_arg_count(name, args, 1)?;
            match &args[0] {
                Value::Text(s) => Ok(Value::Text(s.clone())),
                Value::Integer(i) => Ok(Value::Text(i.to_string())),
                Value::Real(f) => Ok(Value::Text(f.to_string())),
                Value::Blob(_) => Ok(Value::Text("<Blob>".to_string())),
                Value::Null => Ok(Value::Text("null".to_string())),
            }
        }

        // date(val) -> timestamp
        "date" => {
            check_arg_count(name, args, 1)?;
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
        "len" => {
            check_arg_count(name, args, 1)?;
            match &args[0] {
                Value::Text(s) => Ok(Value::Integer(s.len() as i64)),
                Value::Blob(b) => Ok(Value::Integer(b.len() as i64)),
                Value::Null => Ok(Value::Null),
                v => Err(format!("len(): Expected Text/Blob, found {}", val_type(v))),
            }
        }

        // lower(val)
        "lower" => {
            check_arg_count(name, args, 1)?;
            match &args[0] {
                Value::Text(s) => Ok(Value::Text(s.to_lowercase())),
                Value::Null => Ok(Value::Null),
                v => Err(format!("lower(): Expected Text, found {}", val_type(v))),
            }
        }

        // upper(val)
        "upper" => {
            check_arg_count(name, args, 1)?;
            match &args[0] {
                Value::Text(s) => Ok(Value::Text(s.to_uppercase())),
                Value::Null => Ok(Value::Null),
                v => Err(format!("upper(): Expected Text, found {}", val_type(v))),
            }
        }

        // contains(haystack, needle)
        "contains" => {
            check_arg_count(name, args, 2)?;
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
        "is_null" => {
            check_arg_count(name, args, 1)?;
            Ok(Value::Integer(if matches!(args[0], Value::Null) {
                1
            } else {
                0
            }))
        }

        _ => Err(format!("Unknown function: '{name}'")),
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
