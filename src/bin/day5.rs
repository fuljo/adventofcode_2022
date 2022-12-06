use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

use scanf::sscanf;

fn main() {
    let f = File::open("input/day5.txt").unwrap();
    let read = BufReader::new(f);
    let mut lines = read.lines();

    const N: usize = 9;
    const PART: usize = 2;

    // back is top, front is bottom
    let mut stacks: [VecDeque<char>; N] = Default::default();

    // Parse input
    loop {
        let l = lines.next();
        if let Some(Ok(l)) = l {
            if !l.contains('[') {
                break;
            }
            assert_eq!(l.len(), 4 * N - 1);
            for (i, stack) in stacks.iter_mut().enumerate() {
                let cr = l.chars().nth(4 * i + 1).unwrap();
                match cr {
                    ' ' => (),
                    cr => {
                        stack.push_front(cr);
                    }
                }
            }
        } else {
            panic!()
        }
    }
    assert!(lines.next().unwrap().unwrap().is_empty());
    for (i, stack) in stacks.iter().enumerate() {
        println!("{}: {:?}", i, stack);
    }
    println!();

    // Moves
    for l in lines {
        let l = l.unwrap();
        let mut num: usize = 0;
        let mut src: usize = 0;
        let mut dst: usize = 0;
        sscanf!(&l, "move {} from {} to {}", num, src, dst).expect("scanf");
        src -= 1;
        dst -= 1;

        match PART {
            1 => { // one crate at a time
                for _ in 0..num {
                    let el = stacks[src].pop_back().unwrap();
                    stacks[dst].push_back(el);
                }
            }
            2 => { // multiple crates at a time
                let off = stacks[src].len() - num;
                let els = stacks[src].split_off(off);
                stacks[dst].extend(els);
            },
            _ => panic!()
        }
    }
    for (i, stack) in stacks.iter().enumerate() {
        println!("{}: {:?}", i, stack);
    }
    println!();

    let s: String = stacks.iter().map(|s| s.back().unwrap_or(&' ')).collect();
    println!("{}", s);
}
