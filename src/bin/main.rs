use editor::app::Editor;
use iced::Font;

fn main() -> iced::Result {
    iced::application("Rusty-Editor", Editor::update, Editor::view)
        .theme(Editor::theme)
        .font(include_bytes!("../../fonts/icons.ttf").as_slice())
        .default_font(Font::MONOSPACE)
        .run_with(Editor::new)
}
