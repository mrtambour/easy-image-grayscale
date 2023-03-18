use crate::gui::iced_gui::ImageGrayscale;

use iced::{window, Application, Settings};

mod gui;
mod processing;

fn main() {
    println!("Easy Image Grayscale");
    let settings = Settings {
        window: window::Settings {
            size: (700, 500),
            resizable: false,
            decorations: true,

            ..Default::default()
        },
        ..Default::default()
    };
    ImageGrayscale::run(settings).unwrap();
}
