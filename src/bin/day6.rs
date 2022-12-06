use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("input/day6.txt").unwrap();
    let read = BufReader::new(f);

    const PART: usize = 2;
    const MARK_LEN: usize = if PART == 1 { 4 } else { 14 };

    let mut off: usize = 0;
    let mut seq: VecDeque<char> = Default::default();

    for (i, c) in read.lines().next().unwrap().unwrap().chars().enumerate() {
        seq.push_back(c);
        if seq.len() > MARK_LEN {
            seq.pop_front();
            let mut unique = true;
            for &c in seq.iter() {
                unique &= seq.iter().filter(|&&d| d == c).count() == 1;
            }
            if unique {
                off = i;
                break;
            }
        }
    }
    println!("{}", off + 1);
}
