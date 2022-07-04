use crate::domain::expr::Expr;
use nom::{character::complete::char, IResult};

use super::expr_parser::expr_parser;

pub fn paren_expr_parser(input: &str) -> IResult<&str, Expr> {
    let (unused, _) = char('(')(input)?;
    let (unused, expr) = expr_parser(input)?;
    let (unused, _) = char(')')(input)?;
    Ok((unused, expr))
}
