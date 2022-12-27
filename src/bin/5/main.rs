use aoc2022::run_day;

mod p1;
mod p2;

const DAY: i32 = 5;

fn main() {
    run_day(DAY, p1::solve, p2::solve);
}
