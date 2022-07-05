use parser::formula_parser::formula_parser;

pub mod domain;
pub mod parser;

pub fn calc(input: &str) -> Result<i32, &str> {
    let res = formula_parser(input);
    match res {
        Ok((_unused, expr)) => Ok(expr.eval()),
        Err(_) => Err("syntax error"),
    }
}

#[cfg(test)]
mod tests {
    use super::calc;

    #[test]
    fn it_works() {
        let result = calc("34 + 5 * 6 / 2 - 2 - 1").unwrap();
        assert_eq!(result, 34 + 5 * 6 / 2 - 2 - 1);
    }

    #[test]
    fn should_panic() {
        let res = calc("a1+2");
        assert_eq!(res.unwrap_err(), "syntax error")
    }
}
