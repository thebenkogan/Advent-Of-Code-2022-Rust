use std::collections::HashSet;

use itertools::Itertools;

fn chr_value(c: char) -> u32 {
    if c.is_uppercase() {
        (c as u32) - ('A' as u32) + 27
    } else {
        (c as u32) - ('a' as u32) + 1
    }
}

pub fn solve(input: &str) -> i32 {
    input
        .split('\n')
        .chunks(3)
        .into_iter()
        .map(|sacks| {
            let mut sets = sacks.map(|sack| HashSet::from_iter(sack.chars()));
            let mut start: HashSet<char> = sets.next().unwrap();
            for set in sets {
                start.retain(|e| set.contains(e));
            }
            let c = start.drain().next().unwrap();
            chr_value(c)
        })
        .map(|n| n as i32)
        .sum::<i32>()
}
