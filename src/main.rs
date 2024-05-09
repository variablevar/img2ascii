use std::{env, fs::File, io::BufReader};

use colorful::RGB;
use image::{
    codecs::gif::GifDecoder, open, AnimationDecoder, DynamicImage, Frame, Frames, GenericImageView,
    Pixel,
};

pub mod utils;
extern crate colorful;

use colorful::Colorful;

// Define a struct to hold image path and scale factor
struct ImageInfo {
    pub image_path: String,
    pub scale_factor: u32,
}
impl ImageInfo {
    // Function to parse command-line arguments and create an ImageInfo struct
    fn new(args: &[String]) -> Result<ImageInfo, &'static str> {
        if args.len() != 3 {
            return Err("Usage: program_name <image_path> <scale_factor>");
        }

        let image_path = args[1].clone();
        let scale_factor = args[2].parse::<u32>().unwrap_or_else(|_| {
            eprintln!(
                "Error: Invalid scale factor provided. Please provide a valid integer number."
            );
            std::process::exit(1);
        });

        Ok(ImageInfo {
            image_path,
            scale_factor,
        })
    }
}
fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Create ImageInfo struct from command-line arguments
    let image_info = ImageInfo::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });
    get_image(image_info)
}

fn get_image(img_info: ImageInfo) {
    // Process each frame in the GIF animation
    if img_info.image_path.ends_with(".gif") {
        let f = File::open(img_info.image_path).expect("File not found");
        // Wrap the file in a BufReader to satisfy BufRead trait bound
        let reader = BufReader::new(f);

        // Decode the GIF file
        let decoder = GifDecoder::new(reader).unwrap();
        let frames: Frames = decoder.into_frames();
        for frame in frames {
            let frame: Frame = frame.unwrap();
            let img: DynamicImage = frame.into_buffer().into();
            let ascii = convert_to_ascii(&img, img_info.scale_factor);
            print!("{}", ascii);
        }
    } else {
        let img = open(img_info.image_path).unwrap();
        let ascii = convert_to_ascii(&img, img_info.scale_factor);
        print!("{}", ascii)
    }
}

fn convert_to_ascii(img: &DynamicImage, scale: u32) -> String {
    let density = "N@#W$9876543210?!abc;:+=-,._      "
        .bytes()
        .map(|c| c as char)
        .collect::<Vec<char>>();
    let mut ascii_image = String::new();

    let (width, height) = img.dimensions();
    for col in 0..height {
        for row in 0..width {
            if col % (scale * 2) == 0 && row % scale == 0 {
                let pixel = img.get_pixel(row, col);
                let [red, green, blue] = pixel.to_rgb().0;
                let brightness = red / 3 + green / 3 + blue / 3;
                if brightness == 0 || brightness == 255 {
                    ascii_image = format!("{}{}", ascii_image, " ");
                } else {
                    let idx = ((brightness as usize) * density.len()) / 255;
                    ascii_image = format!(
                        "{}{}",
                        ascii_image,
                        density[idx].to_string().color(RGB::new(red, green, blue))
                    );
                }
            }
        }
        if col % (scale * 2) == 0 {
            ascii_image = format!("{}{}", ascii_image, "\n");
        }
    }
    ascii_image
}
