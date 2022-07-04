use super::{constant_val_parser::constant_val_parser, paren_expr_parser::paren_expr_parser};
use crate::domain::expr::Expr;
use nom::{branch::alt, combinator::map, IResult};

pub fn factor_parser(input: &str) -> IResult<&str, Expr> {
    alt((
        map(constant_val_parser, |constant_val| {
            Expr::ConstantVal(constant_val)
        }),
        paren_expr_parser,
    ))(input)
}

#[test]
fn factor_parser_test() {
    use crate::domain::expr::{constant_val::ConstantVal, Expr};
    let (_, actual) = factor_parser("235").unwrap();
    let expect = Expr::ConstantVal(ConstantVal::new(235));
    assert_eq!(actual, expect);

    let (_, actual) = factor_parser("(51)").unwrap();
    let expect = Expr::ConstantVal(ConstantVal::new(51));
    assert_eq!(actual, expect);
}
