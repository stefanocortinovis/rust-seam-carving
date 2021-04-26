use image::{GenericImageView, Pixel};
use num_traits::ToPrimitive;

use crate::array::Array2d;

pub fn get_energy_img<T: GenericImageView>(img: &T) -> Result<Array2d<u32>, &'static str> {
    let (width, height) = img.dimensions();
    let mut v = vec![];
    for y in 0..height {
        for x in 0..width {
            v.push(get_energy_pixel(img, x, y))
        }
    }
    Array2d::new(width as usize, v)
}

fn get_energy_pixel<T: GenericImageView>(img: &T, x: u32, y: u32) -> u32 {
    let (width, height) = img.dimensions();
    let above = y.checked_sub(1).unwrap_or(height - 1);
    let below = (y + 1) % height;
    let left = x.checked_sub(1).unwrap_or(width - 1);
    let right = (x + 1) % width;
    squared_diff_pixels(img.get_pixel(x, above), img.get_pixel(x, below))
        + squared_diff_pixels(img.get_pixel(left, y), img.get_pixel(right, y))
}

fn squared_diff_pixels<T: Pixel>(pixel_1: T, pixel_2: T) -> u32 {
    let (channels_1, channels_2) = (pixel_1.channels(), pixel_2.channels());
    let mut diff = 0;
    for (channel_1, channel_2) in channels_1.iter().zip(channels_2) {
        diff += squared_diff_channels(channel_1, channel_2)
    }
    diff
}

fn squared_diff_channels<T: ToPrimitive>(channel_1: &T, channel_2: &T) -> u32 {
    let (channel_1, channel_2) = (
        channel_1.to_i32().unwrap_or(i32::MAX),
        channel_2.to_i32().unwrap_or(i32::MAX),
    );
    i32::pow(channel_1 - channel_2, 2) as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};

    #[test]
    fn energy() {
        let mut img = ImageBuffer::new(3, 4);
        img.put_pixel(0, 0, Rgb([255, 101, 51]));
        img.put_pixel(1, 0, Rgb([255, 101, 153]));
        img.put_pixel(2, 0, Rgb([255, 101, 255]));
        img.put_pixel(0, 1, Rgb([255, 153, 51]));
        img.put_pixel(1, 1, Rgb([255, 153, 153]));
        img.put_pixel(2, 1, Rgb([255, 153, 255]));
        img.put_pixel(0, 2, Rgb([255, 203, 51]));
        img.put_pixel(1, 2, Rgb([255, 204, 153]));
        img.put_pixel(2, 2, Rgb([255, 205, 255]));
        img.put_pixel(0, 3, Rgb([255, 255, 51]));
        img.put_pixel(1, 3, Rgb([255, 255, 153]));
        img.put_pixel(2, 3, Rgb([255, 255, 255]));
        let energy = get_energy_img(&img).unwrap();
        assert_eq!(
            vec![
                20808, 52020, 20808, 20808, 52225, 21220, 20809, 52024, 20809, 20808, 52225, 21220
            ],
            energy.data
        );
    }
}
