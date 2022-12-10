use std::{fs::{File}, io::{BufReader, BufRead}};

use scanf::sscanf;

const PART: usize = 1;

fn main() {
    let f = File::open("input/day4.txt").unwrap();
    let read = BufReader::new(f);

    let tot_overlap: usize = read.lines()
    .map(|l| l.unwrap())
    .filter(|l| {
        let mut s1: usize = 0;
        let mut s2: usize = 0;
        let mut e1: usize = 0;
        let mut e2: usize = 0;
        sscanf!(l, "{}-{},{}-{}", s1, e1, s2, e2).expect("scanf");
        match PART {
            1 => (s1 <= s2 && e2 <= e1) || (s2 <= s1 && e1 <= e2),
            2 => (s1 <= s2 && s2 <= e1) || (s2 <= s1 && s1 <= e2),
            _ => panic!()
        }
    }).count();
    println!("{}", tot_overlap);
}
