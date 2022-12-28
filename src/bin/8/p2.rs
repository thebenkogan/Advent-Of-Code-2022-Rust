use std::cmp::max;

const DIR: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut out = Vec::new();
    for row in input.split('\n') {
        out.push(
            row.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect(),
        );
    }
    out
}

fn check_bounds(trees: &Vec<Vec<i32>>, (x, y): (i32, i32)) -> bool {
    x >= 0 && x < trees[0].len() as i32 && y >= 0 && y < trees.len() as i32
}

pub fn solve(input: &str) -> i32 {
    let trees = parse_input(input);
    let mut best_scenic_score = 0;

    for (i, row) in trees.iter().enumerate() {
        for (j, &tree) in row.iter().enumerate() {
            let mut scenic_score = 1;
            for (dx, dy) in DIR.iter() {
                let mut pos = (j as i32 + dx, i as i32 + dy);
                let mut num_visible = 0;
                while check_bounds(&trees, pos) {
                    if trees[pos.1 as usize][pos.0 as usize] >= tree {
                        num_visible += 1;
                        break;
                    }
                    num_visible += 1;
                    pos = (pos.0 + dx, pos.1 + dy);
                }

                scenic_score *= num_visible;
            }
            best_scenic_score = max(best_scenic_score, scenic_score);
        }
    }

    best_scenic_score
}
