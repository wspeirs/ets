use std::cmp::Ordering;
use std::ops::Range;

#[derive(Debug)]
pub struct RangeSet {
    ranges: Vec<Range<usize>>
}

pub struct RangeSetBuilder {
    range_set: Vec<Range<usize>>
}


impl RangeSetBuilder {
    pub fn new() -> RangeSetBuilder {
        return RangeSetBuilder{ range_set: vec![] };
    }

    pub fn add_range(&mut self, range: Range<usize>) -> &mut RangeSetBuilder {
        self.range_set.push(range);
        self
    }

    pub fn add(&mut self, num: usize) -> &mut RangeSetBuilder {
        self.range_set.push(num..(num+1));
        self
    }

    pub fn build(&mut self) -> RangeSet {
        // sort the ranges so it's faster to search
        self.range_set.sort_by(|a, b| a.start.cmp(&b.start));
        return RangeSet{ranges: self.range_set.clone()};
    }
}

impl RangeSet {
    pub fn contains(&self, num: usize) -> bool {
        let res = self.ranges.binary_search_by(|r| {
            if num < r.start {
                return Ordering::Greater;
            } else if r.start <= num && num < r.end {
                return Ordering::Equal;
            } else {
                return Ordering::Less;
            }
        });

        res.is_ok()
    }
}


#[cfg(test)]
mod test {
    use crate::range_set::RangeSetBuilder;

    #[test]
    pub fn test_create_empty() {
        let mut rsb = RangeSetBuilder::new();
        let _ = rsb.build();
    }

    #[test]
    pub fn test_add() {
        let mut rsb = RangeSetBuilder::new();

        rsb.add(7);

        let rs = rsb.build();

        assert!(!rs.contains(6));
        assert!(rs.contains(7));
        assert!(!rs.contains(8));
    }

    #[test]
    pub fn test_add_separate() {
        let mut rsb = RangeSetBuilder::new();

        rsb.add(7);
        rsb.add(9);

        let rs = rsb.build();

        assert!(!rs.contains(6));
        assert!(rs.contains(7));
        assert!(!rs.contains(8));
        assert!(rs.contains(9));
        assert!(!rs.contains(10));
    }

    #[test]
    pub fn test_add_together() {
        let mut rsb = RangeSetBuilder::new();

        rsb.add(7);
        rsb.add(8);

        let rs = rsb.build();

        assert!(!rs.contains(6));
        assert!(rs.contains(7));
        assert!(rs.contains(8));
        assert!(!rs.contains(9));
    }

    #[test]
    pub fn test_add_range() {
        let mut rsb = RangeSetBuilder::new();

        rsb.add_range(1..3);

        let rs = rsb.build();

        assert!(!rs.contains(0));
        assert!(rs.contains(1));
        assert!(rs.contains(2));
        assert!(!rs.contains(3));
    }

    #[test]
    pub fn test_add_range_separate() {
        let mut rsb = RangeSetBuilder::new();

        rsb.add_range(1..3);
        rsb.add_range(4..6);

        let rs = rsb.build();

        assert!(!rs.contains(0));
        assert!(rs.contains(1));
        assert!(rs.contains(2));
        assert!(!rs.contains(3));
        assert!(rs.contains(4));
        assert!(rs.contains(5));
        assert!(!rs.contains(6));
    }

    #[test]
    pub fn test_add_range_together() {
        let mut rsb = RangeSetBuilder::new();

        rsb.add_range(1..3);
        rsb.add_range(3..5);

        let rs = rsb.build();

        assert!(!rs.contains(0));
        assert!(rs.contains(1));
        assert!(rs.contains(2));
        assert!(rs.contains(3));
        assert!(rs.contains(4));
        assert!(!rs.contains(5));
    }

    #[test]
    pub fn test_mixed() {
        let mut rsb = RangeSetBuilder::new();

        rsb.add_range(1..3);
        rsb.add(4);

        let rs = rsb.build();

        assert!(!rs.contains(0));
        assert!(rs.contains(1));
        assert!(rs.contains(2));
        assert!(!rs.contains(3));
        assert!(rs.contains(4));
        assert!(!rs.contains(5));
    }

}