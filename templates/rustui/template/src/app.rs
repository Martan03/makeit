use termint::{
    enums::Modifier,
    geometry::{Constraint, TextAlign},
    term::{
        Action, Application, Frame,
        backend::{Event, KeyCode, KeyEvent},
    },
    widgets::{Element, Layout, Spacer, ToSpan},
};

#[derive(Debug, Clone)]
pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    /// Creates a small screen message.
    pub fn small_screen<M: Clone>() -> Element<M> {
        let mut layout = Layout::vertical().center();
        layout.push(
            "Terminal too small!"
                .modifier(Modifier::BOLD)
                .align(TextAlign::Center),
            Constraint::Min(0),
        );
        layout.push(
            "You have to increase terminal size".align(TextAlign::Center),
            Constraint::Min(0),
        );
        layout.into()
    }
}

impl Application for App {
    type Message = ();

    fn view(&self, _frame: &Frame) -> Element<Self::Message> {
        // todo!()
        Spacer::new().into()
    }

    fn event(&mut self, event: Event) -> Action {
        match event {
            Event::Key(key) => self.key_handler(key),
            _ => Action::NONE,
        }
    }
}

impl App {
    fn key_handler(&mut self, event: KeyEvent) -> Action {
        match event.code {
            KeyCode::Char('q') | KeyCode::Esc => return Action::QUIT,
            _ => Action::NONE,
        }
    }
}
