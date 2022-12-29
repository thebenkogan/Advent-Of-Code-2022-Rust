use std::collections::HashSet;

use itertools::Itertools;

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
    let (mut hx, mut hy) = (0, 0); // head position
    let (mut tx, mut ty) = (0, 0); // tail position
    visited.insert((tx, ty));
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
                hx += dx;
                hy += dy;
                match (hx.abs_diff(tx), hy.abs_diff(ty)) {
                    (2, _) | (_, 2) => {
                        tx = hx - dx;
                        ty = hy - dy;
                        visited.insert((tx, ty));
                    }
                    _ => (),
                }
            }
        });

    visited.len() as i32
}
