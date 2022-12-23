pub fn solve(input: &str) -> i32 {
    let mut sums: Vec<i32> = input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|n| n.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect();

    sums.sort_by(|a, b| b.cmp(a));
    sums.iter().take(3).sum()
}
