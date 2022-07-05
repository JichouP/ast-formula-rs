use self::{binary_op::BinaryOp, constant_val::ConstantVal};

pub mod binary_op;
pub mod constant_val;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    ConstantVal(ConstantVal),
    BinaryOp(Box<BinaryOp>),
}

impl Expr {
    pub fn eval(&self) -> i32 {
        match self {
            Expr::ConstantVal(e) => e.eval(),
            Expr::BinaryOp(e) => e.eval(),
        }
    }
}
