use crate::gui::iced_gui::ImageGrayscale;

use iced::{Application, Settings};

mod gui;
mod processing;

fn main() {
    println!("Easy Image Grayscale");
    ImageGrayscale::run(Settings::default()).unwrap();
}
