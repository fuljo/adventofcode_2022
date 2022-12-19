use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    hash::{Hash, Hasher},
    io::{BufRead, BufReader},
    ops::{Index, IndexMut},
};

const WIDTH: usize = 7;
const SPAWN_X: usize = 2;
const SPAWN_Y: usize = 3;
const TARGET_PART_1: usize = 2022;
const TARGET_PART_2: usize = 1000000000000;

#[derive(Debug, Clone)]
struct Chamber {
    cells: [Vec<bool>; WIDTH],
    height: usize, // relative
}

impl Chamber {
    fn new() -> Self {
        Self {
            cells: Default::default(),
            height: 0,
        }
    }

    fn fit(&mut self, target: usize) {
        if target < self.height {
            return;
        }

        for col in self.cells.iter_mut() {
            col.resize(target + 1, false);
        }
        self.height = target + 1;
    }

    fn trim_bottom(&mut self) -> usize {
        let y = (0..self.height).rev().find(|&y| {
            self.cells.iter().all(|col| col[y])
        });

        if let Some(y) = y {
            self.height -= y + 1;
            for col in self.cells.iter_mut() {
                col.drain(0..=y);
            }
            y + 1
        } else {
            0
        }
    }

    fn collides(&self, x: isize, y: isize) -> bool {
        if x < 0 || x >= WIDTH as isize || y < 0 {
            return true;
        }
        let x = x as usize;
        let y = y as usize;

        if y >= self.height {
            false
        } else {
            self.cells[x][y]
        }
    }

    fn piece_collides(&self, piece: &[(usize, usize)], dx: isize, dy: isize) -> bool {
        piece
            .iter()
            .any(|(x, y)| self.collides(*x as isize + dx, *y as isize + dy))
    }

    fn place(&mut self, piece: &[(usize, usize)]) {
        for (x, y) in piece.iter() {
            let x = *x;
            let y = *y;
            self.fit(y);
            self.cells[x][y] = true;
        }
    }
}

impl Index<(usize, usize)> for Chamber {
    type Output = bool;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        self.cells.index(x).index(y)
    }
}

impl IndexMut<(usize, usize)> for Chamber {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        self.cells.index_mut(x).index_mut(y)
    }
}

impl PartialEq for Chamber {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height && self.cells == other.cells
    }
}

impl Eq for Chamber {}

impl Hash for Chamber {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.height.hash(state);
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..self.height).rev() {
            write!(f, "|")?;
            for x in 0..WIDTH {
                if self.cells[x][y] {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "+-------+")?;
        Ok(())
    }
}

// the anchor is the bottom-left corner
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Piece {
    Horizontal,
    Cross,
    Angle,
    Vertical,
    Square,
}

impl Piece {
    fn cells(&self) -> &'static [(usize, usize)] {
        match self {
            Piece::Horizontal => &[(0, 0), (1, 0), (2, 0), (3, 0)],
            Piece::Vertical => &[(0, 0), (0, 1), (0, 2), (0, 3)],
            Piece::Cross => &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            Piece::Angle => &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Piece::Square => &[(0, 0), (1, 0), (0, 1), (1, 1)],
        }
    }

    fn for_id(id: usize) -> Self {
        match id % 5 {
            0 => Self::Horizontal,
            1 => Self::Cross,
            2 => Self::Angle,
            3 => Self::Vertical,
            4 => Self::Square,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct State {
    piece: Piece,
    seq_offset: usize,
    chamber: Chamber,
}

impl State {
    fn new() -> Self {
        State {
            piece: Piece::for_id(0),
            seq_offset: 0,
            chamber: Chamber::new(),
        }
    }

    fn spawn(&self) -> Vec<(usize, usize)> {
        let mut cells = self.piece.cells().to_vec();
        cells.iter_mut().for_each(|(x, y)| {
            *x += SPAWN_X;
            *y += self.chamber.height + SPAWN_Y;
        });
        cells
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "State(piece: {}, seq: {})",
            self.piece as usize, self.seq_offset
        )
    }
}

struct CacheValue {
    piece_count: usize,
    floor_offset: usize,
}

impl CacheValue {
    fn new(piece_count: usize, floor_offset: usize) -> Self {
        Self {
            piece_count,
            floor_offset,
        }
    }
}

#[allow(unused)]
fn show(chamber: &Chamber, piece: &[(usize, usize)]) {
    let mut chamber = chamber.clone();
    chamber.place(piece);
    println!("{}", chamber);
}

fn main() {
    let f = File::open("input/day17.txt").unwrap();
    let read = BufReader::new(f);
    let mut lines = read.lines();

    let Some(Ok(line)) = lines.next() else {
        panic!("cannot read line");
    };

    // initial state
    let mut state = State::new();
    let mut piece_count: usize = 0;
    let mut floor_offset: usize = 0;

    let mut cache = HashMap::<State, CacheValue>::new(); // stores the previous ground offset
    let seq_len = line.len();
    cache.insert(state.clone(), CacheValue::new(piece_count, floor_offset));

    let mut piece = state.spawn();

    for mov in line.chars().cycle() {
        // update state
        state.seq_offset = (state.seq_offset + 1) % seq_len;

        // push left/right
        let dx: isize = match mov {
            '<' => -1,
            '>' => 1,
            _ => panic!("invalid move"),
        };
        if !state.chamber.piece_collides(&piece, dx, 0) {
            piece.iter_mut().for_each(|(x, _)| {
                *x = (*x as isize + dx) as usize;
            });
        }

        // push down
        if !state.chamber.piece_collides(&piece, 0, -1) {
            piece.iter_mut().for_each(|(_, y)| {
                *y -= 1;
            });
            continue;
        }

        // rest
        state.chamber.place(&piece);
        let trimmed = state.chamber.trim_bottom();
        floor_offset += trimmed;

        // update state
        piece_count += 1;
        state.piece = Piece::for_id(piece_count);
        piece = state.spawn();

        // cache
        if let Some(cached) = cache.get(&state) {
            // determine the period
            let piece_incr = piece_count - cached.piece_count;
            let floor_incr = floor_offset - cached.floor_offset;
            // take the shortcut
            let periods = (TARGET_PART_2 - piece_count) / piece_incr;
            piece_count += periods * piece_incr;
            floor_offset += periods * floor_incr;
            // further states should not do the same
            cache.clear();
        } else {
            cache.insert(state.clone(), CacheValue::new(piece_count, floor_offset));
        }

        // termination
        if piece_count < TARGET_PART_2 {
            if piece_count == TARGET_PART_1 {
                println!("{}", state.chamber.height + floor_offset);
            }
        } else {
            println!("{}", state.chamber.height + floor_offset);
            break;
        }
    }
}
