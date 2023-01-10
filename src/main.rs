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

    // // Write the ASCII art to a file
    let mut output_file = File::create("output.txt").unwrap();
    write!(output_file, "{}", ascii_art).unwrap();   
}

fn image_to_ascii(image: &DynamicImage) -> String {
    let (width, height) = image.dimensions();
    let block_width = 10;
    let block_height = 10;
    let mut ascii_art = String::new();

    // Iterate over the blocks of pixels in the image
    for y_block in 0..(height/block_height) {
        for x_block in 0..(width/block_width) {
            let mut red_sum = 0;
            let mut green_sum = 0;
            let mut blue_sum = 0;
            let mut total_pixels = 0;

            // Sum the color values of all pixels in the block
            for y in (y_block*block_height)..((y_block+1)*block_height) {
                for x in (x_block*block_width)..((x_block+1)*block_width) {
                    let pixel = image.get_pixel(x, y);
                    red_sum += pixel[0] as u32;
                    green_sum += pixel[1] as u32;
                    blue_sum += pixel[2] as u32;
                    total_pixels += 1;
                }
            }

            // Calculate the average color of the block
            let avg_red = red_sum / total_pixels;
            let avg_green = green_sum / total_pixels;
            let avg_blue = blue_sum / total_pixels;

            // Use the average color to map the block to an ASCII character
            let brightness = (0.299 * f64::from(avg_red)
                + 0.587 * f64::from(avg_green)
                + 0.114 * f64::from(avg_blue)) as u8;

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



