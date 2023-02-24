use std::path::PathBuf;
use std::{env, fs};

fn current_directory() -> PathBuf {
    env::current_dir().expect("error getting current directory")
}

fn find_images() -> Vec<String> {
    let current_directory = current_directory();
    let mut images = Vec::new();
    for entry in
        fs::read_dir(current_directory).expect("error occurred while trying to scan directory")
    {
        let file_name = entry
            .expect("error while scanning directory")
            .file_name()
            .into_string()
            .expect("error getting file name");

        if file_name.ends_with(".jpg") | file_name.ends_with("jpeg") | file_name.ends_with(".png") {
            println!("found image: {}", file_name);
            images.push(file_name);
        }
    }
    images
}

fn main() {
    println!("Easy Image Grayscale");
    let images = find_images();
}
