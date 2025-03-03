use std::ffi;
use std::path::{Path, PathBuf};

use iced::Alignment::Center;
use iced::highlighter;
use iced::widget::{column, horizontal_space, pick_list, row, text, text_editor, toggler};
use iced::{Element, Fill, Task, Theme};

use crate::{message::Message, utils};

#[derive(Debug)]
pub struct Editor {
    file: Option<PathBuf>,
    content: text_editor::Content,
    theme: highlighter::Theme,
    is_loading: bool,
    is_dirty: bool,
    word_wrap: bool,
}

impl Editor {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                file: None,
                content: text_editor::Content::new(),
                theme: highlighter::Theme::Base16Ocean,
                is_loading: true,
                word_wrap: true,
                is_dirty: false,
            },
            Task::batch([
                Task::perform(
                    utils::load_file(format!("{}/src/bin/main.rs", env!("CARGO_MANIFEST_DIR"))),
                    Message::FileOpened,
                ),
                iced::widget::focus_next(),
            ]),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ActionPerformed(action) => {
                self.is_dirty = self.is_dirty || action.is_edit();
                self.content.perform(action);

                Task::none()
            }
            Message::NewFile => {
                if !self.is_loading {
                    self.content = text_editor::Content::new();
                    self.file = None;
                }
                Task::none()
            }

            Message::OpenFile => {
                if self.is_loading {
                    Task::none()
                } else {
                    self.is_loading = true;
                    Task::perform(utils::open_file(), Message::FileOpened)
                }
            }
            Message::FileOpened(result) => {
                self.is_loading = false;
                self.is_dirty = false;

                if let Ok((path, contents)) = result {
                    self.file = Some(path);

                    self.content = text_editor::Content::with_text(&contents);
                }

                Task::none()
            }
            Message::SaveFile => {
                if self.is_loading {
                    Task::none()
                } else {
                    self.is_loading = true;

                    let mut text = self.content.text();

                    if let Some(ending) = self.content.selection() {
                        if !text.ends_with(ending.as_str()) {
                            text.push_str(ending.as_str());
                        }
                    }

                    Task::perform(
                        utils::save_file(self.file.clone(), text),
                        Message::FileSaved,
                    )
                }
            }
            Message::FileSaved(result) => {
                self.is_loading = false;

                if let Ok(path) = result {
                    self.file = Some(path);
                    self.is_dirty = false;
                }

                Task::none()
            }
            Message::ThemeSelected(theme) => {
                self.theme = theme;
                Task::none()
            }

            Message::WordWrapToggled(toggled) => {
                self.word_wrap = toggled;

                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let controls = row![
            utils::action(utils::new_icon(), "New file", Some(Message::NewFile)),
            utils::action(
                utils::open_icon(),
                "Open file",
                (!self.is_loading).then_some(Message::OpenFile)
            ),
            utils::action(
                utils::save_icon(),
                "Save file",
                self.is_dirty.then_some(Message::SaveFile)
            ),
            horizontal_space(),
            toggler(self.word_wrap)
                .label("Word wrap")
                .on_toggle(Message::WordWrapToggled),
            pick_list(
                highlighter::Theme::ALL,
                Some(self.theme),
                Message::ThemeSelected
            )
            .text_size(14)
            .padding([5, 10])
        ]
        .spacing(10)
        .align_y(Center);

        let status = row![
            text(if let Some(filepath) = &self.file {
                let components: Vec<_> = filepath.components().map(|c| c.as_os_str()).collect();

                if components.len() > 3 {
                    let truncated_path: PathBuf =
                        components[components.len() - 3..].iter().collect();
                    format!("../{}", truncated_path.display())
                } else {
                    filepath.display().to_string()
                }
            } else {
                String::from("New File")
            }),
            horizontal_space(),
            text({
                let (line, column) = self.content.cursor_position();

                format!("{}:{}", line + 1, column + 1)
            })
        ]
        .spacing(10);

        let contents = text_editor(&self.content)
            .height(Fill)
            .on_action(Message::ActionPerformed)
            .wrapping(if self.word_wrap {
                text::Wrapping::Word
            } else {
                text::Wrapping::None
            })
            .highlight(
                self.file
                    .as_deref()
                    .and_then(Path::extension)
                    .and_then(ffi::OsStr::to_str)
                    .unwrap_or("rs"),
                self.theme,
            )
            .key_binding(|key_press| match key_press.key.as_ref() {
                iced::keyboard::Key::Character("s") if key_press.modifiers.control() => {
                    Some(text_editor::Binding::Custom(Message::SaveFile))
                }

                _ => text_editor::Binding::from_key_press(key_press),
            });

        column![controls, contents, status]
            .spacing(10)
            .padding(10)
            .into()
    }

    pub fn theme(&self) -> Theme {
        if self.theme.is_dark() {
            Theme::Dark
        } else {
            Theme::Light
        }
    }
}
