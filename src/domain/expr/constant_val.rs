/// Express constant value.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ConstantVal(i32);

impl ConstantVal {
    /// Create new constant value.
    pub fn new(val: i32) -> ConstantVal {
        ConstantVal(val)
    }

    /// Evaluate the constant value.
    pub fn eval(&self) -> i32 {
        self.0
    }
}

#[test]
fn constant_val_test() {
    let expect = 123;
    let constant_val = ConstantVal::new(expect);
    assert_eq!(constant_val.eval(), expect);
}
