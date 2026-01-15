use regex::Regex;
use std::collections::HashSet;
use std::fmt;

#[derive(Clone)]
pub struct CompiledPattern {
    pub re: Regex,
    pub original: String,
}

impl PartialEq for CompiledPattern {
    fn eq(&self, other: &Self) -> bool {
        self.original == other.original
    }
}

impl fmt::Debug for CompiledPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Regex(\"{}\")", self.original)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    // Logic
    And,
    Or,
    // Comparison
    Eq,
    NotEq,
    Gt,
    Lt,
    Gte,
    Lte,
    Regex, // ~=
    In,    // in
    NotIn, // not in
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Not, // !
    Neg, // - (for negative numbers)
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionId {
    Int,
    Float,
    String,
    Date,
    Len,
    Lower,
    Upper,
    Contains,
    IsNull,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // Values
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),

    // References
    RowIndex, // @row
    Column(String),
    ColumnIndex(usize),

    // Structures
    List(Vec<Expr>), // ["A", "B"]

    // Operations
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Range(Box<Expr>, Box<Expr>), // start..end

    // Optimizations
    RegexMatch(Box<Expr>, CompiledPattern),
    SetMatch(Box<Expr>, HashSet<String>),

    // Functions
    Call(FunctionId, Vec<Expr>),
}
