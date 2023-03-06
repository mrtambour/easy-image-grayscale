use std::path::PathBuf;
use std::{env, fs};

pub fn current_directory() -> PathBuf {
    env::current_dir().expect("error getting current directory")
}

pub fn find_images() -> Vec<String> {
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

pub fn process_images(archive_images: Vec<String>) {
    for archive_image in archive_images {
        let mut archive_name = archive_image.clone();
        let image_format = archive_name[archive_name.len() - 4..].to_string();
        archive_name.truncate(archive_name.len() - 4);
        let final_archive_name = format!("{archive_name}_gryscl{image_format}");

        if std::path::Path::new(&final_archive_name).exists() || archive_name.contains("_gryscl") {
            println!("file exists, skipping");
            continue;
        }
        let grayscale_image = image::open(archive_image)
            .expect("failed to convert image to grayscale")
            .grayscale();
        println!("new image name {final_archive_name}");
        grayscale_image
            .save(final_archive_name)
            .expect("error saving new grayscale image");
    }
}
