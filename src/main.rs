#[macro_use]
extern crate log;
extern crate simplelog;
extern crate clap;
extern crate walkdir;
extern crate sha2;
extern crate glob;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;
extern crate serde_json;

use simplelog::{TermLogger, LevelFilter, Config};
use std::path::PathBuf;
use std::process::exit;
use std::collections::HashMap;

mod config;
mod file_utils;
mod range_set;
mod file_exclude;
//mod data;

use config::Configuration;
use file_utils::{recurse_dir, hash_file};
use file_exclude::FileExclude;

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

//    for file in files.iter() {
//        println!("{:?}", file);
//    }

    println!("BEFORE: {}", files.len());



    // filter out any files that match exclusions without lines
    // unfortunately this algorithm is O(n * m) :-|
    let files = files
        .into_iter()
        .filter(|f| {
            for exclude in config.excludes().iter().filter(|e| !e.has_lines()) {
                if exclude.matches_file(f) {
//                    println!("MATCHED: {:?}", f);
                    return false;  // we matched an exclude without lines, so filter it out
                }
            }

            return true; // didn't find an exclude, so keep it
        }).collect::<Vec<_>>();

    println!("AFTER: {}", files.len());

    let res = files.iter().map(|file| {
        let exclude = config.excludes().iter().find(|e| e.matches_file(file));

        let hash = hash_file(file, exclude);

        (file.to_str().unwrap().to_owned(), hash)
    });

    let data = res.clone().filter(|r| r.1.is_ok()).map(|r| (r.0, r.1.ok().unwrap())).collect::<HashMap<_,_>>();
    let errors = res.clone().filter(|r| r.1.is_err()).map(|r| (r.0, r.1.err().unwrap())).collect::<Vec<_>>();

    let json = serde_json::to_string_pretty(&data).unwrap();

//    println!("{}", json);

    for error in errors {
        println!("Error computing hash for {}: {}", error.0, error.1);
    }
}
