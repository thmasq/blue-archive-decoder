use crate::search::ast::{BinaryOp, Expr, FunctionId, UnaryOp};
use crate::search::tokenizer::Token;
use chumsky::input::{Input, ValueInput};
use chumsky::prelude::*;

pub type Span = SimpleSpan;

pub fn parser<'a, I>() -> impl Parser<'a, I, Expr, extra::Err<Rich<'a, Token>>>
where
    I: ValueInput<'a, Token = Token, Span = Span>,
{
    recursive(|expr| {
        // --- Atoms ---
        let val = select! {
            Token::Null => Expr::Null,
            Token::True => Expr::Bool(true),
            Token::False => Expr::Bool(false),
            Token::Integer(i) => Expr::Integer(i),
            Token::Float(f) => Expr::Float(f),
            Token::StringLiteral(s) => Expr::String(s),
            Token::RowIndex => Expr::RowIndex,
        }
        .labelled("value");

        // --- Lists ---
        let list = expr
            .clone()
            .separated_by(just(Token::Comma))
            .collect::<Vec<_>>()
            .delimited_by(just(Token::LBracket), just(Token::RBracket))
            .map(Expr::List)
            .labelled("list");

        // --- Function Arguments ---
        let args = expr
            .clone()
            .separated_by(just(Token::Comma))
            .collect::<Vec<_>>()
            .delimited_by(just(Token::LParen), just(Token::RParen))
            .labelled("arguments");

        // --- Function Calls vs Columns ---
        let call_or_ref = select! { Token::Identifier(i) => i }
            .then(args.clone().or_not())
            .map(|(name, args)| match args {
                Some(fn_args) => Expr::Call(FunctionId::Unknown(name), fn_args),
                None => Expr::Column(name),
            });

        // Base Atom
        let atom_base = choice((
            val,
            call_or_ref,
            list,
            expr.clone()
                .delimited_by(just(Token::LParen), just(Token::RParen)),
        ));

        // --- Method Chaining ---
        let atom = atom_base.foldl_with(
            just(Token::Dot)
                .ignore_then(select! { Token::Identifier(i) => i })
                .then(args.clone())
                .repeated(),
            |lhs, (method_name, args), _span| {
                let mut call_args = vec![lhs];
                call_args.extend(args);
                Expr::Call(FunctionId::Unknown(method_name), call_args)
            },
        );

        // --- Unary ---
        let unary = choice((
            just(Token::Not).to(UnaryOp::Not),
            just(Token::Minus).to(UnaryOp::Neg),
        ))
        .repeated()
        .collect::<Vec<_>>()
        .then(atom)
        .map(|(ops, rhs)| {
            ops.into_iter()
                .rfold(rhs, |acc, op| Expr::Unary(op, Box::new(acc)))
        });

        let range = unary
            .clone()
            .then(just(Token::RangeOp).ignore_then(unary.clone()).or_not())
            .map(|(start, end_opt)| match end_opt {
                Some(end) => Expr::Range(Box::new(start), Box::new(end)),
                None => start,
            });

        // --- Comparisons & Chaining ---
        let op = choice((
            just(Token::Eq).to(BinaryOp::Eq),
            just(Token::NotEq).to(BinaryOp::NotEq),
            just(Token::Lt).to(BinaryOp::Lt),
            just(Token::Lte).to(BinaryOp::Lte),
            just(Token::Gt).to(BinaryOp::Gt),
            just(Token::Gte).to(BinaryOp::Gte),
            just(Token::Regex).to(BinaryOp::Regex),
            just(Token::In).to(BinaryOp::In),
            just(Token::Not)
                .ignore_then(just(Token::In))
                .to(BinaryOp::NotIn),
        ));

        let comparison = range
            .clone()
            .then(op.then(range).repeated().collect::<Vec<_>>())
            .map(|(lhs, ops)| {
                if ops.is_empty() {
                    lhs
                } else if ops.len() == 1 {
                    let (op, rhs) = ops.into_iter().next().unwrap();
                    Expr::Binary(Box::new(lhs), op, Box::new(rhs))
                } else {
                    let mut exprs = Vec::new();
                    let mut current_lhs = lhs;
                    for (op, rhs) in ops {
                        exprs.push(Expr::Binary(
                            Box::new(current_lhs.clone()),
                            op,
                            Box::new(rhs.clone()),
                        ));
                        current_lhs = rhs;
                    }
                    exprs
                        .into_iter()
                        .reduce(|l, r| Expr::Binary(Box::new(l), BinaryOp::And, Box::new(r)))
                        .unwrap()
                }
            });

        // --- Logic ---
        let logic_and = comparison.clone().foldl_with(
            just(Token::And).ignore_then(comparison).repeated(),
            |lhs, rhs, _span| Expr::Binary(Box::new(lhs), BinaryOp::And, Box::new(rhs)),
        );

        let logic_or = logic_and.clone().foldl_with(
            just(Token::Or).ignore_then(logic_and).repeated(),
            |lhs, rhs, _span| Expr::Binary(Box::new(lhs), BinaryOp::Or, Box::new(rhs)),
        );

        logic_or
    })
}

pub fn parse(input: &str) -> Result<Expr, Vec<String>> {
    use logos::Logos;

    let lex = Token::lexer(input)
        .spanned()
        .map(|(tok, span)| (tok, SimpleSpan::from(span)));

    let (tokens, errors): (Vec<_>, Vec<_>) = lex
        .map(|(tok, span)| match tok {
            Ok(t) => Ok((t, span)),
            Err(_) => Err(span),
        })
        .partition(Result::is_ok);

    if !errors.is_empty() {
        return Err(vec!["Tokenization error".to_string()]);
    }

    let tokens: Vec<(Token, Span)> = tokens.into_iter().map(Result::unwrap).collect();
    let len = input.len();
    let eoi = SimpleSpan::from(len..len);

    let token_slice = tokens.as_slice();
    let input = token_slice.map(eoi, |(t, s)| (t, s));

    match parser().parse(input).into_result() {
        Ok(ast) => Ok(ast),
        Err(errors) => Err(errors.into_iter().map(|e| e.to_string()).collect()),
    }
}
