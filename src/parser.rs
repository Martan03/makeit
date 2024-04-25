use crate::{
    err::lexer_err::LexerErr,
    lexer::{Lexer, Token},
};

pub struct Parser<'a, I>
where
    I: Iterator<Item = char>,
{
    text: &'a mut I,
}

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = char>,
{
    /// Creates new [`Parser`]
    pub fn new(text: &'a mut I) -> Self {
        Self { text }
    }

    /// Parses given text
    pub fn parse(&mut self) -> Result<(), String> {
        while let Some(c) = self.text.next() {
            if c == '{' {
                self.check_opening().map_err(|e| e.to_string())?;
            } else {
                print!("{c}");
            }
        }
        Ok(())
    }

    fn check_opening(&mut self) -> Result<(), LexerErr> {
        let Some(c) = self.text.next() else {
            print!("{{");
            return Ok(());
        };

        if c != '{' {
            print!("{{{c}");
            return Ok(());
        }

        println!();
        let mut lexer = Lexer::new(self.text);
        let mut token = lexer.next()?;
        while token != Token::End {
            println!("{:?}", token);
            token = lexer.next()?;
        }

        Ok(())
    }
}
