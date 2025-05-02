use crate::ast::Expr;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Counter for generating fresh variable names.
static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Generate a fresh variable name based on the given base.
fn fresh_var(base: &str) -> String {
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("{}_{}", base, id)
}

/// Rename occurrences of `old` to `new` in the expression.
fn rename(expr: &Expr, old: &str, new: &str) -> Expr {
    match expr {
        Expr::Var(n) => {
            if n == old { Expr::Var(new.to_string()) } else { Expr::Var(n.clone()) }
        }
        Expr::Abs(param, body) => {
            let new_param = if param == old { new.to_string() } else { param.clone() };
            let new_body = rename(body, old, new);
            Expr::Abs(new_param, Box::new(new_body))
        }
        Expr::App(f, a) => Expr::App(
            Box::new(rename(f, old, new)),
            Box::new(rename(a, old, new)),
        ),
    }
}

/// Substitute `val` for variable `var` in `expr`, avoiding capture.
fn substitute(expr: &Expr, var: &str, val: &Expr) -> Expr {
    match expr {
        Expr::Var(n) => {
            if n == var { val.clone() } else { Expr::Var(n.clone()) }
        }
        Expr::Abs(param, body) => {
            if param == var {
                Expr::Abs(param.clone(), body.clone())
            } else if val.free_vars().contains(param) {
                let new_param = fresh_var(param);
                let renamed = rename(body, param, &new_param);
                Expr::Abs(new_param.clone(), Box::new(substitute(&renamed, var, val)))
            } else {
                Expr::Abs(param.clone(), Box::new(substitute(body, var, val)))
            }
        }
        Expr::App(f, a) => Expr::App(
            Box::new(substitute(f, var, val)),
            Box::new(substitute(a, var, val)),
        ),
    }
}

/// Perform one beta-reduction step, if possible.
fn reduce_once(expr: &Expr) -> Option<Expr> {
    match expr {
        Expr::App(f, a) => {
            if let Expr::Abs(param, body) = &**f {
                Some(substitute(body, param, a))
            } else if let Some(new_f) = reduce_once(f) {
                Some(Expr::App(Box::new(new_f), a.clone()))
            } else if let Some(new_a) = reduce_once(a) {
                Some(Expr::App(f.clone(), Box::new(new_a)))
            } else {
                None
            }
        }
        Expr::Abs(param, body) => {
            if let Some(new_body) = reduce_once(body) {
                Some(Expr::Abs(param.clone(), Box::new(new_body)))
            } else {
                None
            }
        }
        Expr::Var(_) => None,
    }
}

/// Evaluate an expression to normal form by repeated reduction.
pub fn evaluate(expr: &Expr) -> Expr {
    let mut current = expr.clone();
    // TODO: Handle properly the maximum amount of reductions allowed
    for _ in 0..100000000 {
        if let Some(next) = reduce_once(&current) {
            current = next;
        } else {
            return current;
        }
    }
    panic!("Maximum reduction steps exceeded");
}

/// Expand defined variables from the environment.
pub fn expand(expr: &Expr, env: &HashMap<String, Expr>) -> Expr {
    match expr {
        Expr::Var(name) => {
            if let Some(def) = env.get(name) {
                expand(def, env)
            } else {
                Expr::Var(name.clone())
            }
        }
        Expr::Abs(param, body) => Expr::Abs(param.clone(), Box::new(expand(body, env))),
        Expr::App(f, a) => Expr::App(Box::new(expand(f, env)), Box::new(expand(a, env))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    #[test]
    fn test_identity() {
        let expr = parse("(\\x.x) y");
        let res = evaluate(&expr);
        assert_eq!(res, Expr::Var("y".to_string()));
    }

    #[test]
    fn test_k_combinator() {
        let expr = parse("(\\x.\\y.x) a b");
        let res = evaluate(&expr);
        assert_eq!(res, Expr::Var("a".to_string()));
    }
}
