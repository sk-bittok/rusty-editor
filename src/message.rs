use std::{path::PathBuf, sync::Arc};

use crate::error::Error;

use iced::{highlighter::Theme, widget::text_editor::Action};

#[derive(Debug, Clone)]
pub enum Message {
    ActionPerformed(Action),
    NewFile,
    OpenFile,
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    SaveFile,
    FileSaved(Result<PathBuf, Error>),
    ThemeSelected(Theme),
    WordWrapToggled(bool),
}
