use std::error::Error;
use std::path::PathBuf;

use image::io::Reader as ImageReader;
use image::RgbImage;

pub mod array;
pub mod energy;
pub mod seam;

pub struct Config {
    pub infile: PathBuf,
    pub new_width: u32,
    pub new_height: u32,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, Box<dyn Error>> {
        if args.len() < 4 {
            Err("Usage: rsc /path/to/img.jpg new_height new_width".into())
        } else {
            let infile: PathBuf = args[1].parse()?;
            let new_width: u32 = args[2].parse()?;
            let new_height: u32 = args[3].parse()?;

            Ok(Self {
                infile,
                new_width,
                new_height,
            })
        }
    }

    fn get_outfile(&self) -> PathBuf {
        self.infile.with_file_name(format!(
            "{}_carved.{}",
            self.infile.file_stem().unwrap().to_str().unwrap(),
            self.infile.extension().unwrap().to_str().unwrap()
        ))
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let img_original = ImageReader::open(&config.infile)?.decode()?.to_rgb8();
    let img_carved = seamcarve(&img_original, config.new_width, config.new_height)?;
    img_carved.save(config.get_outfile())?;
    Ok(())
}

pub fn seamcarve(
    img: &RgbImage,
    new_width: u32,
    _new_height: u32,
) -> Result<RgbImage, &'static str> {
    let width = img.dimensions().0;
    let vertical_to_remove = width - new_width;
    let mut img = array::Array2d::from_image(img)?;
    let mut energy_map = energy::get_energy_img(&img)?;
    for _ in 0..vertical_to_remove {
        let seam = seam::find_vertical_seam(&energy_map);
        img.remove_seam(&seam)?;
        energy::update_energy_img(&mut energy_map, &img, &seam)?;
    }
    Ok(img.to_image())
}
