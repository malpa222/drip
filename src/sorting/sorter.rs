use std::cmp::Ordering;

use image::{DynamicImage, Rgba, GenericImageView, GenericImage};
use super::pixel_value;

const BLACK: i32 = 0x00000;
const WHITE: i32 = 0xfffff;

pub(crate) struct Options {
    pub Marker: Markers,
    pub Direction: Directions
}

pub(crate) enum Markers {
    Hue,
    Lightness,
    Saturation
}

pub(crate) enum Directions {
    Rows,
    Columns,
    Both
}

pub(crate) fn sort_image(img: DynamicImage, treshold: f32, options: Options) -> DynamicImage {
    let pixel_fn = match options.Marker {
        Markers::Hue => pixel_value::get_hue,
        Markers::Saturation => pixel_value::get_hue,
        Markers::Lightness => pixel_value::get_luminace
    };

    let mut image = img.clone();
    match options.Direction {
        Directions::Rows => sort_rows(&mut image, treshold, pixel_fn),
        Directions::Columns => sort_columns(&mut image, treshold, pixel_fn),
        Directions::Both => {
            sort_rows(&mut image, treshold, pixel_fn);
            sort_columns(&mut image, treshold, pixel_fn);
        },
    };

    return image;
}

fn sort_rows(image: &mut DynamicImage, treshold: f32, pixel_fn: fn(&Rgba<u8>) -> f32) {
    let (width, height) = image.dimensions();

    for row in 0..height {
        let mut pixels: Vec<Rgba<u8>> = Vec::with_capacity(width as usize);

        // populate row with pixels from image
        let mut x = 0u32;
        for _ in 0..width {
            pixels.push(image.get_pixel(x, row));
            x += 1; // move to the next pixel
        }

        // do the sorting
        sort_pixels(treshold, &mut pixels, pixel_fn);

        // reorder pixels in the row
        for (i, pixel) in pixels.iter().enumerate() {
            image.put_pixel(i as u32, row, *pixel)
        }
    }

}

fn sort_columns(image: &mut DynamicImage, treshold: f32, pixel_fn: fn(&Rgba<u8>) -> f32) {
    let (width, height) = image.dimensions();

    for column in 0..width {
        let mut pixels: Vec<Rgba<u8>> = Vec::with_capacity(height as usize);

        // populate row with pixels from image
        let mut y = 0u32;
        for _ in 0..height {
            pixels.push(image.get_pixel(column, y));
            y += 1; // move to the next pixel
        }

        // do the sorting
        sort_pixels(treshold, &mut pixels, pixel_fn);

        // reorder pixels in the row
        for (i, pixel) in pixels.iter().enumerate() {
            image.put_pixel(column, i as u32, *pixel)
        }
    }
}

fn sort_pixels(treshold: f32, pixels: &mut Vec<Rgba<u8>>, pixel_fn: fn(&Rgba<u8>) -> f32) {
    pixels.sort_by(|a, b| {
        let val_a = pixel_fn(a);

        if val_a < treshold {
            return Ordering::Equal
        }

        let val_b = pixel_fn(b);
        val_a.partial_cmp(&val_b).unwrap()
    })
}