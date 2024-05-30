use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufWriter},
    path::PathBuf,
};

use crate::{
    err::{error::Error, lexer_err::LexerErr},
    writer::Writer,
};

use super::{
    ast::{
        CheckExpr, EqualsExpr, Expr, LitExpr, NullCheckExpr, Value, VarExpr,
    },
    lexer::{Lexer, Token},
};

pub struct Parser<'a, I>
where
    I: Iterator<Item = Result<char, io::Error>>,
{
    lexer: Lexer<'a, I>,
    output: Writer<'a>,
    vars: &'a HashMap<String, String>,
    token: Option<Token>,
}

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = Result<char, io::Error>>,
{
    /// Creates new [`Parser`]
    /// This exists for testing purposes
    #[allow(unused)]
    pub fn new(text: &'a mut I, vars: &'a HashMap<String, String>) -> Self {
        Self {
            lexer: Lexer::new(text),
            output: Writer::Stdout,
            vars,
            token: Some(Token::End),
        }
    }

    /// Creates new [`Parser`] that outputs to the file
    pub fn file(
        text: &'a mut I,
        vars: &'a HashMap<String, String>,
        file: &PathBuf,
    ) -> Result<Self, io::Error> {
        Ok(Self {
            lexer: Lexer::new(text),
            output: Writer::File(BufWriter::<File>::new(File::create(file)?)),
            vars,
            token: Some(Token::End),
        })
    }

    /// Creates new [`Parser`] that outputs to the given string
    pub fn string(
        text: &'a mut I,
        vars: &'a HashMap<String, String>,
        out: &'a mut String,
    ) -> Self {
        Self {
            lexer: Lexer::new(text),
            output: Writer::String(out),
            vars,
            token: Some(Token::End),
        }
    }

    /// Parses given text
    pub fn parse(&mut self) -> Result<(), Error> {
        while let Some(c) = self.lexer.cur {
            if c == '{' {
                self.check_opening()?;
            } else {
                self.output.write(c)?;
            }
            self.lexer.next_char();
        }
        Ok(())
    }

    fn check_opening(&mut self) -> Result<(), LexerErr> {
        self.lexer.next_char();
        let Some(c) = self.lexer.cur else {
            _ = self.output.write('{');
            return Ok(());
        };

        if c != '{' {
            _ = self.output.write_str(&format!("{{{c}"));
            return Ok(());
        }

        self.lexer.next_char();
        self.handle_code()
    }

    fn handle_code(&mut self) -> Result<(), LexerErr> {
        let expr = self.parse_expr()?;
        _ = self.output.write_str(&format!("{}", expr.eval(&self.vars)));

        Ok(())
    }

    fn parse_expr(&mut self) -> Result<Expr, LexerErr> {
        self.next_token()?;
        let mut prev = Expr::None;

        while let Some(token) = self.token.take() {
            match token {
                Token::Question => return self.parse_check(prev),
                Token::NullCheck => return self.parse_null_check(prev),
                Token::Equals => return self.parse_equals(prev),
                Token::Ident(v) => {
                    prev = self.parse_var(prev, v.to_owned())?
                }
                Token::Literal(v) => {
                    prev = self.parse_lit(prev, v.to_owned())?
                }
                _ => {
                    self.token = Some(token);
                    break;
                }
            }
            self.next_token()?;
        }
        Ok(prev)
    }

    fn parse_expr_hp(&mut self) -> Result<Expr, LexerErr> {
        self.next_token()?;
        let prev = Expr::None;

        while let Some(token) = self.token.take() {
            match token {
                Token::Ident(v) => return self.parse_var(prev, v.to_owned()),
                Token::Literal(v) => {
                    return self.parse_lit(prev, v.to_owned())
                }
                _ => break,
            }
        }
        Ok(prev)
    }

    fn parse_check(&mut self, prev: Expr) -> Result<Expr, LexerErr> {
        let left = self.parse_expr()?;

        if !matches!(self.token, Some(Token::Colon)) {
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

    fn parse_equals(&mut self, prev: Expr) -> Result<Expr, LexerErr> {
        let right = self.parse_expr_hp()?;

        Ok(Expr::Equals(EqualsExpr::new(
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

    fn next_token(&mut self) -> Result<(), LexerErr> {
        if self.token.is_none() {
            self.token = Some(self.lexer.next()?);
        }
        Ok(())
    }
}
