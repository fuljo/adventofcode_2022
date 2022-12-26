use std::{
    fmt::Display,
    fs::File,
    io::{BufReader, Read},
    ops::{Index, IndexMut, Range},
};

const PART: usize = 2;
// const CUBE_SIZE: usize = 4;
const CUBE_SIZE: usize = 50;
const WIDTH: usize = 3 * CUBE_SIZE;
const HEIGHT: usize = 4 * CUBE_SIZE;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
#[repr(usize)]
enum Direction {
    #[default]
    E = 0,
    S = 1,
    W = 2,
    N = 3,
}

impl Direction {
    fn rotate_clock(&self) -> Self {
        match self {
            Self::E => Self::S,
            Self::S => Self::W,
            Self::W => Self::N,
            Self::N => Self::E,
        }
    }

    fn rotate_anticlock(&self) -> Self {
        match self {
            Self::E => Self::N,
            Self::N => Self::W,
            Self::W => Self::S,
            Self::S => Self::E,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
enum Elem {
    #[default]
    Empty,
    Open,
    Wall,
}

#[derive(Debug)]
struct Map {
    cells: [Elem; HEIGHT * WIDTH],
    range_row: [Range<usize>; HEIGHT],
    range_col: [Range<usize>; WIDTH],
}

impl Default for Map {
    fn default() -> Self {
        Self {
            cells: [Elem::default(); HEIGHT * WIDTH],
            range_row: (0..HEIGHT)
                .map(|_| Range::default())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            range_col: (0..WIDTH)
                .map(|_| Range::default())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl Index<(usize, usize)> for Map {
    type Output = Elem;
    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        self.cells.index(WIDTH * i + j)
    }
}

impl IndexMut<(usize, usize)> for Map {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        self.cells.index_mut(i * WIDTH + j)
    }
}

impl Map {
    fn parse<I: Iterator<Item = u8>>(it: &mut I) -> Self {
        let mut map = Map::default();
        let mut i = 0;
        let mut j = 0;
        // fill grid
        loop {
            match it.next() {
                Some(b'\n') => {
                    if j == 0 {
                        break;
                    }
                    i += 1;
                    j = 0;
                }
                Some(b' ') => {
                    j += 1;
                }
                Some(b'.') => {
                    map[(i, j)] = Elem::Open;
                    j += 1;
                }
                Some(b'#') => {
                    map[(i, j)] = Elem::Wall;
                    j += 1;
                }
                Some(_) => panic!("unexpected cell"),
                None => panic!("unexpected end"),
            };
        }
        // find first and last non-empty cells for each row and column
        for i in 0..HEIGHT {
            let mut j = 0;
            // find start
            while j < WIDTH && map[(i, j)] == Elem::Empty {
                j += 1;
            }
            let start = j;
            // find end
            while j < WIDTH && map[(i, j)] != Elem::Empty {
                j += 1;
            }
            let end = j;
            map.range_row[i] = Range { start, end };
        }
        for j in 0..WIDTH {
            let mut i = 0;
            // find start
            while i < HEIGHT && map[(i, j)] == Elem::Empty {
                i += 1;
            }
            let start = i;
            // find end
            while i < HEIGHT && map[(i, j)] != Elem::Empty {
                i += 1;
            }
            let end = i;
            map.range_col[j] = Range { start, end };
        }
        map
    }

    fn walk_plane(&self, pos: (usize, usize), dir: Direction, amt: usize) -> (usize, usize) {
        let (i, j) = pos;
        match dir {
            Direction::E => {
                let rng = &self.range_row[i];
                let mut j_rel = j - rng.start;
                for _ in 0..amt {
                    let j_next =
                        rng.start + (j_rel as isize + 1).rem_euclid(rng.len() as isize) as usize;
                    if self[(i, j_next)] != Elem::Wall {
                        j_rel = j_next - rng.start;
                    } else {
                        break;
                    }
                }
                let j = rng.start + j_rel % rng.len();
                (i, j)
            }
            Direction::W => {
                let rng = &self.range_row[i];
                let mut j_rel = j - rng.start;
                for _ in 0..amt {
                    let j_next =
                        rng.start + (j_rel as isize - 1).rem_euclid(rng.len() as isize) as usize;
                    if self[(i, j_next)] != Elem::Wall {
                        j_rel = j_next - rng.start;
                    } else {
                        break;
                    }
                }
                let j = rng.start + j_rel % rng.len();
                (i, j)
            }
            Direction::S => {
                let rng = &self.range_col[j];
                let mut i_rel = i - rng.start;
                for _ in 0..amt {
                    let i_next =
                        rng.start + (i_rel as isize + 1).rem_euclid(rng.len() as isize) as usize;
                    if self[(i_next, j)] != Elem::Wall {
                        i_rel = i_next - rng.start;
                    } else {
                        break;
                    }
                }
                let i = rng.start + i_rel % rng.len();
                (i, j)
            }
            Direction::N => {
                let rng = &self.range_col[j];
                let mut i_rel = i - rng.start;
                for _ in 0..amt {
                    let i_next =
                        rng.start + (i_rel as isize - 1).rem_euclid(rng.len() as isize) as usize;
                    if self[(i_next, j)] != Elem::Wall {
                        i_rel = i_next - rng.start;
                    } else {
                        break;
                    }
                }
                let i = rng.start + i_rel % rng.len();
                (i, j)
            }
        }
    }

    fn walk_cube(
        &self,
        pos: (usize, usize),
        dir: Direction,
        amt: usize,
    ) -> ((usize, usize), Direction) {
        const SZ: isize = CUBE_SIZE as isize;
        let (i, j) = pos;
        let (mut i, mut j) = (i as isize, j as isize);
        let mut dir = dir;
        for _ in 0..amt {
            let ((i_next, j_next), dir_next) = match dir {
                Direction::E => {
                    match (i, j + 1) {
                        // q1 -> q4
                        (i, j) if (0..SZ).contains(&i) && j >= 3 * SZ => {
                            ((-(i) + 3 * SZ - 1, j - SZ - 1), Direction::W)
                        }
                        // q4 -> q1
                        (i, j) if (2 * SZ..3 * SZ).contains(&i) && j >= 2 * SZ => {
                            ((-(i - 2 * SZ) + SZ - 1, j + SZ - 1), Direction::W)
                        }
                        // q3 -> q1
                        (i, j) if (SZ..2 * SZ).contains(&i) && j >= 2 * SZ => {
                            ((j - SZ - 1, i + SZ), Direction::N)
                        }
                        // q6 -> q4
                        (i, j) if (3 * SZ..4 * SZ).contains(&i) && j >= SZ => {
                            ((j + 2 * SZ - 1, i - 2 * SZ), Direction::N)
                        }
                        // otherwise
                        (i, j) => ((i, j), dir),
                    }
                }
                Direction::W => {
                    match (i, j - 1) {
                        // q2 -> q5
                        (i, j) if (0..SZ).contains(&i) && j < SZ => {
                            ((-(i) + 3 * SZ - 1, j - SZ + 1), Direction::E)
                        }
                        // q5 -> q2
                        (i, j) if (2 * SZ..3 * SZ).contains(&i) && j < 0 => {
                            ((-(i - 2 * SZ) + SZ - 1, j + SZ + 1), Direction::E)
                        }
                        // q6 -> q2
                        (i, j) if (3 * SZ..4 * SZ).contains(&i) && j < 0 => {
                            ((j + 1, i - 2 * SZ), Direction::S)
                        }
                        // q3 -> q5
                        (i, j) if (SZ..2 * SZ).contains(&i) && j < SZ => {
                            ((j + SZ + 1, i - SZ), Direction::S)
                        }
                        // otherwise
                        (i, j) => ((i, j), dir),
                    }
                }
                Direction::S => {
                    match (i + 1, j) {
                        // q1 -> q3
                        (i, j) if (2 * SZ..3 * SZ).contains(&j) && i >= SZ => {
                            ((j - SZ, i + SZ - 1), Direction::W)
                        }
                        // q6 -> q1
                        (i, j) if (0..SZ).contains(&j) && i >= 4 * SZ => {
                            ((i - 4 * SZ, j + 2 * SZ), Direction::S)
                        }
                        // q4 -> q6
                        (i, j) if (SZ..2 * SZ).contains(&j) && i >= 3 * SZ => {
                            ((j + 2 * SZ, i - 2 * SZ - 1), Direction::W)
                        }
                        // otherwise
                        (i, j) => ((i, j), dir),
                    }
                }
                Direction::N => {
                    match (i - 1, j) {
                        // q1 -> q6
                        (i, j) if (2 * SZ..3 * SZ).contains(&j) && i < 0 => {
                            ((i + 4 * SZ, j - 2 * SZ), Direction::N)
                        }
                        // q2 -> q6
                        (i, j) if (SZ..2 * SZ).contains(&j) && i < 0 => {
                            ((j + 2 * SZ, i + 1), Direction::E)
                        }
                        // q5 -> q3
                        (i, j) if (0..SZ).contains(&j) && i < 2 * SZ => {
                            ((j + SZ, i - SZ + 1), Direction::E)
                        }
                        // otherwise
                        (i, j) => ((i, j), dir),
                    }
                }
            };
            if self[(i_next as usize, j_next as usize)] == Elem::Empty {
                println!("ERROR  {:?}", ((i_next, j_next), dir_next));
                // panic!();
            }
            if self[(i_next as usize, j_next as usize)] != Elem::Wall {
                i = i_next;
                j = j_next;
                dir = dir_next;
            } else {
                break;
            }
        }
        ((i as usize, j as usize), dir)
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let c = match self[(i, j)] {
                    Elem::Empty => ' ',
                    Elem::Open => '.',
                    Elem::Wall => '#',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let f = File::open("input/day22.txt").unwrap();
    let read = BufReader::new(f);

    let mut it = read.bytes().map(|b| b.unwrap());
    let map = Map::parse(&mut it);

    // println!("{}", map);

    let mut it = it.peekable();
    let mut pos = {
        let i = 0;
        let j = map.range_row[i].start;
        (i, j)
    };
    let mut dir = Direction::default();
    // println!("{:?}, {:?}", pos, dir);

    loop {
        // parse the number
        let mut s = String::new();
        while let Some(b) = it.next_if(|b| b.is_ascii_digit()) {
            s.push(b.into())
        }
        let amt: usize = s.parse().expect("cannot parse amount");

        // println!("walk {:?}, {:?}", amt, dir);
        if PART == 1 {
            pos = map.walk_plane(pos, dir, amt);
        } else {
            (pos, dir) = map.walk_cube(pos, dir, amt);
        }

        // parse the change of direction
        dir = match it.next() {
            Some(b'R') => dir.rotate_clock(),
            Some(b'L') => dir.rotate_anticlock(),
            _ => break,
        };
        // println!("{:?}, {:?}", pos, dir);
    }
    // println!("\n{:?}, {:?}", pos, dir);

    let final_password = 1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + dir as usize;
    println!("{}", final_password);
}
