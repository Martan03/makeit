use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

/// Writer enum that support uniform writing to file, string and stdout
pub enum Writer<'a> {
    File(BufWriter<File>),
    String(&'a mut String),
    Stdout,
}

impl<'a> Writer<'a> {
    /// Writers given character
    pub fn write(&mut self, content: char) -> io::Result<()> {
        match self {
            Writer::File(writer) => writer.write_all(&[content as u8]),
            Writer::String(string) => Ok(string.push(content)),
            Writer::Stdout => Ok(print!("{content}")),
        }
    }

    /// Writes given string
    pub fn write_str(&mut self, content: &str) -> io::Result<()> {
        match self {
            Writer::File(writer) => writer.write_all(content.as_bytes()),
            Writer::String(string) => Ok(string.push_str(content)),
            Writer::Stdout => Ok(print!("{content}")),
        }
    }
}
