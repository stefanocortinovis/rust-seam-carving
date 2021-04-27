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
    let img_carved = seamcarve(&img_original, config.new_height, config.new_width)?;
    img_carved.save(config.get_outfile())?;
    Ok(())
}

pub fn seamcarve(
    img: &RgbImage,
    _new_height: u32,
    _new_width: u32,
) -> Result<RgbImage, &'static str> {
    let img = array::Array2d::from_image(img)?;
    energy::get_energy_img(&img)?;
    Ok(img.to_image())
}
