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
    fn energy_1() {
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

    #[test]
    fn energy_2() {
        let mut img = ImageBuffer::new(6, 5);
        img.put_pixel(0, 0, Rgb([78, 209, 79]));
        img.put_pixel(1, 0, Rgb([63, 118, 247]));
        img.put_pixel(2, 0, Rgb([92, 175, 95]));
        img.put_pixel(3, 0, Rgb([243, 73, 183]));
        img.put_pixel(4, 0, Rgb([210, 109, 104]));
        img.put_pixel(5, 0, Rgb([252, 101, 119]));
        img.put_pixel(0, 1, Rgb([224, 191, 182]));
        img.put_pixel(1, 1, Rgb([108, 89, 82]));
        img.put_pixel(2, 1, Rgb([80, 196, 230]));
        img.put_pixel(3, 1, Rgb([112, 156, 180]));
        img.put_pixel(4, 1, Rgb([176, 178, 120]));
        img.put_pixel(5, 1, Rgb([142, 151, 142]));
        img.put_pixel(0, 2, Rgb([117, 189, 149]));
        img.put_pixel(1, 2, Rgb([171, 231, 153]));
        img.put_pixel(2, 2, Rgb([149, 164, 168]));
        img.put_pixel(3, 2, Rgb([107, 119, 71]));
        img.put_pixel(4, 2, Rgb([120, 105, 138]));
        img.put_pixel(5, 2, Rgb([163, 174, 196]));
        img.put_pixel(0, 3, Rgb([163, 222, 132]));
        img.put_pixel(1, 3, Rgb([187, 117, 183]));
        img.put_pixel(2, 3, Rgb([92, 145, 69]));
        img.put_pixel(3, 3, Rgb([158, 143, 79]));
        img.put_pixel(4, 3, Rgb([220, 75, 222]));
        img.put_pixel(5, 3, Rgb([189, 73, 214]));
        img.put_pixel(0, 4, Rgb([211, 120, 173]));
        img.put_pixel(1, 4, Rgb([188, 218, 244]));
        img.put_pixel(2, 4, Rgb([214, 103, 68]));
        img.put_pixel(3, 4, Rgb([163, 166, 246]));
        img.put_pixel(4, 4, Rgb([79, 125, 246]));
        img.put_pixel(5, 4, Rgb([211, 201, 98]));
        let energy = get_energy_img(&img).unwrap();
        assert_eq!(
            vec![
                57685, 50893, 91370, 25418, 33055, 37246, 15421, 56334, 22808, 54796, 11641, 25496,
                12344, 19236, 52030, 17708, 44735, 20663, 17074, 23678, 30279, 80663, 37831, 45595,
                32337, 30796, 4909, 73334, 40613, 36556
            ],
            energy.data
        );
    }
}
