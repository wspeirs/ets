use std::collections::{HashMap, HashSet};
use std::io::{Read, ErrorKind, Error as IOError};
use std::fs::File;
use std::path::PathBuf;

use anyhow::{Context, Error, Result};
use serde::{Serialize, Deserialize};
use serde_json::{from_reader, to_string_pretty};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Report  {
    matches: HashMap<String, String>,
    changed: HashMap<String, String>,
    missing: Vec<String>,
    errors: HashMap<String, String>
}

/// Reads in the data file (JSON), computes the report, and returns the report
/// Returns the report in JSON
pub fn compute_report(data_file: PathBuf, hashes: HashMap<String, String>, errors: HashMap<String, String>) -> Result<String> {
    let file = File::open(&data_file)
        .with_context(|| format!("Attempting to open {}", data_file.display()))?;

    let database :HashMap<String, String> = from_reader(file)
        .with_context(|| format!("Trying to read JSON file {}", data_file.display()))?;

    let mut matches :HashMap<String, String> = HashMap::with_capacity(database.len());
    let mut changed :HashMap<String, String> = HashMap::with_capacity(database.len());

    for (file, hash) in hashes.iter() {
        if let Some(ref v) = database.get(file) {
            if *hash == **v {
                matches.insert(file.to_string(), hash.to_string());
            } else {
                changed.insert(file.to_string(), hash.to_string());
            }
        }
    }

    // compute the missing files
    let database_keys = database.keys().cloned().collect::<HashSet<String>>();
    let hashes_keys = hashes.keys().cloned().collect::<HashSet<String>>();
    let missing = database_keys.difference(&hashes_keys).cloned().collect::<Vec<String>>();

    let report = Report{matches, changed, missing, errors};

    return Ok(to_string_pretty(&report).unwrap());
}