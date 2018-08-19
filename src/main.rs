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
extern crate chrono;

use std::io::Error as IOError;
use std::io::Write;
use std::process::exit;
use std::collections::HashMap;
use std::fs::File;

use simplelog::{TermLogger, LevelFilter, Config};
use chrono::Local;

mod config;
mod file_utils;
mod range_set;
mod file_exclude;
mod report;

use config::Configuration;
use file_utils::{recurse_dir, hash_file};
use report::compute_report;

fn main() -> Result<(), IOError> {
    TermLogger::init(LevelFilter::Debug, Config::default()).unwrap();

    let config = Configuration::new();

    if let Err(e) = config {
        eprintln!("Error: {}", e);
        exit(1);
    }

    let config = config.unwrap();

    // get all the files we care about
    let files = recurse_dir(config.root_dir());

    debug!("BEFORE: {}", files.len());

    // filter out any files that match exclusions without lines
    // unfortunately this algorithm is O(n * m) :-|
    let files = files
        .into_iter()
        .filter(|f| {
            for exclude in config.excludes().iter().filter(|e| !e.has_lines()) {
                if exclude.matches_file(f) {
                    return false;  // we matched an exclude without lines, so filter it out
                }
            }

            return true; // didn't find an exclude, so keep it
        }).collect::<Vec<_>>();

    debug!("AFTER: {}", files.len());

    // compute all the hashes
    let res = files.iter().map(|file| {
        let exclude = config.excludes().iter().find(|e| e.matches_file(file));

        let hash = hash_file(file, exclude);

        (file.to_str().unwrap().to_owned(), hash)
    }).collect::<Vec<_>>();

    let hashes = res.iter().filter(|r| r.1.is_ok()).cloned().map(|r| (r.0, r.1.ok().unwrap())).collect::<HashMap<_,_>>();
    let errors = res.iter().filter(|r| r.1.is_err()).cloned().map(|r| (r.0, r.1.err().unwrap())).collect::<HashMap<_,_>>();


    if config.update() {
        let file = File::create(config.data_file().clone())?;

        serde_json::to_writer_pretty(file, &hashes)?;

        for (file, error) in errors.iter() {
            println!("Error computing hash for {}: {}", file, error);
        }
    } else {
        let now = Local::now().format("%Y%m%d%H%M%S").to_string();
        let report_name = format!("{}/ets_report_{}.json", config.report_dir().to_str().unwrap(), now);
        println!("REPORT NAME: {}", report_name);

        let mut file = File::create(report_name)?;

        let report = compute_report(config.data_file().clone(), hashes, errors)?;

        file.write(report.as_bytes())?;
    }

    Ok( () )
}
