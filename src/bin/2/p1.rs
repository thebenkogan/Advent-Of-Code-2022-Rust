use itertools::Itertools;
use std::str::FromStr;

#[derive(PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn value(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn outcome_score(us: &Self, opp: &Self) -> i32 {
        match (us, opp) {
            (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper)
            | (Self::Rock, Self::Scissors) => 6,
            (us, opp) if us == opp => 3,
            _ => 0,
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

pub fn solve(input: &str) -> i32 {
    input
        .split('\n')
        .map(|round| {
            let (opp, us) = round
                .split(' ')
                .map(|hand| hand.parse::<Hand>().unwrap())
                .next_tuple()
                .unwrap();

            us.value() + Hand::outcome_score(&us, &opp)
        })
        .sum()
}
