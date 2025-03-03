use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use iced::{
    Element, Font,
    widget::{button, container, text, tooltip},
};

use crate::error::{Error, Result};

pub(crate) async fn load_file(path: impl Into<PathBuf>) -> Result<(PathBuf, Arc<String>)> {
    let path = path.into();

    let contents = tokio::fs::read_to_string(&path).await.map(Arc::new)?;

    Ok((path, contents))
}

pub(crate) async fn open_file() -> Result<(PathBuf, Arc<String>)> {
    let file = rfd::AsyncFileDialog::new()
        .set_title("Open a text file")
        .pick_file()
        .await
        .ok_or(Error::DialogueClosed)?;

    load_file(file).await
}

pub(crate) async fn save_file(path: Option<PathBuf>, contents: String) -> Result<PathBuf> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
            .save_file()
            .await
            .as_ref()
            .map(rfd::FileHandle::path)
            .map(Path::to_owned)
            .ok_or(Error::DialogueClosed)?
    };

    tokio::fs::write(&path, contents).await?;

    Ok(path)
}

pub(crate) fn action<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    label: &'a str,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let action = button(container(content).center_x(30));

    if let Some(on_press) = on_press {
        tooltip(
            action.on_press(on_press),
            label,
            tooltip::Position::FollowCursor,
        )
        .style(container::rounded_box)
        .into()
    } else {
        action.style(button::secondary).into()
    }
}

fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("editor-icons");

    text(codepoint).font(ICON_FONT).into()
}

pub(crate) fn new_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e800}')
}

pub(crate) fn open_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0f115}')
}

pub(crate) fn save_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e801}')
}
