use parser::expr_parser::expr_parser;

pub mod domain;
pub mod parser;

pub fn calc(input: &str) -> i32 {
    let (_, expr) = expr_parser(input).expect("syntax error");
    expr.eval()
}

#[cfg(test)]
mod tests {
    use super::calc;

    #[test]
    fn it_works() {
        let result = calc("34 + 5 * 6 / 2 - 2 - 1");
        assert_eq!(result, 34 + 5 * 6 / 2 - 2 - 1);
    }

    #[test]
    #[should_panic = "syntax error"]
    fn should_panic() {
        calc("a1+2");
    }
}
