use image::{open, DynamicImage, GenericImageView, ImageBuffer};
use ndarray::Array3;

#[derive(Debug, PartialEq, Eq)]
pub enum Scale {
    Down(u32),
    Up(u32),
}

pub fn scale_image(dir: &str, scale: Scale) {
    let img = open(dir).unwrap();
    let (actual_width, actual_height) = img.dimensions();
    let (width, height): (u32, u32);
    match scale {
        Scale::Down(s) => {
            width = (actual_width / s) as u32;
            height = (actual_height / s) as u32;
        }
        Scale::Up(s) => {
            width = (actual_width * s) as u32;
            height = (actual_height * s) as u32;
        }
    }
    println!("({},{})", width, height)
}

pub fn image_to_ndarray(img: &DynamicImage) -> Array3<u8> {
    let (width, height) = img.dimensions();
    let img_rgb = img.to_rgb8();
    let img_vec = img_rgb.into_raw();
    return Array3::from_shape_vec((height as usize, width as usize, 3), img_vec).unwrap();
}

pub fn ndarray_to_image(arr: &Array3<u8>) -> DynamicImage {
    let (height, width, _) = arr.dim();
    let img_vec = &arr.clone().into_raw_vec();
    return DynamicImage::ImageRgba8(
        ImageBuffer::from_vec(width as u32, height as u32, img_vec.to_vec()).unwrap(),
    );
}
