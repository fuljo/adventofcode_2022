use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(i32, i32);

impl Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, rhs: Pos) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Pos> for Pos {
    type Output = Pos;
    fn sub(self, rhs: Pos) -> Self::Output {
        Pos(
            (self.0 - rhs.0).clamp(-1, 1),
            (self.1 - rhs.1).clamp(-1, 1),
        )
    }
}

fn main() {
    let f = File::open("input/day9.txt").unwrap();
    let read = BufReader::new(f);
    let lines = read.lines();

    let mut visited_1 = HashSet::<Pos>::new();
    let mut visited_9 = HashSet::<Pos>::new();

    let mut knots = [Pos(0,0); 10];

    for line in lines {
        let line = line.expect("unknown line");
        let mut toks = line.split(' ');
        let dir = toks.next().expect("direction");
        let cnt: usize = toks.next().expect("count").parse().expect("integer");

        for _ in 0..cnt {
            // move head
            match dir {
                "U" => knots[0].0 += 1,
                "D" => knots[0].0 -= 1,
                "L" => knots[0].1 -= 1,
                "R" => knots[0].1 += 1,
                _ => panic!("unknown dir"),
            }
            // follow with the other knots
            for k in 1..10 {
                let knot = knots[k];
                let prev = knots[k-1];
                let tgt = knot + (prev - knot);
                if tgt != prev {
                    knots[k] = tgt;
                }
                match k {
                    1 => visited_1.insert(knots[k]),
                    9 => visited_9.insert(knots[k]),
                    _ => false,
                };
            }
        }
    }

    println!("{}", visited_1.len());
    println!("{}", visited_9.len());
}
