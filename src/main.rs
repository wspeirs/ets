#[macro_use]
extern crate log;
extern crate simplelog;
extern crate clap;
extern crate walkdir;
extern crate sha2;
extern crate yaml_rust;

use simplelog::{TermLogger, LevelFilter, Config};
use std::path::PathBuf;
use std::process::exit;

mod config;
mod file_utils;
mod range_set;
mod file_exclude;

use config::Configuration;
use file_utils::{recurse_dir, hash_file};

fn main() {
    TermLogger::init(LevelFilter::Debug, Config::default()).unwrap();

    let config = Configuration::new();

    if let Err(e) = config {
        eprintln!("Error: {}", e);
        exit(1);
    }

    let config = config.unwrap();

    // get all the files we care about
    let files = recurse_dir(config.root_dir());

    println!("BEFORE: {}", files.len());

    // remove all the files that are excluded without file ranges
    let files = if let Some(excludes) = config.excludes() {
        files.into_iter().filter(|f| {
            // TODO: change this to binary_search_by and use glob
            if let Ok(i) = excludes.binary_search_by_key(f, |fe| fe.file().to_owned()) {
                !excludes[i].has_lines()
            } else {
                true
            }
        }).collect::<Vec<_>>()
    } else {
        files
    };

    println!("AFTER: {}", files.len());


//    for file in files {
//        let hash = hash_file(file.clone());
//
//        println!("{:?}: {}", file, hash);
//    }
}
