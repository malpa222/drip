pub mod sorting;
use sorting::sorter::{
    self,
    Options,
    Directions,
    Markers,
};

use image::DynamicImage;
use std::time::{SystemTime, UNIX_EPOCH, Instant};

const PATH: &str = "data/clouds.jpg";

const TRESHOLD: f32 = 0.675;

fn main() {
    let image = load_image(PATH);
    let options = Options {
        Marker: Markers::Hue,
        Direction: Directions::Rows
    };

    let t_zero = Instant::now();
    let sorted = sorter::sort_image(image.clone(), TRESHOLD, options);
    println!("Sorted image in: {:?}", t_zero.elapsed().as_secs_f32());

    save_image(sorted);
}

fn load_image(path: &str) -> DynamicImage {
    match image::open(path) {
        Ok(image) => image,
        Err(err) => panic!("{err}")
    }
}

fn save_image(image: DynamicImage) {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut save_path = String::from(PATH);
    save_path.insert_str(
        PATH.chars().position(|c| c == '.').unwrap(),
        &format!("-{:?}", timestamp));

    match image.save(save_path) {
        Ok(_) => (),
        Err(err) => panic!("{err}")
    }
}