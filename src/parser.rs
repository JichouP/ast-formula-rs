pub mod constant_val_parser;

#[test]
fn digit1_test() {
    use nom::{character::complete::digit1, IResult};
    let s = "298abc59";
    let result: IResult<&str, &str> = digit1(s);
    let (no_used, used) = result.expect("msg");
    assert_eq!("298", used);
    assert_eq!("abc59", no_used);
}
