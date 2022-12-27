use std::collections::VecDeque;

use itertools::Itertools;

const WINDOW_SIZE: usize = 4;

pub fn solve(input: &str) -> i32 {
    let mut window: VecDeque<char> = VecDeque::with_capacity(WINDOW_SIZE);
    for (i, c) in input.chars().enumerate() {
        if window.len() < WINDOW_SIZE {
            window.push_front(c);
            continue;
        }

        if window.iter().unique().count() == window.len() {
            return i as i32;
        }

        window.pop_back();
        window.push_front(c);
    }

    unreachable!()
}
