use image::io::Reader as ImageReader;
use image::{GrayImage, Rgb};

use rsc;

const SCALING: f64 = 2000f64;

#[test]
#[should_panic]
fn wider() {
    let img_original = ImageReader::open("./img/Broadway_tower_edit.jpg")
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();
    let (original_width, original_height) = img_original.dimensions();
    let (new_width, new_height) = (original_width + 1000, original_height);
    rsc::seamcarve(&img_original, new_width, new_height).unwrap();
}

#[test]
#[should_panic]
fn taller() {
    let img_original = ImageReader::open("./img/Broadway_tower_edit.jpg")
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();
    let (original_width, original_height) = img_original.dimensions();
    let (new_width, new_height) = (original_width, original_height + 1000);
    rsc::seamcarve(&img_original, new_width, new_height).unwrap();
}

#[test]
#[ignore]
fn energy_map() {
    let img_original = ImageReader::open("./img/Broadway_tower_edit.jpg")
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();
    let (width, height) = img_original.dimensions();
    let positions = rsc::array::positions_from_image(&img_original).unwrap();
    let energy_map = rsc::energy::get_energy_img(&img_original, &positions).unwrap();
    let mut energy_map_scaled = vec![];
    for p in energy_map.raw_data() {
        energy_map_scaled.push(((*p as f64) / SCALING * (u8::MAX as f64)) as u8)
    }
    let energy_img = GrayImage::from_raw(width, height, energy_map_scaled).unwrap();
    energy_img
        .save("./img/Broadway_tower_edit_energy.jpg")
        .unwrap();
}

#[test]
#[ignore]
fn seam_removal_img() {
    let mut img_original = ImageReader::open("./img/Broadway_tower_edit.jpg")
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();
    let width = img_original.dimensions().0 as usize;
    let positions = rsc::array::positions_from_image(&img_original).unwrap();
    let energy_map = rsc::energy::get_energy_img(&img_original, &positions).unwrap();
    let seam = rsc::seam::find_vertical_seam(&energy_map);
    seam.iter().enumerate().for_each(|(y, &x)| {
        img_original.put_pixel(x as u32, y as u32, Rgb([255, 0, 0]));
        if x > 0 {
            img_original.put_pixel((x - 1) as u32, y as u32, Rgb([255, 0, 0]));
        }
        if x > 1 {
            img_original.put_pixel((x - 2) as u32, y as u32, Rgb([255, 0, 0]));
        }
        if x < width - 1 {
            img_original.put_pixel((x + 1) as u32, y as u32, Rgb([255, 0, 0]));
        }
        if x < width - 2 {
            img_original.put_pixel((x + 2) as u32, y as u32, Rgb([255, 0, 0]));
        }
    });
    img_original
        .save("./img/Broadway_tower_edit_seam.jpg")
        .unwrap();
}

#[test]
#[ignore]
fn multiple_seam_removal_img() {
    let mut img_original = ImageReader::open("./img/Broadway_tower_edit.jpg")
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();
    let width = img_original.dimensions().0;
    let new_width = 957;
    let vertical_to_remove = width - new_width;
    let mut positions = rsc::array::positions_from_image(&img_original).unwrap();
    let mut energy_map = rsc::energy::get_energy_img(&img_original, &positions).unwrap();
    let mut seam;
    for _ in 0..vertical_to_remove {
        seam = rsc::seam::find_vertical_seam(&energy_map);
        seam.iter().enumerate().for_each(|(y, &x)| {
            img_original[positions[(x, y)]] = Rgb([255, 0, 0]);
        });
        positions.remove_seam(&seam).unwrap();
        rsc::energy::update_energy_img(&mut energy_map, &img_original, &positions, &seam).unwrap();
    }
    img_original
        .save("./img/Broadway_tower_edit_seam_multiple.jpg")
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
        rsc::seamcarve(&img_original, new_width, new_height).unwrap()
    );
}

#[test]
#[ignore]
fn carve_width() {
    // dimensions: 1428 x 968
    let img_original = ImageReader::open("./img/Broadway_tower_edit.jpg")
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();
    let (new_width, new_height) = (957, img_original.dimensions().1);
    let img_carved = rsc::seamcarve(&img_original, new_width, new_height).unwrap();
    img_carved
        .save("./img/Broadway_tower_edit_carved_width.jpg")
        .unwrap();
    assert_eq!((new_width, new_height), img_carved.dimensions());
}

#[test]
#[ignore]
fn carve_height() {
    let img_original = ImageReader::open("./img/Broadway_tower_edit.jpg")
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();
    let (new_width, new_height) = (img_original.dimensions().0, 550);
    let img_carved = rsc::seamcarve(&img_original, new_width, new_height).unwrap();
    img_carved
        .save("./img/Broadway_tower_edit_carved_height.jpg")
        .unwrap();
    assert_eq!((new_width, new_height), img_carved.dimensions());
}

#[test]
#[ignore]
fn carve_both() {
    let img_original = ImageReader::open("./img/Broadway_tower_edit.jpg")
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();
    let (new_width, new_height) = (550, 550);
    let img_carved = rsc::seamcarve(&img_original, new_width, new_height).unwrap();
    img_carved
        .save("./img/Broadway_tower_edit_carved_both.jpg")
        .unwrap();
    assert_eq!((new_width, new_height), img_carved.dimensions());
}

#[test]
fn carve_both_fast() {
    let img_original = ImageReader::open("./img/Broadway_tower_edit.jpg")
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();
    let (width, height) = img_original.dimensions();
    let (new_width, new_height) = (width - 1, height - 1);
    let img_carved = rsc::seamcarve(&img_original, new_width, new_height).unwrap();
    assert_eq!((new_width, new_height), img_carved.dimensions());
}
