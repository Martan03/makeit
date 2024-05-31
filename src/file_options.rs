use serde::{Deserialize, Serialize};

/// Represents file action
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum FileAction {
    #[default]
    Copy,
    Make,
    Ignore,
}

impl FileAction {
    /// Checks if action is make
    fn is_copy(&self) -> bool {
        matches!(self, FileAction::Copy)
    }
}

/// File options struct
#[derive(Debug, Serialize, Deserialize)]
pub struct FileOptions {
    #[serde(default, skip_serializing_if = "FileAction::is_copy")]
    pub action: FileAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
