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
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Not, // !
    Neg, // - (for negative numbers)
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
    RowIndex,       // @row
    Column(String), // name, school_id

    // Structures
    List(Vec<Expr>), // ["A", "B"]

    // Operations
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),

    // Functions: func_name, args
    Call(String, Vec<Expr>),
}
