use std::path::PathBuf;
use std::{env, fs};

use eframe::egui;
use iced::{Alignment, Element, Length, Sandbox, Settings};

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
        "My egui App",
        options,
        Box::new(|cc| Box::new(ImageGrayscale::default())),
    )
    .expect("unable to start window");
}
