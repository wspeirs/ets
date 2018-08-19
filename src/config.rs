use clap::{Arg, App, SubCommand};

use std::io::{Read, ErrorKind, Error as IOError};
use std::io::prelude::BufRead;
use std::fs::File;
use std::path::PathBuf;
use yaml_rust::yaml::Yaml;
use yaml_rust::YamlLoader;

use file_exclude::FileExclude;

pub struct Configuration {
    update: bool,
    root_dir: PathBuf,
    data_dir: PathBuf,
    report_dir: PathBuf,
    report_proc: Option<PathBuf>,
    exclude: Option<Vec<FileExclude>>,
    ensure: Option<Vec<PathBuf>>
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

    pub fn root_dir(&self) -> &PathBuf {
        return &self.root_dir;
    }
}

fn load_config_file(path: PathBuf, update: bool) -> Result<Configuration, IOError> {
    let mut file = File::open(path.clone()).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading config file");

    let config_yaml = YamlLoader::load_from_str(&contents).expect("Error parsing Yaml file");
    let config_yaml = &config_yaml[0];

    println!("{:?}", config_yaml);

    let hash_map = config_yaml.as_hash().unwrap();

    let root_dir = hash_map.get(&Yaml::String(String::from("root_dir")));
    let data_dir = hash_map.get(&Yaml::String(String::from("data_dir")));
    let report_dir = hash_map.get(&Yaml::String(String::from("report_dir")));
    let excludes = hash_map.get(&Yaml::String(String::from("exclude")));
    let ensure = hash_map.get(&Yaml::String(String::from("ensure")));

    // make sure all required values are found
    if root_dir.is_none() {
        return Err(IOError::new(ErrorKind::NotFound, "Required parameter root_dir not found in config file"));
    }

    if data_dir.is_none() {
        return Err(IOError::new(ErrorKind::NotFound, "Required parameter data_dir not found in config file"));
    }

    if report_dir.is_none() {
        return Err(IOError::new(ErrorKind::NotFound, "Required parameter report_dir not found in config file"));
    }

    if excludes.is_none() {
        return Err(IOError::new(ErrorKind::NotFound, "Required parameter exclude not found in config file"));
    }

    let excludes = excludes.unwrap().as_vec();

    if excludes.is_none() {
        return Err(IOError::new(ErrorKind::InvalidInput, "Required parameter exclude is not a list"));
    }

    let excludes = excludes.unwrap().into_iter().map(|e| FileExclude::new(e.as_str().unwrap().to_owned())).collect::<Vec<_>>();

    println!("{:?}", excludes);

    return Ok(Configuration {
        update: update,
        root_dir: PathBuf::from(root_dir.unwrap().as_str().unwrap()),
        data_dir: PathBuf::from(data_dir.unwrap().as_str().unwrap()),
        report_dir: PathBuf::from(report_dir.unwrap().as_str().unwrap()),
        report_proc: None,
        exclude: Some(excludes),
        ensure: None,
    })
}