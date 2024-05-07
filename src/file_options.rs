use serde::{Deserialize, Serialize};

/// Represents file action
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum FileAction {
    Copy,
    #[default]
    Make,
    Ignore,
}

impl FileAction {
    /// Checks if action is make
    fn is_make(&self) -> bool {
        matches!(self, FileAction::Make)
    }
}

/// File options struct
#[derive(Debug, Serialize, Deserialize)]
pub struct FileOptions {
    #[serde(default, skip_serializing_if = "FileAction::is_make")]
    pub action: FileAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
