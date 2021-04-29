use std::mem;

use image::{Pixel, Rgb};
use num_traits::ToPrimitive;

use crate::array::Array2d;

pub fn get_energy_img(img: &Array2d<Rgb<u8>>) -> Result<Array2d<u32>, &'static str> {
    let (width, height) = img.dimensions();
    let mut e = vec![];
    for y in 0..height {
        for x in 0..width {
            e.push(get_energy_pixel(img, x, y))
        }
    }
    Array2d::new(width as usize, e)
}

pub fn update_energy_img(
    energy: &mut Array2d<u32>,
    img: &Array2d<Rgb<u8>>,
    seam: &[usize],
) -> Result<(), &'static str> {
    energy.remove_seam(&seam)?;
    let (width, height) = img.dimensions(); // seam already removed
    let (mut first, mut last) = (seam[0], seam[height - 1]); // fine even if last on right boundary
    if first > last {
        mem::swap(&mut first, &mut last);
    }
    for (y, &x) in seam.iter().enumerate() {
        if (y == 0) || (y == height - 1) {
            for i in first..last {
                energy[(i, y)] = get_energy_pixel(img, i, y);
            }
        }
        let left = x.checked_sub(1).unwrap_or(width - 1);
        let right = x % width;
        energy[(left, y)] = get_energy_pixel(img, left, y);
        energy[(right, y)] = get_energy_pixel(img, right, y);
    }
    Ok(())
}

fn get_energy_pixel(img: &Array2d<Rgb<u8>>, x: usize, y: usize) -> u32 {
    let (width, height) = img.dimensions();
    let above = y.checked_sub(1).unwrap_or(height - 1);
    let below = (y + 1) % height;
    let left = x.checked_sub(1).unwrap_or(width - 1);
    let right = (x + 1) % width;
    squared_diff_pixels(img[(x, above)], img[(x, below)])
        + squared_diff_pixels(img[(left, y)], img[(right, y)])
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
    use image::{Rgb, RgbImage};

    #[test]
    fn energy_computation_1() {
        let mut img = RgbImage::new(3, 4);
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
        let img_array = Array2d::from_image(&img).unwrap();
        let energy = get_energy_img(&img_array).unwrap();

        #[rustfmt::skip]
        assert_eq!(
            vec![
                20808, 52020, 20808,
                20808, 52225, 21220,
                20809, 52024, 20809,
                20808, 52225, 21220
            ],
            energy.raw_data()
        );
    }

    #[test]
    fn energy_computation_2() {
        let mut img = RgbImage::new(6, 5);
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
        let img_array = Array2d::from_image(&img).unwrap();
        let energy = get_energy_img(&img_array).unwrap();

        #[rustfmt::skip]
        assert_eq!(
            vec![
                57685, 50893, 91370, 25418, 33055, 37246,
                15421, 56334, 22808, 54796, 11641, 25496,
                12344, 19236, 52030, 17708, 44735, 20663,
                17074, 23678, 30279, 80663, 37831, 45595,
                32337, 30796, 4909, 73334, 40613, 36556
            ],
            energy.raw_data()
        );
    }

    #[test]
    fn energy_update_1() {
        let mut img = RgbImage::new(3, 4);
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
        let mut img_array = Array2d::from_image(&img).unwrap();
        let seam = [0, 1, 2, 1];
        let mut energy_updated = get_energy_img(&img_array).unwrap();
        img_array.remove_seam(&seam).unwrap();
        let energy_computed = get_energy_img(&img_array).unwrap();
        update_energy_img(&mut energy_updated, &img_array, &seam).unwrap();

        assert_eq!(energy_computed, energy_updated);
    }
    #[test]
    fn energy_update_2() {
        let mut img = RgbImage::new(6, 5);
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

        let mut img_array = Array2d::from_image(&img).unwrap();
        let seam_1 = [0, 1, 2, 3, 4];
        let mut energy_updated = get_energy_img(&img_array).unwrap();
        img_array.remove_seam(&seam_1).unwrap();
        let energy_computed = get_energy_img(&img_array).unwrap();
        update_energy_img(&mut energy_updated, &img_array, &seam_1).unwrap();
        assert_eq!(energy_computed, energy_updated);

        let mut img_array = Array2d::from_image(&img).unwrap();
        let seam_2 = [4, 3, 2, 1, 0];
        let mut energy_updated = get_energy_img(&img_array).unwrap();
        img_array.remove_seam(&seam_2).unwrap();
        let energy_computed = get_energy_img(&img_array).unwrap();
        update_energy_img(&mut energy_updated, &img_array, &seam_2).unwrap();
        assert_eq!(energy_computed, energy_updated);

        let mut img_array = Array2d::from_image(&img).unwrap();
        let seam_3 = [5, 4, 3, 2, 1];
        let mut energy_updated = get_energy_img(&img_array).unwrap();
        img_array.remove_seam(&seam_3).unwrap();
        let energy_computed = get_energy_img(&img_array).unwrap();
        update_energy_img(&mut energy_updated, &img_array, &seam_3).unwrap();
        assert_eq!(energy_computed, energy_updated);
    }
}
