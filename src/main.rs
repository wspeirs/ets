#[macro_use]
extern crate log;
extern crate simplelog;
extern crate clap;

use simplelog::{TermLogger, LevelFilter, Config};

mod config;

use config::Configuration;

fn main() {
    TermLogger::init(LevelFilter::Debug, Config::default()).unwrap();

    Configuration::new();
}
