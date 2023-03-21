use std::fmt::Formatter;
use std::path::PathBuf;

use iced::alignment::Vertical;
use iced::widget::{column, container, horizontal_rule, image, row, scrollable, text, Column};
use iced::{theme, Alignment, Element, Length, Sandbox, Theme};
use iced_aw::native::Split;
use iced_aw::split::Axis;
use iced_native::widget::scrollable::Properties;
use iced_native::widget::{button, svg, vertical_space, Button};

use crate::processing::images::{current_directory, find_images, images_to_bytes, process_images};

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
    ver_divider_position: Option<u16>,
    input_value: f32,
    max_input_value: f32,
}

#[derive(Debug, Clone)]
pub enum Message {
    NumInputChanged(f32),
    PickListChanged(FileOptions),
    PressedClearList,
    PressedProcess,
    PressedScanFolder,
    Resized(u16),
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
            current_path: PathBuf::new(),
            current_scroll_offset: scrollable::RelativeOffset::START,
            file_options: vec![],
            image_list: vec![],
            input_value: 100.0,
            keep_original_files: Some(FileOptions::KeepOriginalFiles),
            max_input_value: 100.0,
            raw_images: vec![],
            scrollbar_margin: 0,
            scrollbar_width: 12,
            scroller_width: 10,
            ver_divider_position: Some(500),
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
                self.ver_divider_position = Some(position);
            }
            Message::PressedProcess => {
                if self.image_list.is_empty() {
                } else {
                    process_images(self.image_list.clone());
                    self.image_list.clear();
                    self.raw_images.clear();
                }
            }
            Message::NumInputChanged(quality) => self.input_value = quality,
            _ => {}
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let logo = include_bytes!("cuddlyferris_gray.svg");
        let logo_handle = svg::Handle::from_memory(logo.as_slice());
        let svg = svg(logo_handle)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme::Svg::Default);

        // let pick_list = pick_list(
        //     &FileOptions::ALL[..],
        //     self.keep_original_files,
        //     Message::PickListChanged,
        // );

        let mut process_button = Button::new("Process");

        if self.image_list.len() > 1 {
            process_button = button("Process").on_press(Message::PressedProcess)
        }

        let settings_column = Column::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            // .push(row![pick_list.width(Length::Fill)].padding(5.0))
            .push(horizontal_rule(5.0))
            .push(
                row![
                    button("Scan Folder").on_press(Message::PressedScanFolder),
                    button("Clear List").on_press(Message::PressedClearList),
                ]
                .align_items(Alignment::Center)
                .spacing(20.0)
                .padding(10.0)
                .height(Length::Shrink),
            )
            .push(horizontal_rule(5.0))
            // .push(
            //     container(
            //         row![
            //             text("Desired Quality:"),
            //             Space::with_width(5.0),
            //             NumberInput::new(
            //                 self.input_value,
            //                 self.max_input_value,
            //                 Message::NumInputChanged,
            //             )
            //             .step(1.0)
            //         ]
            //         .align_items(Alignment::Center)
            //         .width(Length::Fill)
            //         .height(Length::Shrink),
            //     )
            //     .align_y(Vertical::Top)
            //     // .height(Length::FillPortion(5))
            //     .padding(10.0),
            // )
            .push(
                container(column![svg].width(Length::Fill))
                    .align_y(Vertical::Center)
                    .height(Length::FillPortion(7))
                    .padding(15.0),
            )
            .push(text(format!("Files to Process: {}", self.image_list.len())))
            .push(vertical_space(10.0))
            .push(
                container(process_button)
                    .align_y(Vertical::Bottom)
                    .height(Length::Shrink)
                    .padding(10.0),
            );

        let mut image_column = Column::new()
            .height(Length::Shrink)
            .width(Length::Fill)
            .align_items(Alignment::Center);

        for image_bytes in &self.raw_images {
            let my_handle = image::Handle::from_memory(image_bytes.to_owned());
            image_column = image_column.push(image::viewer(my_handle).width(Length::Fill));
        }

        // for image_bytes in self.raw_images.chunks(2) {
        //     let mut new_row = Row::new();
        //     for elem in image_bytes.iter() {
        //         let my_handle = image::Handle::from_memory(elem.clone());
        //         new_row = new_row.push(image::viewer(my_handle));
        //     }
        //
        //     image_column = image_column.push(new_row);
        // }

        let scrollable_column = scrollable(image_column)
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
            self.ver_divider_position,
            Axis::Vertical,
            Message::Resized,
        )
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
