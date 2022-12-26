use itertools::Itertools;
use std::str::FromStr;

type CharStacks = Vec<Vec<char>>;
struct Stacks(CharStacks);

impl FromStr for Stacks {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stacks: CharStacks = Vec::new();
        for line in s.split('\n').rev().skip(1) {
            for (i, cr) in line.chars().skip(1).step_by(4).enumerate() {
                if cr != ' ' {
                    while stacks.len() <= i {
                        stacks.push(Vec::new());
                    }
                    stacks[i].push(cr);
                }
            }
        }
        Ok(Stacks(stacks))
    }
}

impl Stacks {
    fn move_crates(&mut self, move_str: &str) {
        let mut itr = move_str.split(' ').skip(1).step_by(2);
        let amnt = itr.next().unwrap().parse::<i32>().unwrap();
        let from = itr.next().unwrap().parse::<i32>().unwrap() - 1;
        let to = itr.next().unwrap().parse::<i32>().unwrap() - 1;

        let start = &mut self.0[from as usize];
        let mut removed = start
            .drain((start.len() - amnt as usize)..)
            .collect::<Vec<char>>();

        let end = &mut self.0[to as usize];
        end.append(&mut removed);
    }

    fn get_top_crates(&self) -> String {
        let mut out = String::new();
        for s in &self.0 {
            if let Some(c) = s.last() {
                out.push(*c);
            }
        }
        out
    }
}

pub fn solve(input: &str) -> String {
    let (stacks, moves) = input.split("\n\n").next_tuple().unwrap();
    let mut stacks = stacks.parse::<Stacks>().unwrap();
    for mv in moves.split('\n') {
        stacks.move_crates(mv);
    }
    stacks.get_top_crates()
}
