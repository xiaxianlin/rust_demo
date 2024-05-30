pub mod lib;
use crate::minigrep::lib::{exec, Config};
use std::{env, process};

pub fn run() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing argumentss: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = exec(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
