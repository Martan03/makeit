use crate::err::lexer_err::LexerErr;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Colon,
    Question,
    NullCheck,
    Ident(String),
    Literal(String),
    End,
}

#[derive(Debug)]
pub struct Lexer<'a, I>
where
    I: Iterator<Item = char>,
{
    text: &'a mut I,
    cur: Option<char>,
}

impl<'a, I> Lexer<'a, I>
where
    I: Iterator<Item = char>,
{
    /// Creates new [`Lexer`]
    pub fn new(text: &'a mut I) -> Self {
        let c = text.next();
        Self {
            text: text.into(),
            cur: c,
        }
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
                    self.next_char();
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
            }

            res.push(c);
            self.next_char();
        }
        Err(LexerErr::UnclosedLit)
    }

    /// Gets next char from the text
    fn next_char(&mut self) {
        self.cur = self.text.next();
    }
}
