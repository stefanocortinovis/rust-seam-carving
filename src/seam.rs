use crate::array::Array2d;

pub fn find_vertical_seam(energy: &Array2d<u32>) -> Vec<usize> {
    let mut cost = Array2d::new(energy.width(), vec![None; energy.size()]).unwrap();
    let mut path = Array2d::new(energy.width(), vec![None; energy.size()]).unwrap();
    for i in 0..cost.width() {
        for j in 0..cost.height() {
            find_vertical_seam_memoized(energy, &mut cost, &mut path, i, j);
        }
    }
    let mut seam = Vec::with_capacity(energy.height());
    seam.push(
        (0..energy.width())
            .min_by_key(|&i| {
                cost[(i, 0)].unwrap() // maybe better expect?
            })
            .unwrap(),
    );
    for i in 0..(energy.height() - 1) {
        seam.push(path[(seam[i], i)].unwrap())
    }
    seam
}

fn find_vertical_seam_memoized(
    energy: &Array2d<u32>,
    cost: &mut Array2d<Option<u32>>,
    path: &mut Array2d<Option<usize>>,
    i: usize,
    j: usize,
) -> u32 {
    let mut p = 0;
    let mut q;
    if let (Some(r), Some(s)) = (path[(i, j)], cost[(i, j)]) {
        p = r;
        q = s;
    } else {
        if j == energy.height() - 1 {
            q = energy[(i, j)];
        } else {
            p = i;
            q = find_vertical_seam_memoized(energy, cost, path, i, j + 1);
            if i > 0 {
                let s = find_vertical_seam_memoized(energy, cost, path, i - 1, j + 1);
                if s < q {
                    p = i - 1;
                    q = s;
                }
            }
            if i < energy.width() - 1 {
                let s = find_vertical_seam_memoized(energy, cost, path, i + 1, j + 1);
                if s < q {
                    p = i + 1;
                    q = s;
                }
            }
            q += energy[(i, j)]
        }
    }
    path[(i, j)] = Some(p);
    cost[(i, j)] = Some(q);
    q
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::energy::get_energy_img;
    use image::{ImageBuffer, Rgb};

    #[test]
    fn vertical_seam() {
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
        let seam = find_vertical_seam(&energy);
        assert_eq!(vec![3, 4, 3, 2, 2], seam);
    }
}
