use crate::domain::expr::constant_val::ConstantVal;
use nom::{character::complete::digit1, IResult};
use std::str::FromStr;

pub fn constant_val_parser(input: &str) -> IResult<&str, ConstantVal> {
    let (no_used, used) = digit1(input)?;
    let val = FromStr::from_str(used).unwrap();
    Ok((no_used, ConstantVal::new(val)))
}

#[test]
fn test_constant_val_parser() {
    let (_, actual) = constant_val_parser("71ai38").unwrap();
    let expect = ConstantVal::new(71);
    assert_eq!(actual, expect);
}
