use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, Read},
    ops::{Add, AddAssign, Range},
};

const ROUNDS: usize = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    i: i32,
    j: i32,
}

impl Pos {
    fn new(i: i32, j: i32) -> Self {
        Pos { i, j }
    }
}

impl Add for Pos {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
        }
    }
}

impl Add<&Self> for Pos {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        Pos {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
        }
    }
}

impl Add<Self> for &Pos {
    type Output = Pos;
    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
        }
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.i += rhs.i;
        self.j += rhs.j;
    }
}

impl AddAssign<&Self> for Pos {
    fn add_assign(&mut self, rhs: &Self) {
        self.i += rhs.i;
        self.j += rhs.j;
    }
}

fn parse<I: Iterator<Item = u8>>(it: &mut I) -> HashSet<Pos> {
    let mut elves: HashSet<Pos> = HashSet::new();

    let mut i: i32 = 0;
    let mut j: i32 = 0;

    for c in it {
        match c {
            b'#' => {
                elves.insert(Pos::new(i, j));
                j += 1;
            }
            b'.' => j += 1,
            b'\n' => {
                i += 1;
                j = 0
            }
            _ => panic!("unexpected char"),
        }
    }
    elves
}

fn do_round(round: usize, elves: &mut HashSet<Pos>) -> bool {
    const MOVES_TO_CHECK: [[Pos; 3]; 4] = [
        // north
        [
            Pos { i: -1, j: 0 },
            Pos { i: -1, j: 1 },
            Pos { i: -1, j: -1 },
        ],
        // south
        [Pos { i: 1, j: 0 }, Pos { i: 1, j: 1 }, Pos { i: 1, j: -1 }],
        // west
        [
            Pos { i: 0, j: -1 },
            Pos { i: 1, j: -1 },
            Pos { i: -1, j: -1 },
        ],
        // east
        [Pos { i: 0, j: 1 }, Pos { i: 1, j: 1 }, Pos { i: -1, j: 1 }],
    ];
    let offset = round % MOVES_TO_CHECK.len();

    // first half: propose
    let mut proposed_dests: HashMap<Pos, Vec<Pos>> = HashMap::new(); // dest -> elves that want to go there
    for elf in elves.iter() {
        let mut has_occupied_neighbor = false;
        let mut proposed_move: Option<Pos> = None;

        for k in 0..MOVES_TO_CHECK.len() {
            let moves_dir = &MOVES_TO_CHECK[(offset + k) % MOVES_TO_CHECK.len()];
            let conflict = moves_dir.iter().any(|mov| {
                let dest = elf + mov;
                elves.contains(&dest)
            });
            if !conflict {
                proposed_move.get_or_insert(elf + &moves_dir[0]);
            } else {
                has_occupied_neighbor = true;
            }
        }
        if has_occupied_neighbor {
            // try to propose the move
            if let Some(dest) = proposed_move {
                proposed_dests.entry(dest).or_default().push(*elf);
            }
        }
    }

    // second half: move
    let mut changed = false;
    for (dest, candidates) in proposed_dests {
        if let [elf] = candidates[..] {
            // single candidate
            assert!(elves.remove(&elf), "removed nonexisting");
            assert!(elves.insert(dest), "added existing");
            changed = true;
        }
    }
    changed
}

fn calc_bounding_box(elves: &HashSet<Pos>) -> (Range<i32>, Range<i32>) {
    let mut i_min = i32::MAX;
    let mut i_max = i32::MIN;
    let mut j_min = i32::MAX;
    let mut j_max = i32::MIN;

    for &Pos { i, j } in elves {
        i_min = i_min.min(i);
        i_max = i_max.max(i);
        j_min = j_min.min(j);
        j_max = j_max.max(j);
    }
    (i_min..(i_max + 1), j_min..(j_max + 1))
}

#[allow(unused)]
fn show(elves: &HashSet<Pos>) {
    for i in -2..10 {
        for j in -3..11 {
            if elves.contains(&Pos { i, j }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }
}

fn main() {
    let f = File::open("input/day23.txt").unwrap();
    let read = BufReader::new(f);

    let mut it = read.bytes().map(|b| b.unwrap());

    // current pos
    let mut elves = parse(&mut it);
    println!("== Initial State ==");
    show(&elves);
    println!();

    for round in 0.. {
        let changed = do_round(round, &mut elves);
        if round < 5 || round == 9 {
            println!("== End of Round {} ==", round + 1);
            show(&elves);
            println!();
        }
        if round == ROUNDS - 1 {
            // part 1
            let bb = calc_bounding_box(&elves);
            let free_tiles = bb.0.len() * bb.1.len() - elves.len();

            println!("{}", free_tiles);
        }
        if !changed {
            println!("{}", round + 1);
            break;
        }
    }
}
