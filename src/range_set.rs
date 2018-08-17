use std::cmp::Ordering;
use std::ops::Range;

pub struct RangeSet {
    ranges: Vec<Range<usize>>
}

pub struct RangeSetBuilder {
    range_set: RangeSet
}


impl RangeSetBuilder {
    pub fn new() -> RangeSetBuilder {
        let range_set = RangeSet{ ranges: vec![] };

        return RangeSetBuilder{ range_set };
    }

    pub fn add_range(&mut self, range: Range<usize>) -> &mut RangeSetBuilder {
        self.range_set.ranges.push(range);
        self
    }

    pub fn add(&mut self, num: usize) -> &mut RangeSetBuilder {
        self.range_set.ranges.push(num..(num+1));
        self
    }

    pub fn build(&mut self) -> &RangeSet {
        // sort the ranges so it's faster to search
        self.range_set.ranges.sort_by(|a, b| a.start.cmp(&b.start));
        &self.range_set
    }
}

impl RangeSet {
    pub fn contains(&self, num: usize) -> bool {
        let res = self.ranges.binary_search_by(|r| {
            if num < r.start {
                return Ordering::Less;
            } else if r.start <= num && num < r.end {
                return Ordering::Equal;
            } else {
                return Ordering::Greater;
            }
        });

        res.is_ok()
    }
}