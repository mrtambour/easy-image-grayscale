use std::ops::RangeTo;

use eframe::egui::WidgetText::RichText;
use eframe::egui::{Align, Color32, Context, Direction, ScrollArea, Ui};
use eframe::{egui, Frame};

use crate::processing::image_handling::find_images;

pub struct ImageGrayscale {
    image_list: Vec<String>,
    file_options: Vec<String>,
    keep_original_files: String,
}

impl ImageGrayscale {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl Default for ImageGrayscale {
    fn default() -> Self {
        Self {
            image_list: vec!["Click Scan Folder".to_string()],
            keep_original_files: "Keep Original Files".to_string(),
            file_options: vec![
                "Keep Original Files".to_string(),
                "Remove Original Files".to_string(),
            ],
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
                        ui.push_id(row, |ui| {
                            egui::Grid::new("grid")
                                .striped(true)
                                .num_columns(1)
                                .striped(true)
                                .spacing(egui::Vec2::new(10.0, 0.0))
                                .show(ui, |ui| {
                                    if ui
                                        .add_sized(
                                            [ui.available_width() / 1.5, 30.0],
                                            egui::Button::new(
                                                egui::RichText::new(
                                                    self.image_list.get(row).unwrap(),
                                                )
                                                .color(egui::Color32::from_rgb(255, 255, 255))
                                                .monospace(),
                                            ),
                                        )
                                        .clicked()
                                    {
                                        println!("clicked");
                                    }

                                    ui.end_row();
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
