use super::Expr;

/// Enumerate operators
#[derive(Debug, PartialEq)]
pub enum BinaryOpKind {
    Add,
    Sub,
    Mul,
    Div,
}

/// Express binary operator.
#[derive(Debug, PartialEq)]
pub struct BinaryOp {
    /// Kind of binary operator.
    op_kind: BinaryOpKind,
    /// Expression on the left side of the operator.
    left_expr: Expr,
    /// Expression on the right side of the operator.
    right_expr: Expr,
}

impl BinaryOp {
    /// Create new binary operator.
    pub fn new(op_kind: BinaryOpKind, left_expr: Expr, right_expr: Expr) -> BinaryOp {
        BinaryOp {
            op_kind,
            left_expr,
            right_expr,
        }
    }

    /// Evaluate the binary operator.
    pub fn eval(&self) -> i32 {
        let left = self.left_expr.eval();
        let right = self.right_expr.eval();
        match self.op_kind {
            BinaryOpKind::Add => left + right,
            BinaryOpKind::Sub => left - right,
            BinaryOpKind::Mul => left * right,
            BinaryOpKind::Div => left / right,
        }
    }
}

#[test]
fn binary_op_test() {
    use super::{constant_val::ConstantVal, Expr};
    // (7 * (3 + 5)) / (6 - 4)
    let binary_op = BinaryOp::new(
        BinaryOpKind::Div,
        Expr::BinaryOp(Box::new(BinaryOp::new(
            BinaryOpKind::Mul,
            Expr::ConstantVal(ConstantVal::new(7)),
            Expr::BinaryOp(Box::new(BinaryOp::new(
                BinaryOpKind::Add,
                Expr::ConstantVal(ConstantVal::new(3)),
                Expr::ConstantVal(ConstantVal::new(5)),
            ))),
        ))),
        Expr::BinaryOp(Box::new(BinaryOp::new(
            BinaryOpKind::Sub,
            Expr::ConstantVal(ConstantVal::new(6)),
            Expr::ConstantVal(ConstantVal::new(4)),
        ))),
    );
    let expect = (7 * (3 + 5)) / (6 - 4);
    assert_eq!(binary_op.eval(), expect)
}
