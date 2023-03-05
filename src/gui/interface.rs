use std::ops::RangeTo;

use eframe::egui::{Align, Color32, Context, RichText, ScrollArea};
use eframe::{egui, Frame};

pub struct ImageGrayscale {
    image_list: Vec<RangeTo<i32>>,
}

impl ImageGrayscale {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl Default for ImageGrayscale {
    fn default() -> Self {
        Self {
            image_list: vec![..100],
        }
    }
}

impl eframe::App for ImageGrayscale {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical()
                .id_source("main_scroll")
                .auto_shrink([false, false])
                .max_height(350.0)
                .min_scrolled_height(100.0)
                .stick_to_right(true)
                .show_rows(ui, 35.0, 100, |ui, row_range| {
                    for row in row_range {
                        ui.push_id(row, |ui| {
                            egui::Grid::new("Images")
                                .striped(true)
                                .num_columns(1)
                                .striped(true)
                                .spacing(egui::Vec2::new(10.0, 0.0))
                                .show(ui, |ui| {
                                    ui.add_sized(
                                        [25.0, 35.0],
                                        egui::Label::new(
                                            egui::RichText::new("IMAGE")
                                                .color(egui::Color32::from_rgb(255, 255, 255))
                                                .monospace(),
                                        ),
                                    )
                                });
                        });
                    }
                });
        });
    }
}
