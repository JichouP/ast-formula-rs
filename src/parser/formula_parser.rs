use super::expr_parser::expr_parser;
use crate::domain::expr::Expr;
use nom::{
    error::{Error, ErrorKind},
    Err, IResult,
};

pub fn formula_parser(input: &str) -> IResult<&str, Expr> {
    let (unused, expr) = expr_parser(input)?;

    if unused != "" {
        return Err(Err::Error(Error::new(unused, ErrorKind::Eof)));
    }

    Ok((unused, expr))
}
