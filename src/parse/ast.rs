use std::{collections::HashMap, fmt::Display};

/// Represents value that expression returns
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Bool(bool),
    Null,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "{s}"),
            Value::Bool(true) => write!(f, "true"),
            Value::Bool(false) => write!(f, "false"),
            Value::Null => write!(f, "null"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Var(VarExpr),
    Lit(LitExpr),
    Check(CheckExpr),
    NullCheck(NullCheckExpr),
    Equals(EqualsExpr),
    Add(AddExpr),
    None,
}

impl Expr {
    pub fn eval(&self, vars: &HashMap<String, String>) -> Value {
        match self {
            Expr::Var(v) => v.eval(vars),
            Expr::Lit(l) => l.eval(vars),
            Expr::Check(c) => c.eval(vars),
            Expr::NullCheck(c) => c.eval(vars),
            Expr::Equals(e) => e.eval(vars),
            Expr::Add(e) => e.eval(vars),
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
    fn eval(&self, _vars: &HashMap<String, String>) -> Value {
        self.value.clone()
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

expr_struct!(EqualsExpr {
    left: Box<Expr>,
    right: Box<Expr>,
});

impl EqualsExpr {
    fn eval(&self, vars: &HashMap<String, String>) -> Value {
        let left = self.left.eval(vars);
        let right = self.right.eval(vars);

        Value::Bool(left == right)
    }
}

expr_struct!(AddExpr {
    left: Box<Expr>,
    right: Box<Expr>,
});

impl AddExpr {
    fn eval(&self, vars: &HashMap<String, String>) -> Value {
        let left = self.left.eval(vars);
        let right = self.right.eval(vars);

        Value::String(left.to_string() + &right.to_string())
    }
}
