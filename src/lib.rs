use std::{env, fmt::Display, fs, time::Instant};

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

fn benchmark_parts<T>(p1: fn(&str) -> T, p2: fn(&str) -> T, input: &str) {
    let p1_start = Instant::now();
    p1(input);
    println!("Part 1 Time: {:?}", p1_start.elapsed());

    let p2_start = Instant::now();
    p2(input);
    println!("Part 2 Time: {:?}", p2_start.elapsed());
}
