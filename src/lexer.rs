use std::io;

use crate::err::lexer_err::LexerErr;

/// Represents token read by the lexer
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Colon,
    Question,
    NullCheck,
    Ident(String),
    Literal(String),
    End,
}

/// Provides lexical analysis of the file
#[derive(Debug)]
pub struct Lexer<'a, I>
where
    I: Iterator<Item = Result<char, io::Error>>,
{
    text: &'a mut I,
    pub cur: Option<char>,
}

impl<'a, I> Lexer<'a, I>
where
    I: Iterator<Item = Result<char, io::Error>>,
{
    /// Creates new [`Lexer`]
    pub fn new(text: &'a mut I) -> Self {
        let mut lex = Self {
            text: text,
            cur: None,
        };
        lex.next_char();
        lex
    }

    /// Gets next [`Token`]
    pub fn next(&mut self) -> Result<Token, LexerErr> {
        while let Some(c) = self.cur {
            if !c.is_whitespace() {
                break;
            }
            self.next_char();
        }

        match self.cur {
            Some('?') => self.read_question(),
            Some(':') => {
                self.next_char();
                Ok(Token::Colon)
            }
            Some('"') => self.read_literal(),
            Some(c) if c.is_alphabetic() || c == '_' => self.read_ident(),
            Some('}') => {
                self.next_char();
                if self.cur == Some('}') {
                    Ok(Token::End)
                } else {
                    self.next_char();
                    Err(LexerErr::InvalidToken)
                }
            }
            None => Err(LexerErr::UnclosedBlock),
            _ => return Err(LexerErr::InvalidToken),
        }
    }

    /// Reads question and check, whether it's no check or if
    fn read_question(&mut self) -> Result<Token, LexerErr> {
        self.next_char();

        Ok(match self.cur {
            Some('?') => {
                self.next_char();
                Token::NullCheck
            }
            _ => Token::Question,
        })
    }

    /// Reads identifier and check whether it contains allowed characters
    fn read_ident(&mut self) -> Result<Token, LexerErr> {
        let mut res = String::new();
        while let Some(c) = self.cur {
            if c.is_whitespace() {
                break;
            }

            if !c.is_alphanumeric() && c != '_' {
                return Err(LexerErr::InvalidIdent);
            }

            res.push(c);
            self.next_char();
        }
        Ok(Token::Ident(res))
    }

    /// Reads literal
    fn read_literal(&mut self) -> Result<Token, LexerErr> {
        self.next_char();
        let mut res = String::new();
        while let Some(c) = self.cur {
            if c == '"' {
                self.next_char();
                return Ok(Token::Literal(res));
            } else if c == '\\' {
                self.next_char();
                if self.cur == None {
                    return Err(LexerErr::UnclosedLit);
                }
            }

            res.push(c);
            self.next_char();
        }
        Err(LexerErr::UnclosedLit)
    }

    /// Gets next char from the text
    pub fn next_char(&mut self) {
        match self.text.next() {
            Some(Ok(c)) => self.cur = Some(c),
            _ => self.cur = None,
        }
    }
}
