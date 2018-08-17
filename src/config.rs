use clap::{Arg, App, SubCommand};

use std::io::Error as IOError;
use std::path::PathBuf;


pub struct Configuration {
    update: bool,
    pub root_dir: PathBuf,
    data_dir: PathBuf,
    report_dir: PathBuf,
    report_proc: Option<PathBuf>,

}


impl Configuration {
    pub fn new() -> Result<Configuration, IOError> {
        let matches = App::new("ets")
            .version("1.0")
            .author("William Speirs <bill.speirs@gmail.com>")
            .about("Ensures files are the same")
            .arg(Arg::with_name("update")
                .long("update")
                .help("Update the file database replacing it's contents"))
            .arg(Arg::with_name("CONFIG")
                .help("Specifies the configuration file to use")
                .long("config")
                .takes_value(true))
            .arg(Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"))
            .get_matches();

        let config_file = matches.value_of("config").unwrap_or("ets.config");
        let update = matches.is_present("update");

        debug!("Using config file: {}", config_file);

        if update {
            debug!("We are updating the database");
        }

        return Ok(Configuration {
            update: update,
            root_dir: PathBuf::from("/etc"),
            data_dir: PathBuf::from("/tmp/ets_data"),
            report_dir: PathBuf::from("/tmp/ets_report"),
            report_proc: None
        });
    }
}