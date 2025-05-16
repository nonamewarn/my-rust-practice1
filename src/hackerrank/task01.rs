use image::io::Reader as ImageReader;
use std::path::Path;

fn main() {
    let img = ImageReader::open("assets/logo.png")
        .expect("Не вдалося відкрити файл")
        .decode()
        .expect("Не вдалося декодувати зображення");

    println!("Розмір зображення: {:?}", img.dimensions());
}
