use aoc2022::get_input;

mod p1;
mod p2;

const DAY: i32 = 2;

fn main() {
    let input = get_input(DAY);

    println!("Part 1: {}", p1::solve(&input));
    println!("Part 2: {}", p2::solve(&input));
}