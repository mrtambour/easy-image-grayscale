use eframe::egui;

use crate::gui::interface::ImageGrayscale;

mod gui;
mod processing;

fn main() {
    println!("Easy Image Grayscale");

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Easy Image Grayscale",
        options,
        Box::new(|cc| Box::new(ImageGrayscale::default())),
    )
    .expect("unable to open window");
}
