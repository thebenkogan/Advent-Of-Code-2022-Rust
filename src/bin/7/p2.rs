use std::collections::HashMap;

const DISK_SPACE: i32 = 70000000;
const THRESHOLD: i32 = 30000000;

pub fn solve(input: &str) -> i32 {
    let mut file_stack = Vec::new();
    let mut file_sizes: HashMap<String, i32> = HashMap::new();
    for cc in input.split('\n') {
        let splitted = cc.split(' ').collect::<Vec<&str>>();
        let size_res = splitted[0].parse::<i32>();
        match splitted[..] {
            ["$", "cd", ".."] => {
                file_stack.pop();
            }
            ["$", "cd", dir] => {
                file_stack.push(dir);
            }
            [_, _] if size_res.is_ok() => {
                let size = size_res.unwrap();
                let mut file_name = String::new();
                for file in &file_stack {
                    file_name.push_str(file);
                    *file_sizes.entry(file_name.clone()).or_default() += size;
                }
            }
            _ => continue,
        }
    }
    let remaining = THRESHOLD - (DISK_SPACE - file_sizes.get("/").unwrap());
    file_sizes
        .into_values()
        .filter(|s| *s >= remaining)
        .min()
        .unwrap()
}
