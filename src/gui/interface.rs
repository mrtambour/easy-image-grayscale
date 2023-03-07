use std::fs;
use std::ops::RangeTo;

use eframe::egui::WidgetText::RichText;
use eframe::egui::{Align, Color32, Context, Direction, ScrollArea, Ui};
use eframe::epaint::TextureId;
use eframe::{egui, Frame};
use egui_extras::image::load_image_bytes;
use egui_extras::RetainedImage;
use iced::widget::image;
use image::EncodableLayout;

use crate::processing::image_handling::find_images;

pub struct ImageGrayscale {
    image_list: Vec<String>,
    file_options: Vec<String>,
    keep_original_files: String,
    raw_images: Vec<Vec<u8>>,
    retained_images: Vec<RetainedImage>,
}

impl ImageGrayscale {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl Default for ImageGrayscale {
    fn default() -> Self {
        Self {
            image_list: vec![],
            keep_original_files: "Keep Original Files".to_string(),
            file_options: vec![
                "Keep Original Files".to_string(),
                "Remove Original Files".to_string(),
            ],
            raw_images: vec![],
            retained_images: vec![],
        }
    }
}

impl eframe::App for ImageGrayscale {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::TopBottomPanel::top("top_panel")
            .min_height(30.0)
            .show(ctx, |ui| {
                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::Center).with_cross_justify(false),
                    |ui| {
                        egui::ComboBox::from_id_source("keep_file_option")
                            .selected_text(self.keep_original_files.clone())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.keep_original_files,
                                    self.file_options.first().unwrap().to_string(),
                                    self.file_options.first().unwrap().to_string(),
                                );
                                ui.selectable_value(
                                    &mut self.keep_original_files,
                                    self.file_options.get(1).unwrap().to_string(),
                                    self.file_options.get(1).unwrap().to_string(),
                                );
                            });
                    },
                );
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical()
                .max_height(ui.available_height() - 40.0)
                .id_source("main_scroll")
                .auto_shrink([false, true])
                .min_scrolled_height(50.0)
                .stick_to_right(true)
                .show_rows(ui, 35.0, self.image_list.len(), |ui, range| {
                    for row in range {
                        let image_name = self.image_list.get(row).unwrap();
                        println!("trying to get {}", image_name);
                        println!("length of raw images {}", self.raw_images.len());

                        let finale = ui.push_id(row, |ui| {
                            egui::Grid::new("grid")
                                .striped(true)
                                .num_columns(3)
                                .striped(true)
                                .spacing(egui::Vec2::new(10.0, 0.0))
                                .show(ui, |ui| {
                                    ui.add(
                                        egui::ImageButton::new(
                                            self.retained_images.get(row).unwrap().texture_id(ctx),
                                            self.retained_images.get(row).unwrap().size_vec2(),
                                        )
                                        .selected(false),
                                    )
                                });
                        });
                    }
                });
        });

        egui::TopBottomPanel::bottom("bottom_panel")
            .min_height(40.0)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                    if ui.add(egui::Button::new("Scan Folder")).clicked() {
                        self.image_list.clear();
                        self.image_list = find_images();
                        println!("found images");

                        for image in &self.image_list {
                            println!("for images {}", image);
                            self.retained_images.push(
                                RetainedImage::from_image_bytes(
                                    "test",
                                    fs::read(image)
                                        .expect("error reading image file")
                                        .as_bytes(),
                                )
                                .unwrap(),
                            )
                        }

                        println!("{}", &self.keep_original_files);
                    };
                    if ui.add(egui::Button::new("Remove Selected")).clicked() {
                        println!("{}", &self.keep_original_files);
                    };
                    if ui.add(egui::Button::new("Clear List")).clicked() {
                        println!("{}", &self.keep_original_files);
                    };
                    if ui.add(egui::Button::new("Process Images")).clicked() {
                        println!("{}", &self.keep_original_files);
                    };
                });
            });
    }
}
