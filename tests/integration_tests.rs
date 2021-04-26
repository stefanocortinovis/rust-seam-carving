use image::io::Reader as ImageReader;
use image::{GenericImageView, GrayImage};

use rsc;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use image::GrayImage;

//     const SCALING: f64 = 2000f64;
//     #[test]
//     fn no_carving() {
//         let img_original = ImageReader::open("./img/Broadway_tower_edit.jpg")
//             .unwrap()
//             .decode()
//             .unwrap();
//         let (new_width, new_height) = img_original.dimensions();
//         assert_eq!(
//             img_original,
//             seamcarve(&img_original, new_height, new_width)
//         );
//     }
// }

const SCALING: f64 = 2000f64;

#[test]
#[ignore]
fn energy_map() {
    let img_original = ImageReader::open("./img/Broadway_tower_edit.jpg")
        .unwrap()
        .decode()
        .unwrap();
    let (width, height) = img_original.dimensions();
    let energy = rsc::energy::get_energy_img(&img_original).unwrap();
    let mut energy_vec = vec![];
    for p in &energy.data {
        energy_vec.push(((*p as f64) / SCALING * (u8::MAX as f64)) as u8)
    }
    let energy_img = GrayImage::from_raw(width, height, energy_vec).unwrap();
    energy_img
        .save("./img/Broadway_tower_edit_energy.jpg")
        .unwrap();
}

#[test]
#[ignore]
fn no_carving() {
    let img_original = ImageReader::open("./img/Broadway_tower_edit.jpg")
        .unwrap()
        .decode()
        .unwrap();
    let (new_width, new_height) = img_original.dimensions();
    assert_eq!(
        img_original,
        rsc::seamcarve(&img_original, new_height, new_width).unwrap()
    );
}
