use itertools::Itertools;
use std::str::FromStr;

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn value(&self) -> i32 {
        match self {
            Self::Win => 6,
            Self::Lose => 0,
            Self::Draw => 3,
        }
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Debug)]
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

    // returns the hand to play to get the [outcome] against this hand
    fn other_hand(&self, outcome: &Outcome) -> Self {
        match (self, outcome) {
            (Self::Scissors, Outcome::Draw)
            | (Self::Paper, Outcome::Win)
            | (Self::Rock, Outcome::Lose) => Self::Scissors,
            (Self::Rock, Outcome::Draw)
            | (Self::Paper, Outcome::Lose)
            | (Self::Scissors, Outcome::Win) => Self::Rock,
            _ => Self::Paper,
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
            let (opp, outcome) = round.split(' ').next_tuple().unwrap();
            let opp = opp.parse::<Hand>().unwrap();
            let outcome = outcome.parse::<Outcome>().unwrap();

            outcome.value() + opp.other_hand(&outcome).value()
        })
        .sum()
}
