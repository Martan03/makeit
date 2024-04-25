/// Represents value that expression returns
#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Int(i64),
    Double(f64),
    Bool(bool),
    Null,
}

pub trait Expr {
    fn eval(&self) -> Value;
}

macro_rules! expr_struct {
    (
        $name:ident {
            $($var_name:ident: $var_type:ty),* $(,)?
        } => {
            $($code:tt)*
        }
    ) => {
        pub struct $name {
            $(
                $var_name: $var_type,
            )*
        }

        impl $name {
            pub fn new($($var_name: $var_type,)*) -> Self {
                Self {
                    $($var_name,)*
                }
            }
        }

        impl Expr for $name {
            fn eval(&self) -> Value {
                $($code)*
            }
        }
    };
}

expr_struct!(CondExpr {
    left: Box<dyn Expr>,
    cond: String,
    right: Box<dyn Expr>,
} => {
    todo!()
});

expr_struct!(VarExpr { value: Value } => {
    todo!()
});

expr_struct!(CheckExpr {
    cond: Box<dyn Expr>,
    left: Box<dyn Expr>,
    right: Box<dyn Expr>,
} => {
    todo!()
});

expr_struct!(LitExpr {
    value: Value,
} => {
    todo!()
});
