use image::{Rgba, Pixel};

pub fn get_hue(pixel: &Rgba<u8>) -> f32 {
    let r = pixel[0] as f32 / 255.0;
    let g = pixel[1] as f32 / 255.0;
    let b = pixel[2] as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let hue = if delta == 0.0 {
        0.0
    } else if max == r {
        ((g - b) / delta) % 6.0
    } else if max == g {
        ((b - r) / delta) + 2.0
    } else {
        ((r - g) / delta) + 4.0
    };

    hue / 6.0
}

pub fn get_saturation(pixel: &Rgba<u8>) -> f32 {
    // if all values are the same, saturation = 0
    if pixel.0[0] == pixel.0[1] && pixel.0[1] == pixel.0[2] {
        return 0.0;
    }

    let sorted = sort_and_normalize(pixel);
    let luminace = get_luminace(pixel);

    if luminace >= 0f32 && luminace <= 0.5 {
        (sorted[2] - sorted[0]) / (sorted[2] + sorted[0])
    } else if luminace > 0.5 && luminace < 1f32 {
        (sorted[2] - sorted[0]) / (2f32 - sorted[2] - sorted[0])
    } else {
        panic!("luminace exceeds 0..1.0 range: {luminace}");
    }
}

pub fn get_luminace(pixel: &Rgba<u8>) -> f32 {
    pixel.to_luma().0[0] as f32 / 255.0
}

fn sort_and_normalize(pixel: &Rgba<u8>) -> [f32; 3] {
    // create slice with pixel channels
    let mut rgb_channels = [pixel[0], pixel[1], pixel[2]];
    rgb_channels.sort();

    // normalize values to [0.0, 1.0]
    let mut tmp = [0f32; 3];
    for i in 0..2 {
        tmp[i] = rgb_channels[i] as f32 / 255f32
    }

    tmp
}