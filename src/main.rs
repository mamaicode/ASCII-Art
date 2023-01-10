use image::imageops::FilterType;
use image::{DynamicImage, ImageFormat, GenericImageView};
use std::fs::File;
use std::io::{BufReader, Write};

fn main() {
    // Read the image file
    // To do: take file path as argument
    let image_file = File::open("input.jpg").unwrap();
    let image = image::load(BufReader::new(image_file), ImageFormat::Jpeg).unwrap();

    // Convert the image to grayscale
    let grayscale_image = image.grayscale();

    // Create an ASCII art representation of the image
    let ascii_art = image_to_ascii(&grayscale_image);

    // Write the ASCII art to a file
    let mut output_file = File::create("output.txt").unwrap();
    write!(output_file, "{}", ascii_art).unwrap();
}

fn image_to_ascii(image: &DynamicImage) -> String {

    let new_image = image.resize( 500, 30, FilterType::Triangle);


    let (width, height) = new_image.dimensions();
    let mut ascii_art = String::new();

    // Iterate over the pixels in the image
    for y in 0..height {
        for x in 0..width {
            let pixel = new_image.get_pixel(x, y);
            let brightness = (0.299 * f64::from(pixel[0])
                + 0.587 * f64::from(pixel[1])
                + 0.114 * f64::from(pixel[2])) as u8;

            let ascii_char = match brightness {
                0..=50 => '#',
                51..=100 => '@',
                101..=150 => '%',
                151..=200 => '*',
                201..=255 => ' ',
            };

            ascii_art.push(ascii_char);
        }
        ascii_art.push('\n');
    }

    ascii_art
}


