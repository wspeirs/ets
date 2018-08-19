use std::path::PathBuf;
use std::fs::{File, symlink_metadata};
use std::io::{BufRead, BufReader, Seek, SeekFrom, Error as IOError};

use walkdir::WalkDir;
use sha2::{Sha512, Digest};

use file_exclude::FileExclude;

pub fn recurse_dir(path: &PathBuf) -> Vec<PathBuf> {
    let mut files = Vec::<PathBuf>::new();

    // recurse through the directory skipping any errors silently
    // this is OK because if we should be able to access the file,
    // it'll show up in the report as an error
    for entry in WalkDir::new(path).follow_links(true).into_iter().filter_map(|e| e.ok()) {
        // filter out the directories, because we only care about the files
        if entry.path().is_file() && !symlink_metadata(entry.path()).unwrap().file_type().is_symlink() {
            files.push(entry.path().to_owned());
        }
    }

    return files;
}

pub fn hash_file(file_path: &PathBuf, exclude: Option<&FileExclude>) -> Result<String, String> {
    let mut hasher = Sha512::new();

    let file_res = File::open(file_path.clone());

    if let Err(e) = file_res {
        return Err(format!("{}", e));
    }

    let mut file = file_res.unwrap();
    let mut not_text = false;
    let mut line_num = 1;

    // try to hash the file line-by-line so we can exclude lines if needed
    for line_res in BufReader::new(file.try_clone().unwrap()).lines() {
        if line_res.is_err() {
            not_text = true;
            break;
        }

        // only hash the line if it's NOT in the exclude set
        if exclude.is_none() || (exclude.is_some() && !exclude.unwrap().in_lines(line_num)) {
            hasher.input(line_res.unwrap().as_bytes());
        }

        line_num += 1;
    }

    // we tried to read the file as text, but found non-text
    // so we're going to start over hashing it as binary
    if not_text {
        file.seek(SeekFrom::Start(0));

        let hash_res = Sha512::digest_reader(&mut file);

        if let Err(e) = hash_res {
            return Err(format!("{}", e));
        }

        return Ok(format!("{:x}", hash_res.unwrap()));
    } else {
        let hash = hasher.result();

        return Ok(format!("{:x}", hash));
    }
}