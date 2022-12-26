use itertools::Itertools;
use std::str::FromStr;

struct Range {
    min: i32,
    max: i32,
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (low_str, max_str) = s.split('-').next_tuple().unwrap();
        Ok(Self {
            min: low_str.parse::<i32>().unwrap(),
            max: max_str.parse::<i32>().unwrap(),
        })
    }
}

impl Range {
    fn overlaps(&self, other: &Self) -> bool {
        self.min >= other.min && self.min <= other.max
            || other.min >= self.min && other.min <= self.max
    }
}

pub fn solve(input: &str) -> i32 {
    input
        .split('\n')
        .map(|pair| {
            let (left, right) = pair.split(',').next_tuple().unwrap();
            let left = left.parse::<Range>().unwrap();
            let right = right.parse::<Range>().unwrap();
            i32::from(left.overlaps(&right))
        })
        .sum()
}
