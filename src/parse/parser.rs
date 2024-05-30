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
        AddExpr, CheckExpr, EqualsExpr, Expr, LitExpr, NullCheckExpr, Value,
        VarExpr,
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
            token: None,
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
            token: None,
        })
    }

    /// Creates new [`Parser`] that outputs to the given string
    pub fn string(
        text: &'a mut I,
        vars: &'a HashMap<String, String>,
        out: &'a mut String,
    ) -> Result<(), Error> {
        let mut parser = Self {
            lexer: Lexer::new(text),
            output: Writer::String(out),
            vars,
            token: None,
        };
        parser.parse()
    }

    /// Parses given text
    pub fn parse(&mut self) -> Result<(), Error> {
        while let Some(c) = self.lexer.cur {
            match c {
                '{' => self.check_opening()?,
                '\\' => self.handle_escape()?,
                _ => self.output.write(c)?,
            }
            self.lexer.next_char();
        }
        Ok(())
    }

    fn handle_escape(&mut self) -> Result<(), Error> {
        self.lexer.next_char();
        match self.lexer.cur {
            Some('{') => self.output.write('{')?,
            Some(c) => self.output.write_str(&format!("\\{c}"))?,
            _ => Err(LexerErr::UnclosedBlock)?,
        };
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
        self.token = None;

        Ok(())
    }

    fn parse_expr(&mut self) -> Result<Expr, LexerErr> {
        self.next_token()?;
        let mut prev = Expr::None;

        while let Some(token) = self.token.take() {
            match token {
                Token::Question => return self.parse_check(prev),
                Token::NullCheck => return self.parse_null_check(prev),
                Token::Equals => prev = self.parse_equals(prev)?,
                Token::Ident(v) => {
                    prev = self.parse_var(prev, v.to_owned())?
                }
                Token::Literal(v) => {
                    prev = self.parse_lit(prev, v.to_owned())?
                }
                Token::OpenParen => return self.parse_paren(prev),
                Token::Plus => prev = self.parse_plus(prev)?,
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
                Token::OpenParen => return self.parse_paren(prev),
                _ => {
                    self.token = Some(token);
                    break;
                }
            }
        }
        Ok(prev)
    }

    fn parse_check(&mut self, prev: Expr) -> Result<Expr, LexerErr> {
        let left = self.parse_expr()?;

        if !matches!(self.token.take(), Some(Token::Colon)) {
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

    fn parse_paren(&mut self, _prev: Expr) -> Result<Expr, LexerErr> {
        let expr = self.parse_expr()?;

        if !matches!(self.token.take(), Some(Token::CloseParen)) {
            return Err(LexerErr::UnexpectedToken);
        }
        Ok(expr)
    }

    fn parse_plus(&mut self, prev: Expr) -> Result<Expr, LexerErr> {
        let right = self.parse_expr_hp()?;

        Ok(Expr::Add(AddExpr::new(Box::new(prev), Box::new(right))))
    }

    fn next_token(&mut self) -> Result<(), LexerErr> {
        if self.token.is_none() {
            self.token = Some(self.lexer.next()?);
        }
        Ok(())
    }
}
