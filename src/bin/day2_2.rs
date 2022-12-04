use std::{fs::{File}, io::{BufReader, BufRead}, cmp::Ordering::{Greater, Less, Equal}};

#[derive(Debug, PartialEq, Eq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Ord for Choice {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Self::Rock => {
                match other {
                    Self::Rock => Equal,
                    Self::Paper => Less,
                    Self::Scissors => Greater,
                }
            }
            Self::Paper => {
                match other {
                    Self::Rock => Greater,
                    Self::Paper => Equal,
                    Self::Scissors => Less,
                }
            }
            Self::Scissors => {
                match other {
                    Self::Rock => Less,
                    Self::Paper => Greater,
                    Self::Scissors => Equal,
                }
            }
        }
    }
}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Lose,
    Tie,
    Win,
}

fn decide(opp: &Choice, outcome: &Outcome) -> Choice {
    match opp {
        Choice::Rock => match outcome {
            Outcome::Lose => Choice::Scissors,
            Outcome::Tie => Choice::Rock,
            Outcome::Win => Choice::Paper,
        }
        Choice::Paper => match outcome {
            Outcome::Lose => Choice::Rock,
            Outcome::Tie => Choice::Paper,
            Outcome::Win => Choice::Scissors,
        }
        Choice::Scissors => match outcome {
            Outcome::Lose => Choice::Paper,
            Outcome::Tie => Choice::Scissors,
            Outcome::Win => Choice::Rock,
        }
    }
}
fn main() {
    let f = File::open("input/day2.txt").unwrap();
    let read = BufReader::new(f);

    let total_score: usize = read.lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let mut it = l.chars();
            let opp = match it.next().unwrap() {
                'A' => Choice::Rock,
                'B' => Choice::Paper,
                'C' => Choice::Scissors,
                x => panic!("Invalid move {}", x),
            };
            assert_eq!(it.next(), Some(' '));
            let outcome = match it.next().unwrap() {
                'X' => Outcome::Lose,
                'Y' => Outcome::Tie,
                'Z' => Outcome::Win,
                x => panic!("Invalid move {}", x),
            };
            let mut score: usize = match decide(&opp, &outcome) {
                Choice::Rock => 1,
                Choice::Paper => 2,
                Choice::Scissors => 3,
            };
            score += match outcome {
                Outcome::Lose => 0,
                Outcome::Tie => 3,
                Outcome::Win => 6,
            };
            score
        }).sum();
        println!("{}", total_score);
}
