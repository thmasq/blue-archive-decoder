use crate::search::ast::{BinaryOp, CompiledPattern, Expr, FunctionId, Literal, UnaryOp};
use bitvec::prelude::*;
use chrono::NaiveDate;
use sqlite_wasm_reader::Value;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;

pub enum ColumnarResult<'a> {
    Scalar(Cow<'a, Value>),
    Column(&'a [Value]),
    Vector(Vec<Value>),
    Bitmap(BitVec),
}

impl<'a> ColumnarResult<'a> {
    pub fn get(&self, idx: usize) -> Cow<'_, Value> {
        match self {
            ColumnarResult::Scalar(v) => Cow::Borrowed(v.as_ref()),
            ColumnarResult::Column(c) => Cow::Borrowed(&c[idx]),
            ColumnarResult::Vector(v) => Cow::Borrowed(&v[idx]),
            ColumnarResult::Bitmap(b) => Cow::Owned(Value::Integer(if b[idx] { 1 } else { 0 })),
        }
    }

    pub fn is_scalar(&self) -> bool {
        matches!(self, ColumnarResult::Scalar(_))
    }

    /// Converts any result into a dense Bitmap for ultra-fast bitwise logical operations.
    pub fn into_bitmap(self, row_count: usize) -> BitVec {
        match self {
            ColumnarResult::Bitmap(b) => b,
            ColumnarResult::Scalar(v) => {
                let bit = is_truthy(v.as_ref());
                let mut b = BitVec::with_capacity(row_count);
                b.resize(row_count, bit);
                b
            }
            ColumnarResult::Column(c) => {
                let mut b = BitVec::with_capacity(row_count);
                for v in c.iter() {
                    b.push(is_truthy(v));
                }
                b
            }
            ColumnarResult::Vector(v) => {
                let mut b = BitVec::with_capacity(row_count);
                for val in v.iter() {
                    b.push(is_truthy(val));
                }
                b
            }
        }
    }
}

