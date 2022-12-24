fn chr_value(c: char) -> u32 {
    if c.is_uppercase() {
        (c as u32) - ('A' as u32) + 27
    } else {
        (c as u32) - ('a' as u32) + 1
    }
}

pub fn solve(input: &str) -> i32 {
    input
        .split('\n')
        .map(|sack| {
            let mid = sack.len() / 2;
            let (first, second) = sack.split_at(mid);
            for c in first.chars() {
                if second.contains(c) {
                    return chr_value(c);
                }
            }
            0
        })
        .map(|n| n as i32)
        .sum::<i32>()
}
