use std::path::PathBuf;
use range_set::{RangeSetBuilder, RangeSet};

#[derive(Debug)]
pub struct FileExclude {
    file: PathBuf,
    lines: Option<RangeSet>
}

impl FileExclude {
    pub fn new(string: String) -> FileExclude {
        let fields = string.split(":").collect::<Vec<_>>();

        let file = PathBuf::from(fields[0]);

        if fields.len() == 2 {
            let mut rsb = RangeSetBuilder::new();

            for range in fields[1].split(",").collect::<Vec<_>>() {
                if range.contains("-") {
                    let ranges = range.split("-").collect::<Vec<_>>();
                    let start = ranges[0].parse::<usize>().expect(&format!("Error parsing start line: {:?}", ranges));
                    let end = ranges[1].parse::<usize>().expect(&format!("Error parsing end line: {:?}", ranges));

                    rsb.add_range(start..(end+1));
                } else {
                    rsb.add(range.parse::<usize>().expect(&format!("Error parsing line: {:?}", range)));
                }
            }

            return FileExclude{file, lines: Some(rsb.build())};
        } else {
            return FileExclude{file, lines: None};
        }
    }

    pub fn file(&self) -> &PathBuf {
        &self.file
    }

    pub fn has_lines(&self) -> bool {
        self.lines.is_some()
    }

    pub fn matches_file(&self, file: PathBuf) -> bool {
        true
    }
}