pub mod constant_val_parser;
pub mod expr_parser;
pub mod factor_parser;
pub mod paren_expr_parser;
pub mod term_parser;

#[test]
fn digit1_test() {
    use nom::{character::complete::digit1, IResult};
    let s = "298abc59";
    let result: IResult<&str, &str> = digit1(s);
    let (unused, used) = result.expect("msg");
    assert_eq!("298", used);
    assert_eq!("abc59", unused);
}
