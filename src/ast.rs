use std::collections::HashMap;

/// Represents value that expression returns
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Int(i64),
    Double(f64),
    Bool(bool),
    Null,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Var(VarExpr),
    Lit(LitExpr),
    Cond(CondExpr),
    Check(CheckExpr),
    NullCheck(NullCheckExpr),
    None,
}

impl Expr {
    pub fn eval(&self, vars: &HashMap<String, String>) -> Value {
        match self {
            Expr::Var(v) => v.eval(vars),
            Expr::Lit(l) => l.eval(vars),
            Expr::Cond(c) => c.eval(vars),
            Expr::Check(c) => c.eval(vars),
            Expr::NullCheck(c) => c.eval(vars),
            Expr::None => Value::Null,
        }
    }
}

macro_rules! expr_struct {
    (
        $name:ident {
            $($var_name:ident: $var_type:ty),* $(,)?
        }
    ) => {
        #[derive(Debug, PartialEq)]
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
    };
}

expr_struct!(VarExpr { name: String });

impl VarExpr {
    fn eval(&self, vars: &HashMap<String, String>) -> Value {
        if let Some(val) = vars.get(&self.name) {
            Value::String(val.to_owned())
        } else {
            Value::Null
        }
    }
}

expr_struct!(LitExpr { value: Value });

impl LitExpr {
    fn eval(&self, vars: &HashMap<String, String>) -> Value {
        self.value.clone()
    }
}

expr_struct!(CondExpr {
    left: Box<Expr>,
    cond: String,
    right: Box<Expr>,
});

impl CondExpr {
    fn eval(&self, vars: &HashMap<String, String>) -> Value {
        todo!()
    }
}

expr_struct!(CheckExpr {
    cond: Box<Expr>,
    left: Box<Expr>,
    right: Box<Expr>,
});

impl CheckExpr {
    fn eval(&self, vars: &HashMap<String, String>) -> Value {
        let cond = self.cond.eval(vars);

        match cond {
            Value::Bool(false) | Value::Null => self.right.eval(vars),
            _ => self.left.eval(vars),
        }
    }
}

expr_struct!(NullCheckExpr {
    left: Box<Expr>,
    right: Box<Expr>,
});

impl NullCheckExpr {
    fn eval(&self, vars: &HashMap<String, String>) -> Value {
        let res = self.left.eval(vars);

        match res {
            Value::Null => self.right.eval(vars),
            _ => res,
        }
    }
}

expr_struct!(NoneExpr {});

impl NoneExpr {
    fn eval(&self, vars: &HashMap<String, String>) -> Value {
        Value::Null
    }
}
