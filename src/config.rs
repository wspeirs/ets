use clap::{Arg, App};

use std::io::{Error as IOError};
use std::fs::File;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use serde_yaml::from_reader;

use file_exclude::FileExclude;

pub struct Configuration {
    update: bool,
    root_dir: PathBuf,
    data_file: PathBuf,
    report_dir: PathBuf,
    report_proc: Option<PathBuf>,
    excludes: Vec<FileExclude>,
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

        let config_file = matches.value_of("config").unwrap_or("config_test.yaml");
        let update = matches.is_present("update");

        debug!("Using config file: {}", config_file);
        if update {
            debug!("We are updating the database");
        }

        return load_config_file(PathBuf::from(config_file), update);
    }

    pub fn update(&self) -> bool {
        self.update
    }

    pub fn root_dir(&self) -> &PathBuf {
        &self.root_dir
    }

    pub fn data_file(&self) -> &PathBuf {
        &self.data_file
    }

    pub fn report_dir(&self) -> &PathBuf {
        &self.report_dir
    }

    pub fn report_proc(&self) -> &Option<PathBuf> {
        &self.report_proc
    }

    pub fn excludes(&self) -> &Vec<FileExclude> {
        &self.excludes
    }

}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigFile {
    root_dir: PathBuf,
    data_file: PathBuf,
    report_dir: PathBuf,
    report_proc: Option<PathBuf>,
    exclude: Option<Vec<String>>,
}

fn load_config_file(path: PathBuf, update: bool) -> Result<Configuration, IOError> {
    let file = File::open(path.clone())?;

    let config_file: ConfigFile = from_reader(file).unwrap();

    let excludes = if let Some(excludes_strs) = config_file.exclude {
        excludes_strs
        .into_iter()
        .map(|e| FileExclude::new(e.to_owned()))
        .collect::<Vec<_>>()
    } else {
        vec![]
    };

    debug!("{:?}", excludes);

    return Ok(Configuration {
        update: update,
        root_dir: config_file.root_dir,
        data_file: config_file.data_file,
        report_dir: config_file.report_dir,
        report_proc: None,
        excludes: excludes
    })
}