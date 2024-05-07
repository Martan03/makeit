use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufWriter},
    path::PathBuf,
};

use crate::{
    ast::{CheckExpr, Expr, LitExpr, NullCheckExpr, Value, VarExpr},
    err::lexer_err::LexerErr,
    lexer::{Lexer, Token},
    writer::Writer,
};

pub struct Parser<'a, I>
where
    I: Iterator<Item = Result<char, io::Error>>,
{
    lexer: Lexer<'a, I>,
    output: Writer<'a>,
    vars: HashMap<String, String>,
    token: Token,
}

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = Result<char, io::Error>>,
{
    /// Creates new [`Parser`]
    /// This exists for testing purposes
    #[allow(unused)]
    pub fn new(text: &'a mut I, vars: HashMap<String, String>) -> Self {
        Self {
            lexer: Lexer::new(text),
            output: Writer::Stdout,
            vars,
            token: Token::End,
        }
    }

    /// Creates new [`Parser`] that outputs to the file
    pub fn file(
        text: &'a mut I,
        vars: HashMap<String, String>,
        file: &PathBuf,
    ) -> Result<Self, io::Error> {
        Ok(Self {
            lexer: Lexer::new(text),
            output: Writer::File(BufWriter::<File>::new(File::create(file)?)),
            vars,
            token: Token::End,
        })
    }

    /// Creates new [`Parser`] that outputs to the given string
    pub fn string(
        text: &'a mut I,
        vars: HashMap<String, String>,
        out: &'a mut String,
    ) -> Self {
        Self {
            lexer: Lexer::new(text),
            output: Writer::String(out),
            vars,
            token: Token::End,
        }
    }

    /// Parses given text
    pub fn parse(&mut self) -> Result<(), String> {
        while let Some(c) = self.lexer.cur {
            if c == '{' {
                self.check_opening().map_err(|e| e.to_string())?;
            } else {
                self.output.write(c).map_err(|e| e.to_string())?;
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
