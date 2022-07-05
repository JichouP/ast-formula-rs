use crate::{
    domain::expr::{
        binary_op::{BinaryOp, BinaryOpKind},
        Expr,
    },
    parser::term_parser::term_parser,
};
use nom::{
    branch::alt, character::complete::char, character::complete::space0, combinator::map,
    sequence::tuple, IResult,
};

pub fn expr_parser(input: &str) -> IResult<&str, Expr> {
    // Parser to parse symbols into OpKind.
    fn op_kind_parser(input: &str) -> IResult<&str, BinaryOpKind> {
        map(
            tuple((space0, alt((char('+'), char('-'))), space0)),
            |(_, op_char, _)| match op_char {
                '+' => BinaryOpKind::Add,
                '-' => BinaryOpKind::Sub,
                _ => panic!(""),
            },
        )(input)
    }

    fn binary_parser(input: &str, left_expr: Expr) -> IResult<&str, Expr> {
        map(
            tuple((op_kind_parser, term_parser)),
            |(op_kind, right_expr)| {
                Expr::BinaryOp(Box::new(BinaryOp::new(
                    op_kind,
                    left_expr.clone(),
                    right_expr,
                )))
            },
        )(input)
    }

    let (mut unused, mut tmp_left_expr) = term_parser(input)?;

    while let Ok((_, _)) = op_kind_parser(unused) {
        let (new_unused, used) = binary_parser(unused, tmp_left_expr.clone())?;
        tmp_left_expr = used;
        unused = new_unused;
    }

    Ok((unused, tmp_left_expr.clone()))
}

#[test]
fn exprs_parser_test() {
    use crate::domain::expr::{binary_op::BinaryOpKind, constant_val::ConstantVal, Expr};

    let expect = Expr::BinaryOp(Box::new(BinaryOp::new(
        BinaryOpKind::Add,
        Expr::BinaryOp(Box::new(BinaryOp::new(
            BinaryOpKind::Div,
            Expr::BinaryOp(Box::new(BinaryOp::new(
                BinaryOpKind::Mul,
                Expr::ConstantVal(ConstantVal::new(4)),
                Expr::ConstantVal(ConstantVal::new(3)),
            ))),
            Expr::ConstantVal(ConstantVal::new(2)),
        ))),
        Expr::ConstantVal(ConstantVal::new(1)),
    )));

    let (_, actual) = expr_parser("4*3/2+1").unwrap();
    assert_eq!(actual, expect);

    let (_, actual) = expr_parser(" 4 *\t 3 /   2 + 1  ").unwrap();
    assert_eq!(actual, expect);
    assert_eq!(actual.eval(), 4 * 3 / 2 + 1);

    let (_, actual) = expr_parser("3-2-1").unwrap();
    assert_eq!(actual.eval(), 3 - 2 - 1);
}
