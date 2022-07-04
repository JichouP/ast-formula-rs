use crate::domain::expr::Expr;
use nom::{character::complete::char, IResult};

use super::expr_parser::expr_parser;

pub fn paren_expr_parser(input: &str) -> IResult<&str, Expr> {
    let (unused, _) = char('(')(input)?;
    let (unused, expr) = expr_parser(unused)?;
    let (unused, _) = char(')')(unused)?;
    Ok((unused, expr))
}

#[test]
fn paren_expr_parser_test() {
    use crate::domain::expr::{constant_val::ConstantVal, Expr};
    let (_, actual) = paren_expr_parser("(14)").unwrap();
    let expect = Expr::ConstantVal(ConstantVal::new(14));
    assert_eq!(actual, expect);
}
