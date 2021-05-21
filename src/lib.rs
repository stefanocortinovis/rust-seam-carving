use std::error::Error;

use image::io::Reader as ImageReader;
use image::RgbImage;

pub mod array;
pub mod energy;
pub mod parser;
pub mod seam;

use array::Array2d;

#[cfg(not(tarpaulin_include))]
pub fn run(config: parser::Config) -> Result<(), Box<dyn Error>> {
    let img_original = ImageReader::open(&config.infile)?.decode()?.to_rgb8();
    let img_carved = seamcarve(&img_original, config.new_width, config.new_height)?; // img_original moved into function, no longer valid
    img_carved.save(config.get_outfile())?;
    Ok(())
}

pub fn seamcarve(
    img: &RgbImage,
    new_width: u32,
    new_height: u32,
) -> Result<RgbImage, Box<dyn Error>> {
    let mut img_carved = img.clone();
    let (width, height) = img_carved.dimensions();

    if (new_width == width) && (new_height == height) {
        return Ok(img_carved);
    }

    let mut positions = array::positions_from_image(&img_carved)?;
    let mut energy_map = energy::get_energy_img(&img_carved, &positions)?;

    if new_width < width {
        let vertical_to_remove = width - new_width;
        carve_vertical(
            &mut energy_map,
            &img_carved,
            &mut positions,
            vertical_to_remove,
        )?;
    }

    if new_height < height {
        positions.transpose();
        energy_map.transpose();
        let horizontal_to_remove = height - new_height;
        carve_vertical(
            &mut energy_map,
            &img_carved,
            &mut positions,
            horizontal_to_remove,
        )?;
        positions.transpose();
        energy_map.transpose();
    }

    img_carved = array::filter_image_by_positions(&img_carved, &positions);

    if new_width > width {
        positions = array::positions_from_image(&img_carved)?;
        let vertical_to_insert = new_width - width;
        img_carved = insert_vertical(
            &mut energy_map,
            &img_carved,
            &mut positions,
            vertical_to_insert,
        )?;
    }

    if new_height > height {
        positions = array::positions_from_image(&img_carved)?;
        energy_map = energy::get_energy_img(&img_carved, &positions)?;
        positions.transpose();
        energy_map.transpose();
        let horizontal_to_insert = new_height - height;
        img_carved = insert_horizontal(
            &mut energy_map,
            &img_carved,
            &mut positions,
            horizontal_to_insert,
        )?;
    }

    Ok(img_carved)
}

fn carve_vertical(
    energy_map: &mut Array2d<u32>,
    img: &RgbImage,
    positions: &mut Array2d<(u32, u32)>,
    to_remove: u32,
) -> Result<(), Box<dyn Error>> {
    let mut seam;
    for _ in 0..to_remove {
        seam = seam::find_vertical_seam(energy_map);
        positions.remove_seam(&seam)?;
        energy::update_energy_img(energy_map, img, positions, &seam)?;
    }
    Ok(())
}

fn insert_vertical(
    energy_map: &mut Array2d<u32>,
    img: &RgbImage,
    positions: &mut Array2d<(u32, u32)>,
    to_insert: u32,
) -> Result<RgbImage, Box<dyn Error>> {
    let height = positions.height();
    let mut seams = Vec::with_capacity(height);
    for _ in 0..height {
        seams.push(Vec::with_capacity(to_insert as usize));
    }
    let mut seam;
    for _ in 0..to_insert {
        seam = seam::find_vertical_seam(energy_map);
        seam.iter()
            .enumerate()
            .for_each(|(y, &x)| seams[y].push(positions[(x, y)].0 as usize));
        positions.remove_seam(&seam)?;
        energy::update_energy_img(energy_map, img, positions, &seam)?;
    }
    Ok(seam::insert_vertical_seams(img, &seams))
}

fn insert_horizontal(
    energy_map: &mut Array2d<u32>,
    img: &RgbImage,
    positions: &mut Array2d<(u32, u32)>,
    to_insert: u32,
) -> Result<RgbImage, Box<dyn Error>> {
    let height = positions.height();
    let mut seams = Vec::with_capacity(height);
    for _ in 0..height {
        seams.push(Vec::with_capacity(to_insert as usize));
    }
    let mut seam;
    for _ in 0..to_insert {
        seam = seam::find_vertical_seam(energy_map);
        seam.iter()
            .enumerate()
            .for_each(|(y, &x)| seams[y].push(positions[(x, y)].1 as usize));
        positions.remove_seam(&seam)?;
        energy::update_energy_img(energy_map, img, positions, &seam)?;
    }
    Ok(seam::insert_horizontal_seams(img, &seams))
}
