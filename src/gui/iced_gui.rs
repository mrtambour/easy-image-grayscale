use std::fmt::Formatter;
use std::path::PathBuf;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{
    self, column, container, horizontal_rule, image, row, scrollable, text, Column, Text,
};
use iced::{Alignment, Application, Color, Command, Element, Length, Sandbox, Settings, Theme};
use iced_aw::native::{number_input, NumberInput, Split};
use iced_aw::split::Axis;
use iced_aw::Icon::AlignBottom;
use iced_native::widget::scrollable::Properties;
use iced_native::widget::{button, pick_list, vertical_rule, Space};
use iced_native::Widget;

use crate::processing::images::{current_directory, find_images, images_to_bytes};

pub struct ImageGrayscale {
    image_list: Vec<String>,
    file_options: Vec<FileOptions>,
    keep_original_files: Option<FileOptions>,
    raw_images: Vec<Vec<u8>>,
    current_path: PathBuf,
    scrollbar_width: u16,
    scrollbar_margin: u16,
    scroller_width: u16,
    current_scroll_offset: scrollable::RelativeOffset,
    hor_divider_position: Option<u16>,
    input_value: f32,
    max_input_value: f32,
}

#[derive(Debug, Clone)]
pub enum Message {
    PressedScanFolder,
    PressedClearList,
    PickListChanged(FileOptions),
    Scrolled(scrollable::RelativeOffset),
    Resized(u16),
    NumInputChanged(f32),
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
            raw_images: vec![],
            keep_original_files: Some(FileOptions::KeepOriginalFiles),
            current_path: PathBuf::new(),
            scrollbar_width: 10,
            scrollbar_margin: 0,
            scroller_width: 10,
            current_scroll_offset: scrollable::RelativeOffset::START,
            hor_divider_position: Some(400),
            input_value: 50.0,
            max_input_value: 100.0,
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
                self.raw_images = images_to_bytes(self.image_list.to_owned());
            }
            Message::PressedClearList => {
                self.image_list.clear();
                self.current_path.clear();
                self.raw_images.clear();
            }
            Message::Resized(position) => {
                self.hor_divider_position = Some(position);
            }
            Message::NumInputChanged(quality) => self.input_value = quality,
            _ => {}
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let pick_list = pick_list(
            &FileOptions::ALL[..],
            self.keep_original_files,
            Message::PickListChanged,
        );

        let settings_column = Column::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(horizontal_rule(5.0))
            .push(row![pick_list.width(Length::Fill)].padding(5.0))
            .push(horizontal_rule(5.0))
            .push(
                row![
                    button("Scan Folder").on_press(Message::PressedScanFolder),
                    button("Clear List").on_press(Message::PressedClearList),
                ]
                .align_items(Alignment::Center)
                .spacing(20.0)
                .padding(10.0),
            )
            .push(horizontal_rule(5.0))
            .push(
                container(
                    row![
                        text("Desired Quality:"),
                        Space::with_width(5.0),
                        NumberInput::new(
                            self.input_value,
                            self.max_input_value,
                            Message::NumInputChanged,
                        )
                        .step(1.0)
                    ]
                    .align_items(Alignment::Center),
                )
                .align_y(Vertical::Center)
                .height(Length::Fill),
            )
            .push(
                container(button("Process Images").width(Length::Fill))
                    .align_y(Vertical::Bottom)
                    .height(Length::Fill)
                    .padding(10.0),
            );

        let mut image_column = Column::new()
            .height(Length::Shrink)
            .width(Length::Fill)
            .align_items(Alignment::Center);

        for image_bytes in &self.raw_images {
            let my_handle = image::Handle::from_memory(image_bytes.to_owned());
            image_column = image_column.push(image::viewer(my_handle));
        }

        let mut scrollable_column = scrollable(image_column)
            .height(Length::Fill)
            .vertical_scroll(
                Properties::new()
                    .width(self.scrollbar_width)
                    .margin(self.scrollbar_margin)
                    .scroller_width(self.scroller_width),
            )
            .on_scroll(Message::Scrolled);

        Split::new(
            scrollable_column,
            settings_column,
            self.hor_divider_position,
            Axis::Vertical,
            Message::Resized,
        )
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
