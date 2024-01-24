use image::GenericImageView;
use image::Pixel;

pub fn convert_ascii(img: image::DynamicImage) -> String {
    let ascii = "■⠇⠋⠓⠣⠍⠕⠥⠙⠩⠱⠎⠖⠦⠚⠪⠲⠜⠬⠴⠸⠃⠅⠉⠑⠡⠆⠊⠒⠢⠌⠔⠤⠘⠨⠰⠁⠂⠄⠈⠐⠠ ";
    let ascii_chars: Vec<char> = ascii.chars().collect(); // 文字列を文字のベクトルに変換

    // グレースケールに変換する
    let gray = img.grayscale();
    let mut output = String::new();

    // 画像を2次元配列に変換
    let gray2d: Vec<Vec<u8>> = gray
        .pixels()
        .map(|p| p.2.to_luma().0[0]) // p.2 で Rgba<u8> を取り出す
        .collect::<Vec<_>>()
        .chunks(gray.width() as usize)
        .map(|row| row.to_vec())
        .collect();

    for gray2 in gray2d {
        output += "\n";
        for dark in gray2 {
            let ascii_index = (dark as f32 / 255.0 * (ascii_chars.len() - 1) as f32) as usize;
            output.push(ascii_chars[ascii_index]); // 文字のベクトルをインデックス操作
        }
    }
    //println!("{}", output);
    //write(output.as_bytes()).unwrap();
    return output;
}
