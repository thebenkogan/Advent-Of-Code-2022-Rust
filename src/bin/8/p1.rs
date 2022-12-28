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
    let mut count = 0;

    for (i, row) in trees.iter().enumerate() {
        for (j, &tree) in row.iter().enumerate() {
            for (dx, dy) in DIR.iter() {
                let mut pos = (j as i32 + dx, i as i32 + dy);
                let mut visible = true;
                while check_bounds(&trees, pos) {
                    if trees[pos.1 as usize][pos.0 as usize] >= tree {
                        visible = false;
                        break;
                    }
                    pos = (pos.0 + dx, pos.1 + dy);
                }

                if visible {
                    count += 1;
                    break;
                }
            }
        }
    }

    count
}
