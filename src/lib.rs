use std::error::Error;

use image::io::Reader as ImageReader;
use image::RgbImage;

pub mod array;
pub mod energy;
pub mod parser;
pub mod seam;

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
) -> Result<RgbImage, &'static str> {
    let (width, height) = img.dimensions();
    let vertical_to_remove = width - new_width;
    let horizontal_to_remove = height - new_height;
    let mut img = array::Array2d::from_image(img)?;
    let mut energy_map = energy::get_energy_img(&img)?;
    for _ in 0..vertical_to_remove {
        let seam = seam::find_vertical_seam(&energy_map);
        img.remove_seam(&seam)?;
        energy::update_energy_img(&mut energy_map, &img, &seam)?;
    }
    if horizontal_to_remove > 0 {
        img.transpose();
        energy_map.transpose();
        for _ in 0..horizontal_to_remove {
            let seam = seam::find_vertical_seam(&energy_map);
            img.remove_seam(&seam)?;
            energy::update_energy_img(&mut energy_map, &img, &seam)?;
        }
        img.transpose();
    }
    Ok(img.to_image())
}
