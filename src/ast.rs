use std::collections::HashSet;
use std::fmt;

/// The core lambda calculus expression AST.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Var(String),
    Abs(String, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Var(name) => write!(f, "{}", name),
            Expr::Abs(param, body) => write!(f, "(λ{}.{})", param, body),
            Expr::App(func, arg) => write!(f, "({} {})", func, arg),
        }
    }
}

impl Expr {
    /// Compute the set of free variables in the expression.
    pub fn free_vars(&self) -> HashSet<String> {
        match self {
            Expr::Var(n) => [n.clone()].iter().cloned().collect(),
            Expr::Abs(param, body) => {
                let mut s = body.free_vars();
                s.remove(param);
                s
            }
            Expr::App(f, a) => {
                let mut s = f.free_vars();
                s.extend(a.free_vars());
                s
            }
        }
    }
}