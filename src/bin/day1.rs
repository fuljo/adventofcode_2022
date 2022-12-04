use std::{fs::{File}, io::{BufReader, BufRead}};

fn main() {
    let f = File::open("input/day1.txt").unwrap();
    let read = BufReader::new(f);

    let mut max: usize = 0;
    let mut sum: usize = 0;

    for l in read.lines() {
        let l = l.unwrap();
        if l.is_empty() {
            max = if sum > max {sum} else {max};
            sum = 0;
        } else {
            let calories: usize = l.parse().unwrap();
            sum += calories;
        }
    }

    println!("{}", max);
}
