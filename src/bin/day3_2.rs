use std::{fs::{File}, io::{BufReader, BufRead}};

fn main() {
    let f = File::open("input/day3.txt").unwrap();
    let read = BufReader::new(f);
    let mut lines = read.lines();
    let mut tot_prio: usize = 0;

    loop {
        let l = lines.next();
        if l.is_none() {
            break;
        }
        let l1 = l.unwrap().unwrap();
        let l2 = lines.next().unwrap().unwrap();
        let l3 = lines.next().unwrap().unwrap();

        let c = l1.chars().find(|&c1| {
            l2.chars().any(|c2| c1 == c2) && l3.chars().any(|c3| c1 == c3)
        }).unwrap();

        tot_prio += match c {
            'a'..='z' => c as usize - 'a' as usize + 1,
            'A'..='Z' => c as usize - 'A' as usize + 27,
            _ => panic!()
        }
    }
    println!("{}", tot_prio);
}
