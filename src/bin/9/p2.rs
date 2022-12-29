use std::collections::HashSet;

use itertools::Itertools;

const KNOTS: i32 = 10;

fn get_dir(motion: &str) -> (i32, i32) {
    match motion {
        "R" => (1, 0),
        "L" => (-1, 0),
        "D" => (0, 1),
        "U" => (0, -1),
        _ => unreachable!(),
    }
}

pub fn solve(input: &str) -> i32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut positions = [(0, 0); KNOTS as usize];
    visited.insert((0, 0));
    input
        .split('\n')
        .map(|m| {
            let (m, steps) = m.split(' ').next_tuple().unwrap();
            let m = get_dir(m);
            let steps = steps.parse::<i32>().unwrap();
            (m, steps)
        })
        .for_each(|((dx, dy), steps)| {
            for _ in 0..steps {
                positions[0].0 += dx;
                positions[0].1 += dy;
                for i in 1..KNOTS {
                    let (px, py) = positions[(i - 1) as usize];
                    let (cx, cy) = positions[i as usize];
                    match (px.abs_diff(cx), py.abs_diff(cy)) {
                        (2, _) | (_, 2) => {
                            let next_x = if px.abs_diff(cx) == 2 {
                                (px + cx) / 2
                            } else {
                                px
                            };
                            let next_y = if py.abs_diff(cy) == 2 {
                                (py + cy) / 2
                            } else {
                                py
                            };
                            positions[i as usize] = (next_x, next_y);
                        }
                        _ => (),
                    }
                }
                visited.insert(*positions.last().unwrap());
            }
        });

    visited.len() as i32
}
