use std::error::Error;

use image::io::Reader as ImageReader;
use image::RgbImage;

pub mod array;
pub mod energy;
pub mod parser;
pub mod seam;

#[cfg(not(tarpaulin_include))]
pub fn run(config: parser::Config) -> Result<(), Box<dyn Error>> {
    let img_original = ImageReader::open(&config.infile)?.decode()?.to_rgb8();
    let img_carved = seamcarve(&img_original, config.new_width, config.new_height)?;
    img_carved.save(config.get_outfile())?;
    Ok(())
}

pub fn seamcarve(
    img: &RgbImage,
    new_width: u32,
    new_height: u32,
) -> Result<RgbImage, Box<dyn Error>> {
    let (width, height) = img.dimensions();

    let mut img_array = array::Array2d::from_image(img)?;
    let mut energy_map = energy::get_energy_img(&img_array)?;
    let mut seam;

    let vertical_to_remove = width.checked_sub(new_width).ok_or(format!(
        "new_width cannot be greater than original_width, got {} and {}",
        new_width, width
    ))?;
    for _ in 0..vertical_to_remove {
        seam = seam::find_vertical_seam(&energy_map);
        img_array.remove_seam(&seam)?;
        energy::update_energy_img(&mut energy_map, &img_array, &seam)?;
    }

    let horizontal_to_remove = height.checked_sub(new_height).ok_or(format!(
        "new_height cannot be greater than original_height, got {} and {}",
        new_height, height
    ))?;
    if horizontal_to_remove > 0 {
        img_array.transpose();
        energy_map.transpose();
        for _ in 0..horizontal_to_remove {
            seam = seam::find_vertical_seam(&energy_map);
            img_array.remove_seam(&seam)?;
            energy::update_energy_img(&mut energy_map, &img_array, &seam)?;
        }
        img_array.transpose();
    }
    Ok(img_array.to_image())
}
