use super::expr_parser::expr_parser;
use crate::domain::expr::Expr;
use nom::{
    character::complete::{char, space0},
    sequence::tuple,
    IResult,
};

pub fn paren_expr_parser(input: &str) -> IResult<&str, Expr> {
    let (unused, (_, _, expr, _, _)) =
        tuple((char('('), space0, expr_parser, space0, char(')')))(input).unwrap();
    Ok((unused, expr))
}

#[test]
fn paren_expr_parser_test() {
    use crate::domain::expr::{constant_val::ConstantVal, Expr};
    let (_, actual) = paren_expr_parser("( 14 )").unwrap();
    let expect = Expr::ConstantVal(ConstantVal::new(14));
    assert_eq!(actual, expect);
}
