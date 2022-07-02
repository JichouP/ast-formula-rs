pub mod domain;

pub fn add(left: usize, right: usize) -> usize {
    let a = domain::expr::constant_val::ConstantVal::new(1);
    println!("{}", a.eval());
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
