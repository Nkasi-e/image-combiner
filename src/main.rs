mod args;
use args::Args;
use image::{
    imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};
use std::{fs::File, io::BufReader};

#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormats,
    BufferTooSmall,
}

struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FloatingImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = height * width * 4;
        let buffer = Vec::with_capacity(buffer_capacity.try_into().unwrap());
        FloatingImage {
            width,
            height,
            data: buffer,
            name,
        }
    }
    // defining a method
    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        }
        self.data = data;
        Ok(())
    }
}

fn main() -> Result<(), ImageDataErrors> {
    let args: Args = Args::new();
    let (image_1, image_format_1) = find_image_from_path(args.image_1);
    let (image_2, image_format_2) = find_image_from_path(args.image_2);

    // handling error
    if image_format_1 != image_format_2 {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    let (image_1, image_2) = standardize_size(image_1, image_2);

    let mut output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

    let combined_data = combine_images(image_1, image_2);
    output.set_data(combined_data)?; // the ? propagates value

    // defining where to save the image output
    image::save_buffer_with_format(
        output.name,
        &output.data,
        output.width,
        output.height,
        image::ColorType::Rgb8,
        image_format_1,
    )
    .unwrap();
    Ok(())
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap(); // the Reader struct implements an open function which takes a path to an image file and returns a result contained in the Reader
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();
    (image, image_format)
}

// Finding the smallest image
fn get_smallest_image_dimentions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
    let pix_1 = dim_1.0 * dim_1.1;
    let pix_2 = dim_2.0 * dim_2.1;

    // comparing the number of pixels and the image
    return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn standardize_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
    // Destructuring
    let (width, height) = get_smallest_image_dimentions(image_1.dimensions(), image_2.dimensions());
    println!("width: {} height: {}\n", width, height);

    // In order to standardize the size we first find out
    if image_2.dimensions() == (width, height) {
        (image_1.resize_exact(width, height, Triangle), image_2)
    } else {
        (image_1, image_2.resize_exact(width, height, Triangle))
    }
}

// fn to process the combined images
fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    let vec_1 = image_1.to_rgb8().into_vec();
    let vec_2 = image_2.to_rgb8().into_vec();

    alternate_pixels(vec_1, vec_2)
}

// fn to alternate pixels of the two images
fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    // if vec_1.len() % 8 != 0 || vec_2.len() % 8 != 0 {
    //     panic!("Input vectors must have a length that is a multiple of 8");
    // }
    // if vec_1.len() == 5, then the vec macro vec![] will create a vec of u8 of that same length
    let mut combined_data = vec![0u8; vec_1.len()];

    let mut i = 0;
    while i < vec_1.len() {
        if i % 8 == 0 {
            combined_data.splice(i..=1 + 3, set_rgba(&vec_1, i, i + 3));
        } else {
            combined_data.splice(i..=1 + 3, set_rgba(&vec_2, i, i + 3));
        }
        i += 4;
    }
    combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();
    for i in start..=end {
        let val = match vec.get(i) {
            Some(d) => *d, // the * dereference the value
            None => panic!("Index out of bounds"),
        };
        rgba.push(val);
    }
    rgba
}
