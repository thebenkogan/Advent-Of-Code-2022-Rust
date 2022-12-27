use std::collections::VecDeque;

use aoc2022::unique_counter::UniqCounter;

const WINDOW_SIZE: usize = 14;

pub fn solve(input: &str) -> i32 {
    let mut counter = UniqCounter::new(WINDOW_SIZE);
    let mut window: VecDeque<char> = VecDeque::with_capacity(WINDOW_SIZE);
    for (i, c) in input.chars().enumerate() {
        if i < WINDOW_SIZE {
            counter.add(c);
            window.push_front(c);
            continue;
        }

        if counter.is_unique() {
            return i as i32;
        }

        if let Some(removed) = window.pop_back() {
            counter.remove(removed);
        }
        window.push_front(c);
        counter.add(c);
    }

    unreachable!()
}
