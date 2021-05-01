use std::error::Error;
use std::path::PathBuf;

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

    pub fn get_outfile(&self) -> PathBuf {
        self.infile.with_file_name(format!(
            "{}_carved.{}",
            self.infile.file_stem().unwrap().to_str().unwrap(),
            self.infile.extension().unwrap().to_str().unwrap()
        ))
    }
}
