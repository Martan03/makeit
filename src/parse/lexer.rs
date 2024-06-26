use std::io;

use crate::err::lexer_err::LexerErr;

/// Represents token read by the lexer
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Colon,
    Question,
    NullCheck,
    Equals,
    Ident(String),
    Literal(String),
    OpenParen,
    CloseParen,
    Plus,
    End,
}

/// Provides lexical analysis of the text
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
        self.skip_whitespace();

        match self.cur {
            Some('?') => Ok(self.read_question()),
            Some(':') => {
                self.next_char();
                Ok(Token::Colon)
            }
            Some('=') => self.read_equals(),
            Some('"') => self.read_literal(),
            Some(c) if c.is_alphabetic() || c == '_' => Ok(self.read_ident()),
            Some('}') => {
                self.next_char();
                if self.cur == Some('}') {
                    Ok(Token::End)
                } else {
                    self.next_char();
                    Err(LexerErr::InvalidToken)
                }
            }
            Some('(') => {
                self.next_char();
                Ok(Token::OpenParen)
            }
            Some(')') => {
                self.next_char();
                Ok(Token::CloseParen)
            }
            Some('+') => {
                self.next_char();
                Ok(Token::Plus)
            }
            None => Err(LexerErr::UnclosedBlock),
            _ => Err(LexerErr::InvalidToken),
        }
    }

    /// Gets next char from the text
    pub fn next_char(&mut self) {
        match self.text.next() {
            Some(Ok(c)) => self.cur = Some(c),
            _ => self.cur = None,
        }
    }

    /// Reads question or null check
    fn read_question(&mut self) -> Token {
        self.next_char();

        match self.cur {
            Some('?') => {
                self.next_char();
                Token::NullCheck
            }
            _ => Token::Question,
        }
    }

    /// Reads equals
    fn read_equals(&mut self) -> Result<Token, LexerErr> {
        self.next_char();

        match self.cur {
            Some('=') => {
                self.next_char();
                Ok(Token::Equals)
            }
            _ => Err(LexerErr::InvalidToken),
        }
    }

    /// Reads identifier and checks whether it contains allowed characters
    fn read_ident(&mut self) -> Token {
        let mut res = String::new();
        while let Some(c) = self.cur {
            if c.is_whitespace() || (!c.is_alphanumeric() && c != '_') {
                break;
            }

            res.push(c);
            self.next_char();
        }
        Token::Ident(res)
    }

    /// Reads literal
    fn read_literal(&mut self) -> Result<Token, LexerErr> {
        self.next_char();
        let mut res = String::new();
        while let Some(mut c) = self.cur {
            self.next_char();
            if c == '"' {
                return Ok(Token::Literal(res));
            } else if c == '\\' {
                c = match self.cur {
                    Some('n') => '\n',
                    Some('r') => '\r',
                    Some('t') => '\t',
                    Some(c) => c,
                    None => return Err(LexerErr::UnclosedLit),
                }
            }

            res.push(c);
        }
        Err(LexerErr::UnclosedLit)
    }

    /// Skips whitespace characters
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.cur {
            if !c.is_whitespace() {
                break;
            }
            self.next_char();
        }
    }
}
