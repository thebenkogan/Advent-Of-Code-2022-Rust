pub fn solve(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|n| n.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap()
}
