use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq)]
pub struct Config {
    pub infile: PathBuf,
    pub new_width: u32,
    pub new_height: u32,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, Box<dyn Error>> {
        if args.len() < 4 {
            Err("Usage: rsc /path/to/img new_width new_height".into())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new() {
        let args = [
            String::from("rsc"),
            String::from("./img/example_path.png"),
            String::from("100"),
            String::from("42"),
        ];
        assert!(Config::new(&args).is_ok());
    }

    #[test]
    fn config_error() {
        let args = [
            String::from("rsc"),
            String::from("./img/example_path.png"),
            String::from("100"),
        ];
        assert_eq!(
            Err(String::from("Usage: rsc /path/to/img new_width new_height")),
            Config::new(&args).map_err(|e| format!("{}", e))
        );
    }

    #[test]
    fn outfile() {
        let args = [
            String::from("rsc"),
            String::from("./img/example_path.png"),
            String::from("100"),
            String::from("42"),
        ];
        let config = Config::new(&args).unwrap();
        assert_eq!(
            PathBuf::from("./img/example_path_carved.png"),
            config.get_outfile()
        );
    }
}
