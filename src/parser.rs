use std::collections::HashMap;

use crate::{
    ast::{CheckExpr, Expr, LitExpr, NullCheckExpr, Value, VarExpr},
    err::lexer_err::LexerErr,
    lexer::{Lexer, Token},
};

pub struct Parser<'a, I>
where
    I: Iterator<Item = char>,
{
    lexer: Lexer<'a, I>,
    token: Token,
}

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = char>,
{
    /// Creates new [`Parser`]
    pub fn new(text: &'a mut I) -> Self {
        Self {
            lexer: Lexer::new(text),
            token: Token::End,
        }
    }

    /// Parses given text
    pub fn parse(&mut self) -> Result<(), String> {
        // while let Some(c) = self.text.next() {
        //     if c == '{' {
        //         self.check_opening().map_err(|e| e.to_string())?;
        //     } else {
        //         print!("{c}");
        //     }
        // }
        self.handle_code().map_err(|e| e.to_string())
    }

    fn check_opening(&mut self) -> Result<(), LexerErr> {
        // let Some(c) = self.text.next() else {
        //     print!("{{");
        //     return Ok(());
        // };

        // if c != '{' {
        //     print!("{{{c}");
        //     return Ok(());
        // }

        self.handle_code()
    }

    fn handle_code(&mut self) -> Result<(), LexerErr> {
        println!();
        let expr = self.parse_expr()?;
        let mut vars: HashMap<String, String> = HashMap::new();
        vars.insert("val".to_string(), "first".to_string());
        println!("{:?}", expr.eval(&vars));

        Ok(())
    }

    fn parse_expr(&mut self) -> Result<Expr, LexerErr> {
        self.token = self.lexer.next()?;
        let mut prev = Expr::None;

        loop {
            match &self.token {
                Token::Question => return self.parse_check(prev),
                Token::NullCheck => return self.parse_null_check(prev),
                Token::Ident(v) => {
                    prev = self.parse_var(prev, v.to_owned())?
                }
                Token::Literal(v) => {
                    prev = self.parse_lit(prev, v.to_owned())?
                }
                _ => break,
            };
            self.token = self.lexer.next()?;
        }
        Ok(prev)
    }

    fn parse_check(&mut self, prev: Expr) -> Result<Expr, LexerErr> {
        let left = self.parse_expr()?;

        if self.token != Token::Colon {
            return Err(LexerErr::UnexpectedToken);
        }

        let right = self.parse_expr()?;

        Ok(Expr::Check(CheckExpr::new(
            Box::new(prev),
            Box::new(left),
            Box::new(right),
        )))
    }

    fn parse_null_check(&mut self, prev: Expr) -> Result<Expr, LexerErr> {
        let right = self.parse_expr()?;

        Ok(Expr::NullCheck(NullCheckExpr::new(
            Box::new(prev),
            Box::new(right),
        )))
    }

    fn parse_var(&self, prev: Expr, name: String) -> Result<Expr, LexerErr> {
        match prev {
            Expr::None => Ok(Expr::Var(VarExpr::new(name))),
            _ => Err(LexerErr::UnexpectedToken),
        }
    }

    fn parse_lit(&self, prev: Expr, val: String) -> Result<Expr, LexerErr> {
        match prev {
            Expr::None => Ok(Expr::Lit(LitExpr::new(Value::String(val)))),
            _ => Err(LexerErr::UnexpectedToken),
        }
    }
}
