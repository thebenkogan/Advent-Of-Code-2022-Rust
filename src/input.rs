use std::{env, fs};

pub fn get_input(day: i32) -> String {
    let arg = env::args().last().unwrap();
    let input_file = if arg == "test" { "test" } else { "in" };
    let path = format!("src/bin/{day}/{input_file}.txt");
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("file {path} not found"))
}
