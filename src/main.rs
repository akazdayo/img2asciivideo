mod ascii;
mod video;

use image::io::Reader as ImageReader;
use image::GenericImageView;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use video::Video;

fn main() {
    let video = Video::new(30, "test.mp4".to_string());
    let image = read_image(2);
    let text = ascii::convert_ascii(image);
    write(text.as_bytes()).unwrap();
}

fn read_image(resize: u32) -> image::DynamicImage {
    // 画像を読み込む
    let path = readline();
    let img = ImageReader::open(path).unwrap().decode().unwrap();

    // 画像を1/10サイズにリサイズ
    let (width, height) = img.dimensions();
    let small_img = img.resize(
        width / resize,
        height / resize,
        image::imageops::FilterType::Nearest,
    );

    return small_img;
}

fn write(text: &[u8]) -> Result<()> {
    let mut file = File::create("./images/output.txt")?;
    file.write_all(text)?;
    Ok(())
}

fn readline() -> String {
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).unwrap();
    let answer = word.trim().to_string();
    return answer;
}
