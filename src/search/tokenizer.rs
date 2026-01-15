use logos::{Lexer, Logos};
use std::fmt;

// =======================
// Lexer error infrastructure
// =======================

#[derive(Debug, Clone)]
pub struct LexerError {
    pub message: String,
    pub span: std::ops::Range<usize>,
}

#[derive(Default, Debug, Clone)]
pub struct LexerExtras {
    pub error: Option<LexerError>,
}

// =======================
// Tokens
// =======================

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(extras = LexerExtras)]
pub enum Token {
    // --- Logic & Keywords ---
    #[regex("(?i)and")]
    #[token("&&")]
    #[token("&")]
    And,

    #[regex("(?i)or")]
    #[token("||")]
    #[token("|")]
    Or,

    #[regex("(?i)not")]
    #[token("!")]
    Not,

    #[regex("(?i)null")]
    Null,

    #[regex("(?i)true")]
    True,

    #[regex("(?i)false")]
    False,

    #[regex("(?i)in")]
    In,

    #[regex("-")]
    Minus,

    // --- Comparators ---
    #[token("==")]
    #[token("=")]
    Eq,

    #[token("!=")]
    #[token("<>")]
    NotEq,

    #[token(">")]
    Gt,

    #[token("<")]
    Lt,

    #[token(">=")]
    Gte,

    #[token("<=")]
    Lte,

    #[token("~=")]
    Regex,

    // --- Delimiters ---
    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token(",")]
    Comma,

    #[token(".")]
    Dot,

    #[token("..")]
    RangeOp,

    // --- Special ---
    #[token("@row")]
    RowIndex,

    // --- Literals ---
    #[regex(r#""([^"\\]|\\.)*""#, parse_string)]
    StringLiteral(String),

    #[regex(r"-?[0-9]+", parse_integer)]
    Integer(i64),

    #[regex(r"-?[0-9]+\.[0-9]+", parse_float)]
    Float(f64),

    // --- Identifiers ---
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),
}

// =======================
// Callback helpers
// =======================

fn parse_integer(lex: &mut Lexer<Token>) -> Option<i64> {
    match lex.slice().parse::<i64>() {
        Ok(v) => Some(v),
        Err(e) => {
            lex.extras.error = Some(LexerError {
                message: format!("Invalid integer '{}': {}", lex.slice(), e),
                span: lex.span(),
            });
            None
        }
    }
}

fn parse_float(lex: &mut Lexer<Token>) -> Option<f64> {
    match lex.slice().parse::<f64>() {
        Ok(v) => Some(v),
        Err(e) => {
            lex.extras.error = Some(LexerError {
                message: format!("Invalid float '{}': {}", lex.slice(), e),
                span: lex.span(),
            });
            None
        }
    }
}

fn parse_string(lex: &mut Lexer<Token>) -> Option<String> {
    let s = lex.slice();
    let content = &s[1..s.len() - 1];

    match unescape(content) {
        Some(v) => Some(v),
        None => {
            lex.extras.error = Some(LexerError {
                message: format!("Invalid escape sequence in string '{}'", s),
                span: lex.span(),
            });
            None
        }
    }
}

// =======================
// String unescape
// =======================

fn unescape(s: &str) -> Option<String> {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('\\') => result.push('\\'),
                Some('"') => result.push('"'),
                Some(other) => {
                    result.push('\\');
                    result.push(other);
                }
                None => return None,
            }
        } else {
            result.push(c);
        }
    }

    Some(result)
}

// =======================
// Token display
// =======================

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// =======================
// Lexer entry point helper
// =======================

pub fn lex(input: &str) -> Result<Vec<Token>, LexerError> {
    let mut lexer = Token::lexer(input);
    let mut tokens = Vec::new();

    while let Some(result) = lexer.next() {
        if let Some(err) = lexer.extras.error.clone() {
            return Err(err);
        }

        match result {
            Ok(token) => tokens.push(token),

            Err(()) => {
                return Err(LexerError {
                    message: "Invalid input".to_string(),
                    span: lexer.span(),
                });
            }
        }
    }

    Ok(tokens)
}
