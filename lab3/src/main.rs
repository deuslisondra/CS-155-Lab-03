use ArithCmpOp::*;
use ArithExpr::*;
use BinArithOp::*;
use BinLogicOp::*;
use BoolExpr::*;
use Expr::*;
use Value::*;

pub enum Expr {
    ArithExpr(ArithExpr),
    BoolExpr(BoolExpr),
}

pub enum ArithExpr {
    BinArithExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: BinArithOp,
    },
    IntLit(i64),
}

pub enum BoolExpr {
    ArithCmpExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: ArithCmpOp,
    },
    BinBoolExpr {
        left: Box<BoolExpr>,
        right: Box<BoolExpr>,
        op: BinLogicOp,
    },
    NotExpr(Box<BoolExpr>),
    BoolLit(bool),
}

pub enum BinArithOp {
    AddOp,
    SubOp,
    MulOp,
    IntDivOp,
}

pub enum ArithCmpOp {
    LtOp,
    LteOp,
    GtOp,
    GteOp,
    ArithEqOp,
    ArithNeqOp,
}

pub enum BinLogicOp {
    AndOp,
    OrOp,
    BoolEqOp,
    BoolNeqOp,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    BoolValue(bool),
    IntValue(i64),
}

pub fn eval(expr: Expr) -> Value {
    match expr{
        ArithExpr(arith_expr) => IntValue(eval_arith_expr(arith_expr)),
        BoolExpr(bool_expr) => BoolValue(eval_bool_expr(bool_expr)),
    }
}

pub fn eval_arith_expr(arith_expr: ArithExpr) -> i64 {
    match arith_expr{
        IntLit(val) => val,
        BinArithExpr{left, right, op} =>{
            let left_val = eval_arith_expr(*left);
            let right_val = eval_arith_expr(*right);
            match op{
                AddOp => left_val + right_val,
                SubOp => left_val - right_val,
                MulOp => left_val * right_val,
                IntDivOp => left_val / right_val,
            }
        }
    }
}

pub fn eval_bool_expr(bool_expr: BoolExpr) -> bool {
    match bool_expr{
        BoolLit(val) => val,
        ArithCmpExpr{left, right, op} =>{
            let left_val = eval_arith_expr(*left);
            let right_val = eval_arith_expr(*right);
            match op{
                LtOp => left_val < right_val,
                LteOp => left_val <= right_val,
                GtOp => left_val > right_val,
                GteOp => left_val >= right_val,
                ArithEqOp => left_val == right_val,
                ArithNeqOp => left_val != right_val,
            }
        }
        BinBoolExpr{left, right, op} =>{
            let left_val = eval_bool_expr(*left);
            let right_val = eval_bool_expr(*right);
            match op{
                AndOp => left_val && right_val,
                OrOp => left_val || right_val,
                BoolEqOp => left_val == right_val,
                BoolNeqOp => left_val != right_val,
            }
        }
        NotExpr(val) => !eval_bool_expr(*val),
    }
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool_not(){
    	let expr = BoolExpr(NotExpr(Box::new(BoolLit(true))));

    	assert_eq!(eval(expr), BoolValue(false)); // !true == false
    }

    #[test]
    fn test_bool_expr() {
        let expr1 = BoolExpr(BinBoolExpr{left: Box::new(BoolLit(true)), right: Box::new(BoolLit(false)), op: AndOp});
        let expr2 = BoolExpr(BinBoolExpr{left: Box::new(BoolLit(false)), right: Box::new(BoolLit(true)), op: OrOp});
        let expr3 = BoolExpr(BinBoolExpr{left: Box::new(BoolLit(true)), right: Box::new(BoolLit(false)), op: BoolEqOp});
        let expr4 = BoolExpr(BinBoolExpr{left: Box::new(BoolLit(true)), right: Box::new(BoolLit(false)), op: BoolNeqOp});

        assert_eq!(eval(expr1), BoolValue(false));  // (true AND false) == false
        assert_eq!(eval(expr2), BoolValue(true));  // (true OR false) == true
        assert_eq!(eval(expr3), BoolValue(false));  // (true == false) == false
        assert_eq!(eval(expr4), BoolValue(true));  // (true != false) == true
    }

    #[test]
    fn test_arith_comp() {
        let expr1 = BoolExpr(ArithCmpExpr{left: Box::new(IntLit(2)), right: Box::new(IntLit(1)), op: LtOp});
        let expr2 = BoolExpr(ArithCmpExpr{left: Box::new(IntLit(2)), right: Box::new(IntLit(1)), op: LteOp});
        let expr3 = BoolExpr(ArithCmpExpr{left: Box::new(IntLit(2)), right: Box::new(IntLit(1)), op: GtOp});
        let expr4 = BoolExpr(ArithCmpExpr{left: Box::new(IntLit(2)), right: Box::new(IntLit(1)), op: GteOp});
        let expr5 = BoolExpr(ArithCmpExpr{left: Box::new(IntLit(2)), right: Box::new(IntLit(1)), op: ArithEqOp});
        let expr6 = BoolExpr(ArithCmpExpr{left: Box::new(IntLit(2)), right: Box::new(IntLit(1)), op: ArithNeqOp});

        assert_eq!(eval(expr1), BoolValue(false));  // (2 < 1) == false
        assert_eq!(eval(expr2), BoolValue(false));  // (2 <= 1) == false
        assert_eq!(eval(expr3), BoolValue(true));  // (2 > 1) == true
        assert_eq!(eval(expr4), BoolValue(true));  // (2 >= 1) == true
        assert_eq!(eval(expr5), BoolValue(false));  // (2 = 1) == false
        assert_eq!(eval(expr6), BoolValue(true));  // (2 != 1) == true
    }

    #[test]
    fn test_arith_expr() {
        let expr1 = ArithExpr(BinArithExpr{left: Box::new(IntLit(2)), right: Box::new(IntLit(1)), op: AddOp});
        let expr2 = ArithExpr(BinArithExpr{left: Box::new(IntLit(2)), right: Box::new(IntLit(1)), op: SubOp});
        let expr3 = ArithExpr(BinArithExpr{left: Box::new(IntLit(2)), right: Box::new(IntLit(1)), op: MulOp});
        let expr4 = ArithExpr(BinArithExpr{left: Box::new(IntLit(2)), right: Box::new(IntLit(1)), op: IntDivOp});

        assert_eq!(eval(expr1), IntValue(3));  // 2 + 1 == 3
        assert_eq!(eval(expr2), IntValue(1));  // 2 - 1 == 1
        assert_eq!(eval(expr3), IntValue(2));  // 2 * 1 == 1
        assert_eq!(eval(expr4), IntValue(2));  // 2 / 1 == 1
    }

    #[test]
    fn test_sample2() {
        let expr = ArithExpr(IntLit(0));
        let answer = IntValue(0);

        assert_eq!(eval(expr), answer);  // eval(ArithExpr(IntLit(0))) == IntValue(0)
    }

    #[test]
    fn test_sample() {
        let expr = BoolExpr(BoolLit(true));
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);  // eval(BoolExpr(BoolLit(true))) == BoolValue(true)
    }

    #[test]
    fn test_others() {
        main();
        println!("{:?}", BoolValue(true));
    }
}