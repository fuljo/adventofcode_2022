use std::{fs::{File}, io::{BufReader, BufRead}};

fn main() {
    let f = File::open("input/day1.txt").unwrap();
    let read = BufReader::new(f);

    let mut top: [usize; 3] = [0,0,0];
    let mut sum: usize = 0;

    let mut lines = read.lines();
    
    loop {
        let l = lines.next();
        let l = l.map(|x| x.unwrap());
        if l.is_none() || l.as_ref().unwrap().is_empty() {
            for i in 0..3 {
                if sum >= top[i] {
                    // push back
                    for j in (i+1..3).rev() {
                        top[j] = top[j-1];
                    }
                    top[i] = sum;
                    break;
                }
            }
            sum = 0;
        } else {
            let calories: usize = l.as_ref().unwrap().parse().unwrap();
            sum += calories;
        }
        if l.is_none() {break};
    }
    let sum_top: usize = top.iter().sum();

    println!("{}", top[0]);
    println!("{}", sum_top);
}
