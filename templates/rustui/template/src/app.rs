use std::{
    io::{Write, stdout},
    time::Duration,
};

use crossterm::{
    event::{Event, KeyEvent, poll},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    /// Runs the app - does prep. work before main loop and then clean up
    pub fn run(&mut self) -> Result<(), Error> {
        enable_raw_mode()?;
        // Swaps print buffer, clears screen and hides cursor
        print!("\x1b[?1049h\x1b[2J\x1b[?25l");
        _ = stdout().flush();

        let res = self.main_loop();

        // Restores screen
        print!("\x1b[?1049l\x1b[?25h");
        _ = stdout().flush();
        disable_raw_mode()?;

        match res {
            Err(Error::Exit) => Ok(()),
            _ => res,
        }
    }

    fn main_loop(&mut self) -> Result<(), Error> {
        self.render()?;
        loop {
            if poll(Duration::from_millis(100))? {
                self.event()?;
            }
        }
    }

    fn render(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn event(&mut self) -> Result<(), Error> {
        match crossterm::event::read()? {
            Event::Key(e) => self.key_handler(e),
            Event::Resize(_, _) => self.render(),
            _ => Ok(()),
        }
    }

    fn key_handler(&mut self, _event: KeyEvent) -> Result<(), Error> {
        todo!()
    }
}
