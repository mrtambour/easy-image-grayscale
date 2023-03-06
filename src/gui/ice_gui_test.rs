use std::fmt::Formatter;

use iced::widget::{column, container, pick_list, scrollable, vertical_space};
use iced::{Alignment, Element, Length, Sandbox, Settings};

#[derive(Default)]
pub struct ImageGrayscale {
    image_list: Vec<String>,
    file_options: Vec<FileOptions>,
    keep_original_files: Option<FileOptions>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed,
    PickListChanged(FileOptions),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileOptions {
    KeepOriginalFiles,
    RemoveOriginalFiles,
}

impl FileOptions {
    const ALL: [FileOptions; 2] = [
        FileOptions::KeepOriginalFiles,
        FileOptions::RemoveOriginalFiles,
    ];
}

impl Default for FileOptions {
    fn default() -> Self {
        FileOptions::KeepOriginalFiles
    }
}

impl std::fmt::Display for FileOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FileOptions::KeepOriginalFiles => "Keep Original Files",
                FileOptions::RemoveOriginalFiles => "Remove Original Files",
            }
        )
    }
}

impl Sandbox for ImageGrayscale {
    type Message = Message;

    fn new() -> Self {
        ImageGrayscale::default()
    }

    fn title(&self) -> String {
        String::from("Easy Image grayscale")
    }

    fn update(&mut self, message: Self::Message) {
        if let Message::PickListChanged(FileOptions) = message {
            self.keep_original_files = Some(FileOptions);
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let pick_list = pick_list(
            &FileOptions::ALL[..],
            self.keep_original_files,
            Message::PickListChanged,
        )
        .placeholder("choose an option");

        let content = column![pick_list]
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .spacing(10);

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
