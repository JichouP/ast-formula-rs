use super::{constant_val_parser::constant_val_parser, paren_expr_parser::paren_expr_parser};
use crate::domain::expr::Expr;
use nom::{branch::alt, character::complete::space0, combinator::map, sequence::tuple, IResult};

pub fn factor_parser(input: &str) -> IResult<&str, Expr> {
    let (unused, (_, expr, _)) = tuple((
        space0,
        alt((
            map(constant_val_parser, |constant_val| {
                Expr::ConstantVal(constant_val)
            }),
            paren_expr_parser,
        )),
        space0,
    ))(input)
    .unwrap();
    Ok((unused, expr))
}

#[test]
fn factor_parser_test() {
    use crate::domain::expr::{constant_val::ConstantVal, Expr};
    let (_, actual) = factor_parser("   235 \n").unwrap();
    let expect = Expr::ConstantVal(ConstantVal::new(235));
    assert_eq!(actual, expect);

    let (_, actual) = factor_parser("(51)").unwrap();
    let expect = Expr::ConstantVal(ConstantVal::new(51));
    assert_eq!(actual, expect);
}
