use std::{
    cmp::Ordering,
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Element {
    List(Vec<Element>),
    Integer(usize),
}

use Element::{Integer, List};

impl FromStr for Element {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack: VecDeque<Vec<Element>> = VecDeque::new();
        let mut chars = s.chars().peekable();
        let elem = loop {
            match chars.peek() {
                Some('[') => {
                    chars.next(); // consume
                                  // start a new list
                    stack.push_front(vec![]);
                }
                Some(']') => {
                    chars.next(); // consume
                                  // close current list
                    let list = stack.pop_front().ok_or("closing non-existent list")?;
                    match stack.front_mut() {
                        Some(parent) => parent.push(List(list)),
                        None => match chars.peek() {
                            None => break List(list),
                            Some(_) => return Err("found trailing chars after top-level list"),
                        },
                    }
                }
                Some(n) if n.is_ascii_digit() => {
                    // read the full number
                    let mut s = String::new();
                    while let Some(ch) = chars.next_if(|c| c.is_ascii_digit()) {
                        s.push(ch)
                    }
                    // parse it
                    let n: usize = s.parse().map_err(|_| "cannt parse integer")?;
                    // append to current list
                    match stack.front_mut() {
                        Some(list) => list.push(Integer(n)),
                        None => return Err("the outermost element should be a list"),
                    };
                }
                Some(',') | Some(' ') => {
                    chars.next(); // consume
                }
                Some(_) => return Err("unexpected char"),
                None => return Err("input terminated unexpectedly"),
            }
        };
        Ok(elem)
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Integer(l), Integer(r)) => l.cmp(r),
            (List(l), List(r)) => {
                let mut l = l.iter();
                let mut r = r.iter();
                loop {
                    let l = l.next();
                    let r = r.next();
                    match (l, r) {
                        (None, Some(_)) => break Ordering::Less,
                        (Some(_), None) => break Ordering::Greater,
                        (Some(l), Some(r)) => match l.cmp(r) {
                            Ordering::Less => break Ordering::Less,
                            Ordering::Equal => continue,
                            Ordering::Greater => break Ordering::Greater,
                        },
                        (None, None) => break Ordering::Equal,
                    }
                }
            }
            (Integer(l), r) => List(vec![Integer(*l)]).cmp(r),
            (l, Integer(r)) => l.cmp(&List(vec![Integer(*r)])),
        }
    }
}

fn main() {
    let f = File::open("input/day13.txt").unwrap();
    let read = BufReader::new(f);
    let mut lines = read.lines();

    let mut in_order: usize = 0;
    let mut i: usize = 0;

    let mut packets: Vec<Element> = Vec::new();

    loop {
        i += 1;
        let l1 = lines.next().expect("cannot get line 1").unwrap();
        let l2 = lines.next().expect("cannot get line 2").unwrap();

        let pack1: Element = l1.parse().expect("cannot parse line 1");
        let pack2: Element = l2.parse().expect("cannot parse line 2");

        if pack1 < pack2 {
            in_order += i;
        }

        packets.push(pack1);
        packets.push(pack2);

        if lines.next().is_none() {
            break;
        }
    }

    let decode_key = {
        let div_2 = List(vec![List(vec![Integer(2)])]);
        let div_6 = List(vec![List(vec![Integer(6)])]);

        packets.push(div_2.clone());
        packets.push(div_6.clone());

        packets.sort();

        let pos_2 = packets.iter().position(|p| *p == div_2).expect("cannot find marker 2") + 1;
        let pos_6 = packets.iter().position(|p| *p == div_6).expect("cannot find marker 6") + 1;

        pos_2 * pos_6
    };

    println!("{}", in_order);
    println!("{}", decode_key);
}
