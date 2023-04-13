use image::{io::Reader as ImageReader, DynamicImage, ImageError, Rgb, RgbImage, Pixel};
use core::time;
use std::time::{SystemTime, UNIX_EPOCH};

const PATH: &str = "data/clouds.jpg";

const BLACK: i32 = 0x00000;
const WHITE: i32 = 0xfffff;

enum Options {
    Hue,
    Lightness,
    Saturation
}

fn main() {
    let dynamic_img = load_image(PATH);
    if let Some(img) = dynamic_img {
        let sorted = sort_image(img.clone(), Options::Lightness);

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let mut save_path = String::from(PATH);
        save_path.insert_str(
            PATH.chars().position(|c| c == '.').unwrap(),
            &format!("-{:?}", timestamp));

        match sorted.save(save_path) {
            Ok(_) => (),
            Err(err) => println!("{}", err)
        }
    }
}

fn load_image(path: &str) -> Option<DynamicImage> {
    match ImageReader::open(path) {
        Ok(encoded) => {
            match encoded.decode() {
                Ok(image) => Some(image),
                Err(err) => {
                    handle_error(err);
                    None
                }
            }
        },
        Err(err) =>  {
            println!("{err}");
            None
        }
    }
}

fn sort_image(img: DynamicImage, options: Options) -> RgbImage {
    match options {
        Options::Hue => unimplemented!(),
        Options::Saturation => unimplemented!(),
        Options::Lightness => sort_by_lightness(img),
    }
}

fn sort_by_hue(img: RgbImage) {
    unimplemented!()
}

fn sort_by_saturation(img: RgbImage) {
    unimplemented!()
}

fn sort_by_lightness(img: DynamicImage) -> RgbImage {
    let mut image = img.clone().into_rgb8();

    fn lightness(pixel: &Rgb<u8>) -> f32 {
        pixel.to_luma().0[0] as f32 / 255.0
    }

    let (width, height) = image.dimensions();

    for row in 0..height {
        let mut pixels: Vec<Rgb<u8>> = Vec::with_capacity(width as usize);
        let mut x = 0u32;

        // populate row with pixels from image
        for _ in 0..width {
            pixels.push(image[(x, row)]);
            x += 1; // move to the next pixel
        }

        // do the sorting
        pixels.sort_by(|a, b| {
            lightness(a).partial_cmp(&lightness(b)).unwrap()
        });

        // reorder pixels in the row
        for (i, pixel) in pixels.iter().enumerate() {
            image.put_pixel(i as u32, row, *pixel)
        }
    }

    return image;
}


fn handle_error(err: ImageError) {
    unimplemented!()
}