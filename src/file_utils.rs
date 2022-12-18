use std::path::PathBuf;
use std::fs::{File, symlink_metadata};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

use anyhow::{Context, Result};
use walkdir::WalkDir;
use sha2::{Sha512, Digest};

use crate::file_exclude::FileExclude;

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

pub fn hash_file(file_path: &PathBuf, exclude: Option<&FileExclude>) -> Result<String> {
    let mut hasher = Sha512::new();
    let mut not_text = false;
    let mut line_num = 1;

    if exclude.is_some() {
        debug!("EXCLUDE FOR: {:?}", file_path);
    }

    {
        let file = File::open(file_path.clone())
            .with_context(|| format!("Opening {}", file_path.display()))?;
        let mut file = BufReader::new(file);

        // try to hash the file line-by-line so we can exclude lines if needed
        for line_res in file.lines() {
            if line_res.is_err() {
                not_text = true;
                break;
            }

            // only hash the line if it's NOT in the exclude set
            if exclude.is_none() || (exclude.is_some() && !exclude.unwrap().in_lines(line_num)) {
                hasher.update(line_res.unwrap().as_bytes());
                hasher.update("\n".as_bytes()); // newlines are stripped, so add them back in
            }

            line_num += 1;
        }
    }
    // we tried to read the file as text, but found non-text
    // so we're going to start over hashing it as binary
    if not_text {
        warn!("Found non-text for: {}", file_path.display());

        let buffer = std::fs::read(file_path)
            .with_context(|| format!("Trying to read data-file: {}", file_path.display()))?;

        hasher.update(buffer);
    }

    return Ok(format!("{:x}", hasher.finalize()));
}


#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    use create::file_utils::hash_file;
    use file_exclude::FileExclude;


    #[test]
    pub fn test_hash_file_none() {
        let file_path = String::from("/tmp/test");

        let hash1 = {
            let mut file = File::create(file_path.clone()).expect("Failed to create file");

            file.write("\n".as_bytes()).expect("Error writing to file");

            hash_file(&PathBuf::from(file_path.clone()), None)
        };

        let hash2 = {
            let mut file = File::create(file_path.clone()).expect("Failed to create file");

            file.write("\n\n".as_bytes()).expect("Error writing to file");

            hash_file(&PathBuf::from(file_path.clone()), None)
        };

        assert_ne!(hash1, hash2);
    }

    #[test]
    pub fn test_hash_file_exclude_all() {
        let file_path = String::from("/tmp/test");
        let exclude = FileExclude::new(file_path.clone());

        let hash1 = {
            let mut file = File::create(&file_path).expect("Failed to create file");

            file.write("test".as_bytes()).expect("Error writing to file");

            hash_file(&PathBuf::from(&file_path), Some(&exclude))
        };

        let hash2 = {
            let mut file = File::create(&file_path).expect("Failed to create file");

            file.write("hello world".as_bytes()).expect("Error writing to file");

            hash_file(&PathBuf::from(&file_path), Some(&exclude))
        };

        assert_ne!(hash1, hash2);
    }
}
