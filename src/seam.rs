use std::cmp::min;

use crate::array::Array2d;

pub fn find_vertical_seam(energy: &Array2d<u32>) -> Array2d<Option<u32>> {
    let mut result = Array2d::new(energy.width(), vec![None; energy.size()]).unwrap();
    for i in 0..result.width() {
        for j in 0..result.height() {
            find_vertical_seam_memoized(energy, &mut result, i, j);
        }
    }
    result
}

fn find_vertical_seam_memoized(energy: &Array2d<u32>, result: &mut Array2d<Option<u32>>, i: usize, j: usize) -> u32 {
    let mut q;
    if let Some(r) = result[(i, j)] {
        q = r;
    } else {
        if j == energy.height() - 1 {
            q = energy[(i, j)];
        } else {
            q = find_vertical_seam_memoized(energy, result, i, j+1);
            if i > 0 {
                q = min(q, find_vertical_seam_memoized(energy, result, i-1, j+1));
            }
            if i < energy.width() - 1 {
                q = min(q, find_vertical_seam_memoized(energy, result, i+1, j+1));
            }
            q += energy[(i, j)]
        }
    }
    result[(i, j)] = Some(q);
    q
}
