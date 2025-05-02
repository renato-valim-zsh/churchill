use crate::ast::Expr;

/// A simple recursive descent parser for lambda calculus expressions.
pub struct Parser {
    input: Vec<char>,
    pos: usize,
}

impl Parser {
    /// Create a new parser for the given input string.
    pub fn new(s: &str) -> Self {
        Parser { input: s.chars().collect(), pos: 0 }
    }
    fn peek(&self) -> Option<char> { self.input.get(self.pos).copied() }
    fn next(&mut self) -> Option<char> {
        let ch = self.peek();
        if ch.is_some() { self.pos += 1; }
        ch
    }
    fn skip_ws(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() { self.next(); } else { break; }
        }
    }
    /// Parse the full input into an Expr, or panic on error.
    pub fn parse(&mut self) -> Expr {
        self.skip_ws();
        let expr = self.parse_expr();
        self.skip_ws();
        if self.pos != self.input.len() {
            panic!("Unexpected character '{}' at pos {}", self.peek().unwrap(), self.pos);
        }
        expr
    }
    fn parse_expr(&mut self) -> Expr { self.parse_lambda() }
    fn parse_lambda(&mut self) -> Expr {
        self.skip_ws();
        if let Some(ch) = self.peek() {
            if ch == '\\' || ch == 'λ' {
                self.next();
                self.skip_ws();
                let var = self.parse_var_name();
                self.skip_ws();
                if self.next() != Some('.') {
                    panic!("Expected '.' after lambda parameter");
                }
                let body = self.parse_lambda();
                return Expr::Abs(var, Box::new(body));
            }
        }
        self.parse_app()
    }
    fn parse_app(&mut self) -> Expr {
        self.skip_ws();
        let mut expr = self.parse_atom();
        loop {
            self.skip_ws();
            if let Some(ch) = self.peek() {
                if ch == ')' || ch == '.' { break; }
                let atom = self.parse_atom();
                expr = Expr::App(Box::new(expr), Box::new(atom));
            } else { break; }
        }
        expr
    }
    fn parse_atom(&mut self) -> Expr {
        self.skip_ws();
        match self.peek() {
            Some('(') => {
                self.next();
                let expr = self.parse_expr();
                self.skip_ws();
                if self.next() != Some(')') {
                    panic!("Expected ')'");
                }
                expr
            }
            Some(ch) if ch.is_alphanumeric() || ch == '_' => {
                let name = self.parse_var_name();
                Expr::Var(name)
            }
            other => panic!("Unexpected character '{:?}' at pos {}", other, self.pos),
        }
    }
    fn parse_var_name(&mut self) -> String {
        let mut s = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                s.push(ch);
                self.next();
            } else { break; }
        }
        if s.is_empty() {
            panic!("Expected variable name at pos {}", self.pos);
        }
        s
    }
}

/// Top-level parse function.
pub fn parse(s: &str) -> Expr {
    Parser::new(s).parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_var() {
        assert_eq!(format!("{}", parse("x")), "x");
    }

    #[test]
    fn test_parse_abs() {
        assert_eq!(format!("{}", parse("\\x.x")), "(λx.x)");
    }

    #[test]
    fn test_parse_app() {
        assert_eq!(format!("{}", parse("f x")), "(f x)");
    }
}
