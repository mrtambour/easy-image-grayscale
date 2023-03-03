use eframe::egui::{Color32, Context, RichText};
use eframe::{egui, Frame};

#[derive(Default)]
pub struct ImageGrayscale {}

impl ImageGrayscale {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for ImageGrayscale {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                |ui| {
                    ui.label(
                        RichText::new("Easy Image Grayscale")
                            .size(60.0)
                            .color(Color32::WHITE)
                            .strong(),
                    );
                },
            );
        });
    }
}
