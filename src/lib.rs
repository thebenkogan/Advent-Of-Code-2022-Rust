use std::{
    env,
    fmt::Display,
    fs,
    time::{Duration, Instant},
};

pub mod unique_counter;

pub fn run_day<T: Display>(day: i32, p1: fn(&str) -> T, p2: fn(&str) -> T) {
    let (input, bench) = get_input(day);
    if bench {
        benchmark_parts(p1, p2, &input)
    } else {
        run_parts(p1, p2, &input)
    };
}

fn get_input(day: i32) -> (String, bool) {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let bench = args.contains(&"bench".to_string());
    let test = args.contains(&"test".to_string());
    let input_file = if test { "test" } else { "in" };
    let path = format!("src/bin/{day}/{input_file}.txt");
    let input = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("file {path} not found"))
        .strip_suffix('\n')
        .unwrap()
        .to_string();
    (input, bench)
}

fn run_parts<T: Display>(p1: fn(&str) -> T, p2: fn(&str) -> T, input: &str) {
    println!("Part 1: {}", p1(input));
    println!("Part 2: {}", p2(input));
}

const NUM_TRIALS: u32 = 100;

fn benchmark_parts<T>(p1: fn(&str) -> T, p2: fn(&str) -> T, input: &str) {
    println!("Part 1 Time: {:?}", benchmark_part(p1, input));
    println!("Part 2 Time: {:?}", benchmark_part(p2, input));
}

fn benchmark_part<T>(p: fn(&str) -> T, input: &str) -> Duration {
    let mut total = Duration::ZERO;
    for _ in 0..NUM_TRIALS {
        let start = Instant::now();
        p(input);
        let elapsed = start.elapsed();
        total += elapsed
    }
    total / NUM_TRIALS
}
