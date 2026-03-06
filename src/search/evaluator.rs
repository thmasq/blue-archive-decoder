use crate::search::ast::{BinaryOp, CompiledPattern, Expr, FunctionId, Literal, UnaryOp};
use chrono::NaiveDate;
use sqlite_wasm_reader::Value;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;

pub type EvalResult<'a> = Result<Cow<'a, Value>, String>;

thread_local! {
    static REGEX_CACHE: RefCell<HashMap<String, regex::Regex>> = RefCell::new(HashMap::new());
}

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
        Expr::ConstInList(lhs, _) => is_constant(lhs),
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
                        let mut const_list = Vec::with_capacity(items.len());
                        let empty_row: &[Value] = &[];
                        let empty_cols: &[String] = &[];

                        for item in items {
                            if let Ok(val) = evaluate(item, empty_row, empty_cols, 0) {
                                let lit = match val.into_owned() {
                                    Value::Text(t) => Literal::String(t),
                                    Value::Integer(i) => Literal::Integer(i),
                                    Value::Real(f) => Literal::Float(f),
                                    Value::Blob(b) => Literal::Blob(b),
                                    Value::Null => Literal::Null,
                                };
                                if !const_list.contains(&lit) {
                                    const_list.push(lit);
                                }
                            }
                        }

                        let match_expr = Expr::ConstInList(Box::new(lhs_opt.clone()), const_list);

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

            let expected = match resolved_id {
                FunctionId::Contains => 2,
                FunctionId::Unknown(ref name) => return Err(format!("Unknown function: '{name}'")),
                _ => 1,
            };

            if args.len() != expected {
                return Err(format!(
                    "{:?} expected {} argument(s), found {}",
                    resolved_id,
                    expected,
                    args.len()
                ));
            }

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
        let empty_row: &[Value] = &[];
        let empty_cols: &[String] = &[];
        if let Ok(val) = evaluate(&expr, empty_row, empty_cols, 0) {
            expr = match val.into_owned() {
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
        Expr::ConstInList(lhs, set) => Ok(Expr::ConstInList(Box::new(bind(*lhs, columns)?), set)),

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

pub fn evaluate<'a>(
    expr: &'a Expr,
    row: &'a [Value],
    columns: &[String],
    row_index: usize,
) -> EvalResult<'a> {
    match expr {
        Expr::Null => Ok(Cow::Owned(Value::Null)),
        Expr::Bool(b) => Ok(Cow::Owned(Value::Integer(if *b { 1 } else { 0 }))),
        Expr::Integer(i) => Ok(Cow::Owned(Value::Integer(*i))),
        Expr::Float(f) => Ok(Cow::Owned(Value::Real(*f))),
        Expr::String(s) => Ok(Cow::Owned(Value::Text(s.clone()))),

        // --- References ---
        Expr::List(_) => Err("Lists can only be used in 'in' checks".to_string()),
        Expr::Range(_, _) => {
            Err("Ranges can only be used inside lists for 'in' checks".to_string())
        }

        Expr::RowIndex => Ok(Cow::Owned(Value::Integer((row_index + 1) as i64))),

        Expr::ColumnIndex(idx) => Ok(row
            .get(*idx)
            .map(Cow::Borrowed)
            .unwrap_or(Cow::Owned(Value::Null))),

        Expr::Column(name) => {
            let idx = columns
                .iter()
                .position(|c| c.eq_ignore_ascii_case(name))
                .ok_or_else(|| format!("Column not found: '{name}'"))?;
            Ok(row
                .get(idx)
                .map(Cow::Borrowed)
                .unwrap_or(Cow::Owned(Value::Null)))
        }

        // --- Unary Operations ---
        Expr::Unary(op, rhs) => {
            let val = evaluate(rhs, row, columns, row_index)?;

            if let Value::Null = val.as_ref() {
                return Ok(Cow::Owned(Value::Null));
            }

            match op {
                UnaryOp::Not => Ok(Cow::Owned(Value::Integer(if !is_truthy(val.as_ref()) {
                    1
                } else {
                    0
                }))),
                UnaryOp::Neg => match val.as_ref() {
                    Value::Integer(i) => Ok(Cow::Owned(Value::Integer(-i))),
                    Value::Real(f) => Ok(Cow::Owned(Value::Real(-f))),
                    _ => Err(format!("Cannot negate type {}", val_type(val.as_ref()))),
                },
            }
        }

        Expr::RegexMatch(lhs, pat) => {
            let val = evaluate(lhs, row, columns, row_index)?;
            if let Value::Null = val.as_ref() {
                return Ok(Cow::Owned(Value::Null));
            }

            if let Value::Text(t) = val.as_ref() {
                Ok(Cow::Owned(Value::Integer(if pat.re.is_match(t) {
                    1
                } else {
                    0
                })))
            } else {
                Err(format!(
                    "Regex requires (Text), found ({})",
                    val_type(val.as_ref())
                ))
            }
        }

        Expr::ConstInList(lhs, list) => {
            let val = evaluate(lhs, row, columns, row_index)?;
            if let Value::Null = val.as_ref() {
                return Ok(Cow::Owned(Value::Null));
            }

            let mut found = false;
            for item in list {
                let is_eq = match (item, val.as_ref()) {
                    (Literal::Integer(l), Value::Integer(r)) => *l == *r,
                    (Literal::Float(l), Value::Real(r)) => (*l - r).abs() < f64::EPSILON,
                    (Literal::String(l), Value::Text(r)) => l == r,
                    (Literal::Blob(l), Value::Blob(r)) => l == r,
                    (Literal::Null, Value::Null) => true,
                    (Literal::Integer(l), Value::Real(r)) => (*l as f64 - r).abs() < f64::EPSILON,
                    (Literal::Float(l), Value::Integer(r)) => (*l - *r as f64).abs() < f64::EPSILON,
                    _ => false,
                };
                if is_eq {
                    found = true;
                    break;
                }
            }
            Ok(Cow::Owned(Value::Integer(if found { 1 } else { 0 })))
        }

        // --- Binary Operations ---
        Expr::Binary(lhs, op, rhs) => {
            // Short-circuiting logic for AND / OR
            if matches!(op, BinaryOp::And | BinaryOp::Or) {
                let left_val = evaluate(lhs, row, columns, row_index)?;
                let left_truthy = is_truthy(left_val.as_ref());

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

                if let Value::Null = l_val.as_ref() {
                    return Ok(Cow::Owned(Value::Null));
                }

                let is_in = if let Expr::Range(start, end) = &**rhs {
                    let start_val = evaluate(start, row, columns, row_index)?;
                    let end_val = evaluate(end, row, columns, row_index)?;

                    let ge = compare_values(l_val.as_ref(), start_val.as_ref(), &BinaryOp::Gte)?;
                    let le = compare_values(l_val.as_ref(), end_val.as_ref(), &BinaryOp::Lte)?;

                    is_truthy(ge.as_ref()) && is_truthy(le.as_ref())
                } else if let Expr::List(items) = &**rhs {
                    let mut found = false;
                    for item in items {
                        if let Expr::Range(start, end) = item {
                            let start_val = evaluate(start, row, columns, row_index)?;
                            let end_val = evaluate(end, row, columns, row_index)?;

                            let ge =
                                compare_values(l_val.as_ref(), start_val.as_ref(), &BinaryOp::Gte)?;
                            let le =
                                compare_values(l_val.as_ref(), end_val.as_ref(), &BinaryOp::Lte)?;

                            if is_truthy(ge.as_ref()) && is_truthy(le.as_ref()) {
                                found = true;
                                break;
                            }
                        } else {
                            let item_val = evaluate(item, row, columns, row_index)?;
                            if values_equal(l_val.as_ref(), item_val.as_ref()) {
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

                return Ok(Cow::Owned(Value::Integer(
                    if matches!(op, BinaryOp::NotIn) {
                        1 - result
                    } else {
                        result
                    },
                )));
            }

            let l_val = evaluate(lhs, row, columns, row_index)?;
            let r_val = evaluate(rhs, row, columns, row_index)?;

            if matches!(l_val.as_ref(), Value::Null) || matches!(r_val.as_ref(), Value::Null) {
                return Ok(Cow::Owned(Value::Null));
            }

            match op {
                BinaryOp::Eq => Ok(Cow::Owned(Value::Integer(
                    if values_equal(l_val.as_ref(), r_val.as_ref()) {
                        1
                    } else {
                        0
                    },
                ))),
                BinaryOp::NotEq => Ok(Cow::Owned(Value::Integer(
                    if !values_equal(l_val.as_ref(), r_val.as_ref()) {
                        1
                    } else {
                        0
                    },
                ))),

                BinaryOp::Regex => match (l_val.as_ref(), r_val.as_ref()) {
                    (Value::Text(t), Value::Text(pat)) => {
                        let is_match = REGEX_CACHE.with(|cache| {
                            let mut cache = cache.borrow_mut();
                            if let Some(re) = cache.get(pat) {
                                return Ok::<bool, String>(re.is_match(t));
                            }
                            let re = regex::Regex::new(pat)
                                .map_err(|e| format!("Invalid Regex: {e}"))?;
                            let m = re.is_match(t);
                            cache.insert(pat.clone(), re);
                            Ok(m)
                        })?;
                        Ok(Cow::Owned(Value::Integer(if is_match { 1 } else { 0 })))
                    }
                    _ => Err(format!(
                        "Regex requires (Text ~= Text), found ({} ~= {})",
                        val_type(l_val.as_ref()),
                        val_type(r_val.as_ref())
                    )),
                },

                BinaryOp::Gt | BinaryOp::Lt | BinaryOp::Gte | BinaryOp::Lte => {
                    compare_values(l_val.as_ref(), r_val.as_ref(), op)
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

fn compare_values<'a>(lhs: &Value, rhs: &Value, op: &BinaryOp) -> EvalResult<'a> {
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

    Ok(Cow::Owned(Value::Integer(if matches { 1 } else { 0 })))
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

fn call_function<'a>(func_id: &FunctionId, args: &[Cow<'a, Value>]) -> EvalResult<'a> {
    match func_id {
        FunctionId::Int => match args[0].as_ref() {
            Value::Integer(i) => Ok(Cow::Owned(Value::Integer(*i))),
            Value::Real(f) => Ok(Cow::Owned(Value::Integer(*f as i64))),
            Value::Text(s) => s
                .parse::<i64>()
                .map(|i| Cow::Owned(Value::Integer(i)))
                .map_err(|_| format!("int(): Cannot parse '{s}'")),
            Value::Null => Ok(Cow::Owned(Value::Null)),
            v => Err(format!("int(): Cannot cast {}", val_type(v))),
        },

        FunctionId::Float => match args[0].as_ref() {
            Value::Real(f) => Ok(Cow::Owned(Value::Real(*f))),
            Value::Integer(i) => Ok(Cow::Owned(Value::Real(*i as f64))),
            Value::Text(s) => s
                .parse::<f64>()
                .map(|f| Cow::Owned(Value::Real(f)))
                .map_err(|_| format!("float(): Cannot parse '{s}'")),
            Value::Null => Ok(Cow::Owned(Value::Null)),
            v => Err(format!("float(): Cannot cast {}", val_type(v))),
        },

        FunctionId::String => match args[0].as_ref() {
            Value::Text(s) => Ok(Cow::Owned(Value::Text(s.clone()))),
            Value::Integer(i) => Ok(Cow::Owned(Value::Text(i.to_string()))),
            Value::Real(f) => Ok(Cow::Owned(Value::Text(f.to_string()))),
            Value::Blob(_) => Ok(Cow::Owned(Value::Text("<Blob>".to_string()))),
            Value::Null => Ok(Cow::Owned(Value::Text("null".to_string()))),
        },

        FunctionId::Date => match args[0].as_ref() {
            Value::Text(s) => {
                let date =
                    NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| format!("date(): {e}"))?;

                let ts = date
                    .and_hms_opt(0, 0, 0)
                    .ok_or("date(): Invalid time")?
                    .and_utc()
                    .timestamp();

                Ok(Cow::Owned(Value::Integer(ts)))
            }
            Value::Null => Ok(Cow::Owned(Value::Null)),
            v => Err(format!("date(): Expected String, found {}", val_type(v))),
        },

        FunctionId::Len => match args[0].as_ref() {
            Value::Text(s) => Ok(Cow::Owned(Value::Integer(s.len() as i64))),
            Value::Blob(b) => Ok(Cow::Owned(Value::Integer(b.len() as i64))),
            Value::Null => Ok(Cow::Owned(Value::Null)),
            v => Err(format!("len(): Expected Text/Blob, found {}", val_type(v))),
        },

        FunctionId::Lower => match args[0].as_ref() {
            Value::Text(s) => Ok(Cow::Owned(Value::Text(s.to_lowercase()))),
            Value::Null => Ok(Cow::Owned(Value::Null)),
            v => Err(format!("lower(): Expected Text, found {}", val_type(v))),
        },

        FunctionId::Upper => match args[0].as_ref() {
            Value::Text(s) => Ok(Cow::Owned(Value::Text(s.to_uppercase()))),
            Value::Null => Ok(Cow::Owned(Value::Null)),
            v => Err(format!("upper(): Expected Text, found {}", val_type(v))),
        },

        FunctionId::Contains => match (args[0].as_ref(), args[1].as_ref()) {
            (Value::Text(h), Value::Text(n)) => Ok(Cow::Owned(Value::Integer(if h.contains(n) {
                1
            } else {
                0
            }))),
            (Value::Null, _) | (_, Value::Null) => Ok(Cow::Owned(Value::Null)),
            (h, n) => Err(format!(
                "contains(): Expected (Text, Text), found ({}, {})",
                val_type(h),
                val_type(n)
            )),
        },

        FunctionId::IsNull => Ok(Cow::Owned(Value::Integer(
            if matches!(args[0].as_ref(), Value::Null) {
                1
            } else {
                0
            },
        ))),

        FunctionId::Unknown(name) => Err(format!("Unknown function: '{name}'")),
    }
}
