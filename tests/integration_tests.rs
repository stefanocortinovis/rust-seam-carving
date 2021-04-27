use image::io::Reader as ImageReader;
use image::GrayImage;

use rsc;

const SCALING: f64 = 2000f64;

#[test]
#[ignore]
fn energy_map() {
    let img_original = ImageReader::open("./img/Broadway_tower_edit.jpg")
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();
    let (width, height) = img_original.dimensions();
    let img_array = rsc::array::Array2d::from_image(&img_original).unwrap();
    let energy = rsc::energy::get_energy_img(&img_array).unwrap();
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
        .unwrap()
        .to_rgb8();
    let (new_width, new_height) = img_original.dimensions();
    assert_eq!(
        img_original,
        rsc::seamcarve(&img_original, new_height, new_width).unwrap()
    );
}
