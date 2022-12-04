use std::{fs::{File}, io::{BufReader, BufRead}};

fn main() {
    let f = File::open("input/day3.txt").unwrap();
    let read = BufReader::new(f);

    let tot_prio: usize = read.lines()
    .map(|l| l.unwrap())
    .map(|l| {
        assert_eq!(l.len() % 2, 0);
        l[..l.len() / 2].chars().find(|&c| {
            l[l.len() / 2..].chars().any(|other| c == other)
        }).unwrap()
    })
    .map(|c| {
        match c {
            'a'..='z' => c as usize - 'a' as usize + 1,
            'A'..='Z' => c as usize - 'A' as usize + 27,
            _ => panic!()
        }
    }).sum();
    println!("{}", tot_prio);
}
