use std::fmt::Formatter;

use iced::alignment::Horizontal;
use iced::widget::{column, container, pick_list, row};
use iced::{Element, Length, Sandbox};
use iced_native::widget::{button, horizontal_rule};
use iced_native::Theme;

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
        ImageGrayscale {
            image_list: vec![],
            file_options: vec![],
            keep_original_files: Some(FileOptions::KeepOriginalFiles),
        }
    }

    fn title(&self) -> String {
        String::from("Easy Image Grayscale")
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
        );

        let pick_list_con = container(pick_list)
            .width(Length::Fill)
            .height(Length::Shrink)
            .align_x(Horizontal::Right);

        let scan_folder_con = container(button("Scan Folder"))
            .width(Length::Fill)
            .height(Length::Shrink)
            .align_x(Horizontal::Left);

        let top_row = row![scan_folder_con, pick_list_con].padding(10.0);

        column![top_row, horizontal_rule(5.0)].into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
