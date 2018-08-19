#[macro_use]
extern crate log;
extern crate simplelog;
extern crate clap;
extern crate walkdir;
extern crate sha2;
extern crate yaml_rust;

use simplelog::{TermLogger, LevelFilter, Config};
use std::path::PathBuf;

mod config;
mod file_utils;
mod range_set;
mod file_exclude;

use config::Configuration;
use file_utils::{recurse_dir, hash_file};

fn main() {
    TermLogger::init(LevelFilter::Debug, Config::default()).unwrap();

    let config = Configuration::new().unwrap();

    // get all the files we care about
    let files = recurse_dir(config.root_dir());

//    for file in files {
//        let hash = hash_file(file.clone());
//
//        println!("{:?}: {}", file, hash);
//    }
}
