use crate::array::Array2d;

pub fn find_vertical_seam(energy: &Array2d<u32>) -> Vec<usize> {
    let (width, height, size) = (energy.width(), energy.height(), energy.size());
    let mut cost = Array2d::new(width, vec![0; size]).unwrap();
    let mut path = Array2d::new(width, vec![0; size - width]).unwrap();
    let mut seam = Vec::with_capacity(height);
    for x in 0..width {
        cost[(x, height - 1)] = energy[(x, height - 1)];
    }
    for y in (0..(height - 1)).rev() {
        for x in 0..width {
            let (mut best_index, mut min_cost) = (x, cost[(x, y + 1)]);
            if x > 0 && cost[(x - 1, y + 1)] < min_cost {
                best_index = x - 1;
                min_cost = cost[(x - 1, y + 1)]
            }
            if x < width - 1 && cost[(x + 1, y + 1)] < min_cost {
                best_index = x + 1;
                min_cost = cost[(x + 1, y + 1)]
            }
            path[(x, y)] = best_index;
            cost[(x, y)] = energy[(x, y)] + min_cost;
        }
    }
    seam.push((0..width).min_by_key(|&x| cost[(x, 0)]).unwrap());
    for y in 0..(height - 1) {
        seam.push(path[(seam[y], y)])
    }
    seam
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::array::positions_from_image;
    use crate::energy::get_energy_img;
    use image::{Rgb, RgbImage};

    #[test]
    fn vertical_seam() {
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
        let positions = positions_from_image(&img).unwrap();
        let energy = get_energy_img(&img, &positions).unwrap();
        let seam = find_vertical_seam(&energy);
        assert_eq!(vec![3, 4, 3, 2, 2], seam);
    }
}
