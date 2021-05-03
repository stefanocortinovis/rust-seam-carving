use std::{env, process};

use rsc::parser;

#[cfg(not(tarpaulin_include))]
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parser::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    if let Err(e) = rsc::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
