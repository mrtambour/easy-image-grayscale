use std::fmt::Formatter;
use std::path::PathBuf;

use iced::alignment::Horizontal;
use iced::widget::{column, container, pick_list, row, scrollable, Scrollable};
use iced::{Application, Element, Length, Sandbox};
use iced_native::image::Handle;
use iced_native::widget::scrollable::{Properties, State};
use iced_native::widget::{button, horizontal_rule, Column, Image, Row};
use iced_native::{Alignment, Renderer, Theme, Widget};

use crate::processing::image_handling::{current_directory, find_images};

pub struct ImageGrayscale {
    image_list: Vec<String>,
    file_options: Vec<FileOptions>,
    keep_original_files: Option<FileOptions>,
    current_path: PathBuf,
    scrollbar_width: u16,
    scrollbar_margin: u16,
    scroller_width: u16,
    current_scroll_offset: scrollable::RelativeOffset,
}

#[derive(Debug, Clone)]
pub enum Message {
    PressedScanFolder,
    PickListChanged(FileOptions),
    Scrolled(scrollable::RelativeOffset),
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
            current_path: PathBuf::new(),
            scrollbar_width: 10,
            scrollbar_margin: 0,
            scroller_width: 10,
            current_scroll_offset: scrollable::RelativeOffset::START,
        }
    }

    fn title(&self) -> String {
        String::from("Easy Image Grayscale")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::PickListChanged(FileOptions) => {
                self.keep_original_files = Some(FileOptions);
            }
            Message::Scrolled(offset) => self.current_scroll_offset = offset,
            Message::PressedScanFolder => {
                self.current_path = current_directory();
                self.image_list = find_images();
            }
            _ => {}
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

        let scan_folder_con = container(button("Process Images"))
            .width(Length::Fill)
            .height(Length::Shrink)
            .align_x(Horizontal::Left);

        let top_row =
            column![row![scan_folder_con, pick_list_con].padding(10.0),].width(Length::Fill);

        let mut image_column = Column::new()
            .height(Length::Shrink)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .spacing(50);

        for item in &self.image_list {
            image_column = image_column.push(Image::<Handle>::new(item));
        }

        let mut scrollable_collumn = scrollable(image_column)
            .height(Length::Fill)
            .vertical_scroll(
                Properties::new()
                    .width(self.scrollbar_width)
                    .margin(self.scrollbar_margin)
                    .scroller_width(self.scroller_width),
            )
            .on_scroll(Message::Scrolled);

        let bottom_row = row![
            button("Scan Folder").on_press(Message::PressedScanFolder),
            button("Remove Selected"),
            button("Clear List"),
        ]
        .width(Length::Fill)
        .spacing(20.0)
        .align_items(Alignment::Center)
        .padding(10.0);

        column![
            top_row,
            horizontal_rule(5.0),
            scrollable_collumn,
            horizontal_rule(5.0),
            bottom_row,
        ]
        .width(Length::Fill)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
