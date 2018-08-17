#[macro_use]
extern crate log;
extern crate simplelog;
extern crate clap;
extern crate walkdir;
extern crate crypto;

use simplelog::{TermLogger, LevelFilter, Config};
use std::fs::read_dir;
use std::path::PathBuf;

mod config;
mod file_utils;

use config::Configuration;
use file_utils::{recurse_dir, hash_file};

fn main() {
    TermLogger::init(LevelFilter::Debug, Config::default()).unwrap();

    let config = Configuration::new().unwrap();

    // get all the files we care about
    let files = recurse_dir(config.root_dir);

    for file in files {
        let hash_res = hash_file(file.clone());

        if hash_res.is_none() {
            continue;
        }

        println!("{:?}: {}", file, hash_res.unwrap());
    }
}