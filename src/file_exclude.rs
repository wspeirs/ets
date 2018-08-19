use std::path::PathBuf;
use range_set::{RangeSetBuilder, RangeSet};
use glob::Pattern;

#[derive(Debug)]
pub struct FileExclude {
    file_pattern: Pattern,
    lines: Option<RangeSet>
}

impl FileExclude {
    pub fn new(string: String) -> FileExclude {
        let fields = string.split(":").collect::<Vec<_>>();

        let file_pattern = Pattern::new(fields[0]).expect(&format!("Invalid glob pattern: {}", fields[0]));

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

            return FileExclude{ file_pattern: file_pattern, lines: Some(rsb.build())};
        } else {
            return FileExclude{ file_pattern: file_pattern, lines: None};
        }
    }

    pub fn has_lines(&self) -> bool {
        self.lines.is_some()
    }

    pub fn in_lines(&self, line: usize) -> bool {
        if !self.has_lines() {
            return true; // with no lines, we exclude all of them
        } else {
            return self.lines.as_ref().unwrap().contains(line);
        }
    }

    pub fn matches_file(&self, file: &PathBuf) -> bool {
        self.file_pattern.matches(file.to_str().unwrap())
    }
}