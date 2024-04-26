use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

pub enum Writer {
    File(BufWriter<File>),
    String(String),
    Stdout,
}

impl Writer {
    pub fn write(&mut self, content: char) -> io::Result<()> {
        match self {
            Writer::File(writer) => writer.write_all(&[content as u8]),
            Writer::String(string) => Ok(string.push(content)),
            Writer::Stdout => Ok(print!("{content}")),
        }
    }

    pub fn write_str(&mut self, content: &str) -> io::Result<()> {
        match self {
            Writer::File(writer) => writer.write_all(content.as_bytes()),
            Writer::String(string) => Ok(string.push_str(content)),
            Writer::Stdout => Ok(print!("{content}")),
        }
    }
}
