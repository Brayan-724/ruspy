#[macro_export]
macro_rules! scope {
    ($($expr:expr),*) => {
        $crate::ast::node::AstScope(Vec::from([ $($expr),* ]))
    };
}

#[macro_export]
macro_rules! bin_op {
    ($a:expr, $op:ident, $b:expr) => {
        $crate::ast::node::AstExpr::BinaryOp {
            op: $crate::ast::node::AstBinaryOp::$op,
            left: $a.into(),
            right: $b.into(),
        }
    };
}

#[macro_export]
macro_rules! unary_op {
    ($op:ident, $b:expr) => {
        $crate::ast::node::AstExpr::UnaryOp {
            op: $crate::ast::node::AstUnaryOp::$op,
            right: $a.into(),
        }
    };
}

pub use bin_op;
pub use scope;
pub use unary_op;
