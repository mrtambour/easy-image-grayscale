#![windows_subsystem = "windows"]

use iced::{window, Application, Settings};

use crate::gui::interface::ImageGrayscale;

mod gui;
mod processing;

fn main() {
    println!("Easy Image Grayscale");
    let settings = Settings {
        window: window::Settings {
            size: (800, 500),
            resizable: true,
            decorations: true,

            ..Default::default()
        },
        ..Default::default()
    };
    ImageGrayscale::run(settings).unwrap();
}
