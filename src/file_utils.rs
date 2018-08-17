use std::path::PathBuf;
use std::fs::{File, symlink_metadata};
use std::io::{BufRead, BufReader, Error as IOError};

use walkdir::WalkDir;
use crypto::sha2::Sha512;
use crypto::digest::Digest;

pub fn recurse_dir(path: PathBuf) -> Vec<PathBuf> {
    let mut files = Vec::<PathBuf>::new();

    // recurse through the directory skipping any errors silently
    // this is OK because if we should be able to access the file,
    // it'll show up in the report as an error
    for entry in WalkDir::new(path).follow_links(false).into_iter().filter_map(|e| e.ok()) {
        // filter out the directories, because we only care about the files
        if entry.path().is_file() && !symlink_metadata(entry.path()).unwrap().file_type().is_symlink() {
            files.push(entry.path().to_owned());
        }
    }

    return files;
}

pub fn hash_file(file_path: PathBuf) -> Option<String> {
    let mut hasher = Sha512::new();

    let file_res = File::open(file_path.clone());

    if file_res.is_err() {
        return None;
    }

    for line in BufReader::new(file_res.unwrap()).lines() {
        hasher.input_str(line.expect(&format!("ERR: {:?}", file_path.clone())).as_str());
    }

    return Some( hasher.result_str() );
}