pub type EvalResult<'a> = Result<ColumnarResult<'a>, String>;

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
                        let empty_cols_data: &[Vec<Value>] = &[];
                        let empty_cols: &[String] = &[];

                        for item in items {
                            if let Ok(val) = evaluate(item, empty_cols_data, empty_cols, 1) {
                                let lit = match val.get(0).into_owned() {
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
                    "{:?} expected {} arg(s), found {}",
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
        let empty_cols_data: &[Vec<Value>] = &[];
        let empty_cols: &[String] = &[];
        if let Ok(val) = evaluate(&expr, empty_cols_data, empty_cols, 1) {
            expr = match val.get(0).into_owned() {
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
    columns_data: &'a [Vec<Value>],
    columns: &[String],
    row_count: usize,
) -> EvalResult<'a> {
    match expr {
        Expr::Null => Ok(ColumnarResult::Scalar(Cow::Owned(Value::Null))),
        Expr::Bool(b) => Ok(ColumnarResult::Scalar(Cow::Owned(Value::Integer(if *b {
            1
        } else {
            0
        })))),
        Expr::Integer(i) => Ok(ColumnarResult::Scalar(Cow::Owned(Value::Integer(*i)))),
        Expr::Float(f) => Ok(ColumnarResult::Scalar(Cow::Owned(Value::Real(*f)))),
        Expr::String(s) => Ok(ColumnarResult::Scalar(Cow::Owned(Value::Text(s.clone())))),

        // --- References ---
        Expr::List(_) => Err("Lists can only be used in 'in' checks".to_string()),
        Expr::Range(_, _) => {
            Err("Ranges can only be used inside lists for 'in' checks".to_string())
        }

        Expr::RowIndex => {
            let mut vec = Vec::with_capacity(row_count);
            for i in 0..row_count {
                vec.push(Value::Integer((i + 1) as i64));
            }
            Ok(ColumnarResult::Vector(vec))
        }

        Expr::ColumnIndex(idx) => Ok(columns_data
            .get(*idx)
            .map(|c| ColumnarResult::Column(c.as_slice()))
            .unwrap_or_else(|| ColumnarResult::Scalar(Cow::Owned(Value::Null)))),

        Expr::Column(name) => {
            let idx = columns
                .iter()
                .position(|c| c.eq_ignore_ascii_case(name))
                .ok_or_else(|| format!("Column not found: '{name}'"))?;
            Ok(columns_data
                .get(idx)
                .map(|c| ColumnarResult::Column(c.as_slice()))
                .unwrap_or_else(|| ColumnarResult::Scalar(Cow::Owned(Value::Null))))
        }

        // --- Unary Operations ---
        Expr::Unary(op, rhs) => {
            let val = evaluate(rhs, columns_data, columns, row_count)?;
            match op {
                // Not flips the bit vector completely with highly optimized NOT instructions
                UnaryOp::Not => Ok(ColumnarResult::Bitmap(!val.into_bitmap(row_count))),
                UnaryOp::Neg => apply_unary(val, row_count, |v| {
                    if let Value::Null = v {
                        return Ok(Value::Null);
                    }
                    match v {
                        Value::Integer(i) => Ok(Value::Integer(-i)),
                        Value::Real(f) => Ok(Value::Real(-f)),
                        _ => Err(format!("Cannot negate type {}", val_type(v))),
                    }
                }),
            }
        }

        Expr::RegexMatch(lhs, pat) => {
            let val = evaluate(lhs, columns_data, columns, row_count)?;
            let mut bmp = BitVec::with_capacity(row_count);
            for i in 0..row_count {
                let v = val.get(i);
                if let Value::Null = v.as_ref() {
                    bmp.push(false);
                } else if let Value::Text(t) = v.as_ref() {
                    bmp.push(pat.re.is_match(t));
                } else {
                    return Err(format!(
                        "Regex requires (Text), found ({})",
                        val_type(v.as_ref())
                    ));
                }
            }
            Ok(ColumnarResult::Bitmap(bmp))
        }

        Expr::ConstInList(lhs, list) => {
            let val = evaluate(lhs, columns_data, columns, row_count)?;
            let mut bmp = BitVec::with_capacity(row_count);
            for i in 0..row_count {
                let v = val.get(i);
                if let Value::Null = v.as_ref() {
                    bmp.push(false);
                    continue;
                }
                let mut found = false;
                for item in list {
                    let is_eq = match (item, v.as_ref()) {
                        (Literal::Integer(l), Value::Integer(r)) => *l == *r,
                        (Literal::Float(l), Value::Real(r)) => (*l - r).abs() < f64::EPSILON,
                        (Literal::String(l), Value::Text(r)) => l == r,
                        (Literal::Blob(l), Value::Blob(r)) => l == r,
                        (Literal::Null, Value::Null) => true,
                        (Literal::Integer(l), Value::Real(r)) => {
                            (*l as f64 - r).abs() < f64::EPSILON
                        }
                        (Literal::Float(l), Value::Integer(r)) => {
                            (*l - *r as f64).abs() < f64::EPSILON
                        }
                        _ => false,
                    };
                    if is_eq {
                        found = true;
                        break;
                    }
                }
                bmp.push(found);
            }
            Ok(ColumnarResult::Bitmap(bmp))
        }

        // --- Binary Operations ---
        Expr::Binary(lhs, op, rhs) => {
            // Short-circuiting logic for AND / OR
            if matches!(op, BinaryOp::And | BinaryOp::Or) {
                let left_val = evaluate(lhs, columns_data, columns, row_count)?;
                let left_bmp = left_val.into_bitmap(row_count);

                // Bitmap-level short-circuiting!
                if *op == BinaryOp::And && left_bmp.not_any() {
                    return Ok(ColumnarResult::Bitmap(left_bmp));
                } else if *op == BinaryOp::Or && left_bmp.all() {
                    return Ok(ColumnarResult::Bitmap(left_bmp));
                }

                let right_val = evaluate(rhs, columns_data, columns, row_count)?;
                let right_bmp = right_val.into_bitmap(row_count);

                // This compiles down to pure SIMD logic instructions on the vectors
                if *op == BinaryOp::And {
                    return Ok(ColumnarResult::Bitmap(left_bmp & right_bmp));
                } else {
                    return Ok(ColumnarResult::Bitmap(left_bmp | right_bmp));
                }
            }

            if matches!(op, BinaryOp::In | BinaryOp::NotIn) {
                let l_val = evaluate(lhs, columns_data, columns, row_count)?;
                let mut is_in_result = BitVec::with_capacity(row_count);

                if let Expr::Range(start, end) = &**rhs {
                    let start_val = evaluate(start, columns_data, columns, row_count)?;
                    let end_val = evaluate(end, columns_data, columns, row_count)?;

                    for i in 0..row_count {
                        let l = l_val.get(i);
                        if let Value::Null = l.as_ref() {
                            is_in_result.push(false);
                            continue;
                        }

                        let s = start_val.get(i);
                        let e = end_val.get(i);
                        let ge = compare_values_bool(l.as_ref(), s.as_ref(), &BinaryOp::Gte)
                            .unwrap_or(false);
                        let le = compare_values_bool(l.as_ref(), e.as_ref(), &BinaryOp::Lte)
                            .unwrap_or(false);
                        is_in_result.push(ge && le);
                    }
                } else if let Expr::List(items) = &**rhs {
                    enum EvalListItem<'a> {
                        Single(ColumnarResult<'a>),
                        Range(ColumnarResult<'a>, ColumnarResult<'a>),
                    }
                    let mut eval_items = Vec::with_capacity(items.len());
                    for item in items {
                        if let Expr::Range(start, end) = item {
                            eval_items.push(EvalListItem::Range(
                                evaluate(start, columns_data, columns, row_count)?,
                                evaluate(end, columns_data, columns, row_count)?,
                            ));
                        } else {
                            eval_items.push(EvalListItem::Single(evaluate(
                                item,
                                columns_data,
                                columns,
                                row_count,
                            )?));
                        }
                    }

                    for i in 0..row_count {
                        let l = l_val.get(i);
                        if let Value::Null = l.as_ref() {
                            is_in_result.push(false);
                            continue;
                        }

                        let mut found = false;
                        for e_item in &eval_items {
                            match e_item {
                                EvalListItem::Range(s_val, e_val) => {
                                    let s = s_val.get(i);
                                    let e = e_val.get(i);
                                    let ge =
                                        compare_values_bool(l.as_ref(), s.as_ref(), &BinaryOp::Gte)
                                            .unwrap_or(false);
                                    let le =
                                        compare_values_bool(l.as_ref(), e.as_ref(), &BinaryOp::Lte)
                                            .unwrap_or(false);
                                    if ge && le {
                                        found = true;
                                        break;
                                    }
                                }
                                EvalListItem::Single(s_val) => {
                                    let s = s_val.get(i);
                                    if values_equal(l.as_ref(), s.as_ref()) {
                                        found = true;
                                        break;
                                    }
                                }
                            }
                        }
                        is_in_result.push(found);
                    }
                } else {
                    return Err("Right side of 'in' must be a list [...] or range".to_string());
                }

                if *op == BinaryOp::NotIn {
                    is_in_result = !is_in_result;
                }
                return Ok(ColumnarResult::Bitmap(is_in_result));
            }

            let l_val = evaluate(lhs, columns_data, columns, row_count)?;
            let r_val = evaluate(rhs, columns_data, columns, row_count)?;

            let mut bmp = BitVec::with_capacity(row_count);
            for i in 0..row_count {
                let l = l_val.get(i);
                let r = r_val.get(i);

                if matches!(l.as_ref(), Value::Null) || matches!(r.as_ref(), Value::Null) {
                    bmp.push(false);
                    continue;
                }

                let b = match op {
                    BinaryOp::Eq => Ok(values_equal(l.as_ref(), r.as_ref())),
                    BinaryOp::NotEq => Ok(!values_equal(l.as_ref(), r.as_ref())),
                    BinaryOp::Regex => {
                        if let (Value::Text(t), Value::Text(pat)) = (l.as_ref(), r.as_ref()) {
                            REGEX_CACHE.with(|cache| {
                                let mut cache = cache.borrow_mut();
                                if let Some(re) = cache.get(pat) {
                                    return Ok::<bool, String>(re.is_match(t));
                                }
                                let re = regex::Regex::new(pat)
                                    .map_err(|e| format!("Invalid Regex: {e}"))?;
                                let m = re.is_match(t);
                                cache.insert(pat.clone(), re);
                                Ok(m)
                            })
                        } else {
                            Err(format!(
                                "Regex requires (Text ~= Text), found ({} ~= {})",
                                val_type(l.as_ref()),
                                val_type(r.as_ref())
                            ))
                        }
                    }
                    BinaryOp::Gt | BinaryOp::Lt | BinaryOp::Gte | BinaryOp::Lte => {
                        compare_values_bool(l.as_ref(), r.as_ref(), op)
                    }
                    _ => Err(format!("Unknown binary operator {:?}", op)),
                }?;
                bmp.push(b);
            }
            Ok(ColumnarResult::Bitmap(bmp))
        }

        // --- Function Calls ---
        Expr::Call(func_id, args) => {
            let mut evaluated_args = Vec::with_capacity(args.len());
            for arg in args {
                evaluated_args.push(evaluate(arg, columns_data, columns, row_count)?);
            }

            let is_all_scalar = evaluated_args.iter().all(|a| a.is_scalar());

            if is_all_scalar {
                let mut scalar_args = Vec::with_capacity(evaluated_args.len());
                for a in &evaluated_args {
                    scalar_args.push(a.get(0));
                }
                let res = call_function(func_id, &scalar_args)?;
                Ok(ColumnarResult::Scalar(Cow::Owned(res)))
            } else {
                let mut vec = Vec::with_capacity(row_count);
                for i in 0..row_count {
                    let mut row_args = Vec::with_capacity(evaluated_args.len());
                    for a in &evaluated_args {
                        row_args.push(a.get(i));
                    }
                    vec.push(call_function(func_id, &row_args)?);
                }
                Ok(ColumnarResult::Vector(vec))
            }
        }
    }
}

// --- Application Loop Helpers ---

fn apply_unary<'a, F>(
    val: ColumnarResult<'a>,
    row_count: usize,
    op: F,
) -> Result<ColumnarResult<'a>, String>
where
    F: Fn(&Value) -> Result<Value, String>,
{
    match val {
        ColumnarResult::Scalar(v) => {
            let res = op(v.as_ref())?;
            Ok(ColumnarResult::Scalar(Cow::Owned(res)))
        }
        _ => {
            let mut vec = Vec::with_capacity(row_count);
            for i in 0..row_count {
                vec.push(op(val.get(i).as_ref())?);
            }
            Ok(ColumnarResult::Vector(vec))
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

fn compare_values_bool(lhs: &Value, rhs: &Value, op: &BinaryOp) -> Result<bool, String> {
    match (lhs, rhs) {
        (Value::Integer(l), Value::Integer(r)) => Ok(apply_ord(l, r, op)),
        (Value::Real(l), Value::Real(r)) => Ok(apply_ord(l, r, op)),
        (Value::Text(l), Value::Text(r)) => Ok(apply_ord(l, r, op)),

        (Value::Integer(l), Value::Real(r)) => Ok(apply_ord(&(*l as f64), r, op)),
        (Value::Real(l), Value::Integer(r)) => Ok(apply_ord(l, &(*r as f64), op)),

        _ => Err(format!(
            "Type mismatch: Cannot compare {} {} {}",
            val_type(lhs),
            match op {
                BinaryOp::Gt => ">",
                BinaryOp::Lt => "<",
                _ => "?",
            },
            val_type(rhs)
        )),
    }
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

fn call_function(func_id: &FunctionId, args: &[Cow<'_, Value>]) -> Result<Value, String> {
    match func_id {
        FunctionId::Int => match args[0].as_ref() {
            Value::Integer(i) => Ok(Value::Integer(*i)),
            Value::Real(f) => Ok(Value::Integer(*f as i64)),
            Value::Text(s) => s
                .parse::<i64>()
                .map(Value::Integer)
                .map_err(|_| format!("int(): Cannot parse '{s}'")),
            Value::Null => Ok(Value::Null),
            v => Err(format!("int(): Cannot cast {}", val_type(v))),
        },

        FunctionId::Float => match args[0].as_ref() {
            Value::Real(f) => Ok(Value::Real(*f)),
            Value::Integer(i) => Ok(Value::Real(*i as f64)),
            Value::Text(s) => s
                .parse::<f64>()
                .map(Value::Real)
                .map_err(|_| format!("float(): Cannot parse '{s}'")),
            Value::Null => Ok(Value::Null),
            v => Err(format!("float(): Cannot cast {}", val_type(v))),
        },

        FunctionId::String => match args[0].as_ref() {
            Value::Text(s) => Ok(Value::Text(s.clone())),
            Value::Integer(i) => Ok(Value::Text(i.to_string())),
            Value::Real(f) => Ok(Value::Text(f.to_string())),
            Value::Blob(_) => Ok(Value::Text("<Blob>".to_string())),
            Value::Null => Ok(Value::Text("null".to_string())),
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

                Ok(Value::Integer(ts))
            }
            Value::Null => Ok(Value::Null),
            v => Err(format!("date(): Expected String, found {}", val_type(v))),
        },

        FunctionId::Len => match args[0].as_ref() {
            Value::Text(s) => Ok(Value::Integer(s.len() as i64)),
            Value::Blob(b) => Ok(Value::Integer(b.len() as i64)),
            Value::Null => Ok(Value::Null),
            v => Err(format!("len(): Expected Text/Blob, found {}", val_type(v))),
        },

        FunctionId::Lower => match args[0].as_ref() {
            Value::Text(s) => Ok(Value::Text(s.to_lowercase())),
            Value::Null => Ok(Value::Null),
            v => Err(format!("lower(): Expected Text, found {}", val_type(v))),
        },

        FunctionId::Upper => match args[0].as_ref() {
            Value::Text(s) => Ok(Value::Text(s.to_uppercase())),
            Value::Null => Ok(Value::Null),
            v => Err(format!("upper(): Expected Text, found {}", val_type(v))),
        },

        FunctionId::Contains => match (args[0].as_ref(), args[1].as_ref()) {
            (Value::Text(h), Value::Text(n)) => {
                Ok(Value::Integer(if h.contains(n) { 1 } else { 0 }))
            }
            (Value::Null, _) | (_, Value::Null) => Ok(Value::Null),
            (h, n) => Err(format!(
                "contains(): Expected (Text, Text), found ({}, {})",
                val_type(h),
                val_type(n)
            )),
        },

        FunctionId::IsNull => Ok(Value::Integer(if matches!(args[0].as_ref(), Value::Null) {
            1
        } else {
            0
        })),

        FunctionId::Unknown(name) => Err(format!("Unknown function: '{name}'")),
    }
}
