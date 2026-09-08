/// CLI arguments available actions.
///
/// Default action is running the app.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Action {
    #[default]
    Run,
    Help,
    Version,
}
