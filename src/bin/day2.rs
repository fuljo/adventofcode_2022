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
            let mov = match it.next().unwrap() {
                'X' => Choice::Rock,
                'Y' => Choice::Paper,
                'Z' => Choice::Scissors,
                x => panic!("Invalid move {}", x),
            };
            let mut score: usize = match mov {
                Choice::Rock => 1,
                Choice::Paper => 2,
                Choice::Scissors => 3,
            };
            score += match mov.cmp(&opp) {
                Less => 0,
                Equal => 3,
                Greater => 6,
            };
            score
        }).sum();
        println!("{}", total_score);
}
