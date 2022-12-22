use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    str::{self, FromStr},
};

const PART: usize = 2;

enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl FromStr for Op {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Sub),
            "*" => Ok(Self::Mul),
            "/" => Ok(Self::Div),
            _ => Err(()),
        }
    }
}

struct Monkey {
    x: u32,
    y: u32,
    op: Op,
}

impl Monkey {
    fn try_compute(&self, solved: &HashMap<u32, isize>) -> Option<isize> {
        let x = solved.get(&self.x)?;
        let y = solved.get(&self.y)?;
        match self.op {
            Op::Add => Some(x + y),
            Op::Sub => Some(x - y),
            Op::Div => Some(x / y),
            Op::Mul => Some(x * y),
        }
    }

    fn solve_unknown(&self, res: isize, solved: &HashMap<u32, isize>) -> Option<isize> {
        if let Some(x) = solved.get(&self.x) {
            match self.op {
                Op::Add => Some(res - x),
                Op::Sub => Some(x - res),
                Op::Div => Some(x / res),
                Op::Mul => Some(res / x),
            }
        } else if let Some(y) = solved.get(&self.y) {
            match self.op {
                Op::Add => Some(res - y),
                Op::Sub => Some(res + y),
                Op::Div => Some(res * y),
                Op::Mul => Some(res / y),
            }
        } else {
            None
        }
    }

    fn get_known(&self, solved: &HashMap<u32, isize>) -> Option<isize> {
        if let Some(x) = solved.get(&self.x) {
            Some(*x)
        } else { solved.get(&self.y).copied() }
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self.op {
            Op::Add => '+',
            Op::Sub => '-',
            Op::Mul => '*',
            Op::Div => '/',
        };
        write!(f, "{} {} {}", decode(&self.x), op, decode(&self.y))
    }
}

fn encode(s: &str) -> u32 {
    let mut res: u32 = 0;
    for (i, b) in s.bytes().rev().enumerate() {
        res += (b as u32) << (8 * i);
    }
    res
}

#[allow(unused)]
fn decode(n: &u32) -> String {
    let mut s: [u8; 4] = [0; 4];
    s[0] = ((n >> 24) & 0xff) as u8;
    s[1] = ((n >> 16) & 0xff) as u8;
    s[2] = ((n >> 8) & 0xff) as u8;
    s[3] = (n & 0xff) as u8;
    let s = str::from_utf8(&s).unwrap();
    s.to_string()
}

fn solve_cascade(
    start: u32,
    solved: &mut HashMap<u32, isize>,
    unsolved: &mut HashMap<u32, Monkey>,
    waiting_on: &mut HashMap<u32, Vec<u32>>,
) {
    let mut queue: VecDeque<u32> = VecDeque::new();

    if let Some(ks) = waiting_on.remove(&start) {
        for k in ks {
            queue.push_back(k);
        }
    }

    while let Some(k) = queue.pop_front() {
        // try to solve this
        let m = unsolved.get(&k).expect("expected unsolved");
        if let Some(res) = m.try_compute(solved) {
            // solved
            unsolved.remove(&k);
            solved.insert(k, res);
            // enqueue its dependants
            if let Some(ks) = waiting_on.remove(&k) {
                for k in ks {
                    queue.push_back(k);
                }
            }
        }
    }
}

fn solve_root_eq(
    solved: &mut HashMap<u32, isize>,
    unsolved: &mut HashMap<u32, Monkey>,
    waiting_on: &mut HashMap<u32, Vec<u32>>,
) -> isize {
    let mut path_from_root: VecDeque<u32> = VecDeque::new();

    let mut cur = encode("humn");
    while let Some(ks) = waiting_on.get(&cur) {
        match ks[..] {
            [k] => {
                path_from_root.push_front(k);
                cur = k;
            }
            _ => panic!()
        }
    }

    // find the unknown value in the operation
    let root = path_from_root.pop_front().unwrap();
    let root = unsolved.get(&root).unwrap();
    let mut res = root.get_known(solved).unwrap();
    while let Some(k) = path_from_root.pop_front() {
        let m = unsolved.get(&k).unwrap();
        res = m.solve_unknown(res, solved).unwrap();
    }
    res
}

fn main() {
    // part 1
    let f = File::open("input/day21.txt").unwrap();
    let read = BufReader::new(f);
    let lines = read.lines();

    // known/computed numbers
    let mut solved = HashMap::<u32, isize>::new();

    let mut unsolved = HashMap::<u32, Monkey>::new();

    // all monkeys waiting on a given monkey
    let mut waiting_on = HashMap::<u32, Vec<u32>>::new();

    for line in lines {
        let s = line.unwrap();

        // parse
        let (k, s) = s.split_at(4);
        if PART == 2 && k == "humn" {
            continue;
        }
        let k = encode(k);

        let s = s.trim_start_matches(": ");

        let toks: Vec<&str> = s.split(' ').collect();

        match toks[..] {
            [v] => {
                let v = v.parse().unwrap();
                solved.insert(k, v);
                solve_cascade(k, &mut solved, &mut unsolved, &mut waiting_on);
            }
            [x, op, y] => {
                let x: u32 = encode(x);
                let y: u32 = encode(y);
                let op: Op = op.parse().unwrap();

                let m = Monkey { x, y, op };

                if let Some(v) = m.try_compute(&solved) {
                    solved.insert(k, v);
                    solve_cascade(k, &mut solved, &mut unsolved, &mut waiting_on);
                } else {
                    unsolved.insert(k, m);
                    if !solved.contains_key(&x) {
                        waiting_on.entry(x).or_default().push(k);
                    }
                    if !solved.contains_key(&y) {
                        waiting_on.entry(y).or_default().push(k);
                    }
                }
            }
            _ => panic!(),
        }
    }

    if PART == 1 {
        let root = solved.get(&encode("root")).expect("root not solved");
        println!("{}", root);
    } else {
        let humn_value = solve_root_eq(&mut solved, &mut unsolved, &mut waiting_on);
        println!("{}", humn_value);
    }
}
