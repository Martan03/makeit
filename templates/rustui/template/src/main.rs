use std::process::ExitCode;

use pareg::Pareg;
use termint::{enums::Color, style::Stylize, term::Term};

use crate::{
    app::App,
    args::{Action, Args},
    error::Error,
};

pub mod app;
pub mod args;
pub mod error;

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{} {}", "Error:".fg(Color::Red), e);
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Error> {
    let args = Args::parse(Pareg::args())?;
    match args.action {
        Action::Run => run_app()?,
        Action::Help => Args::help(),
        Action::Version => Args::version(),
    }
    Ok(())
}

fn run_app() -> Result<(), Error> {
    let mut app = App::new();
    Term::default()
        .setup()?
        .small_screen(App::small_screen())
        .run(&mut app)?;
    Ok(())
}
