use std::{
    collections::{BinaryHeap, HashSet},
    fmt::Display,
    fs::File,
    hash::{Hash, Hasher},
    io::{BufReader, Read},
    ops::{Add, AddAssign, Index, IndexMut}, mem,
};

// const WIDTH: usize = 8;
// const HEIGHT: usize = 6;
const WIDTH: usize = 122;
const HEIGHT: usize = 27;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    N,
    S,
    W,
    E,
}

impl Direction {
    fn step(&self, pos: &Pos) -> Pos {
        match self {
            Self::N => {
                let mut res = Pos {
                    i: pos.i - 1,
                    j: pos.j,
                };
                if res.i == 0 {
                    // wall
                    res.i = HEIGHT - 2;
                }
                res
            }
            Self::S => {
                let mut res = Pos {
                    i: pos.i + 1,
                    j: pos.j,
                };
                if res.i == HEIGHT - 1 {
                    // wall
                    res.i = 1;
                }
                res
            }
            Self::W => {
                let mut res = Pos {
                    i: pos.i,
                    j: pos.j - 1,
                };
                if res.j == 0 {
                    // wall
                    res.j = WIDTH - 2;
                }
                res
            }
            Self::E => {
                let mut res = Pos {
                    i: pos.i,
                    j: pos.j + 1,
                };
                if res.j == WIDTH - 1 {
                    // wall
                    res.j = 1;
                }
                res
            }
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::N => '^',
                Self::S => 'v',
                Self::W => '<',
                Self::E => '>',
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    i: usize,
    j: usize,
}

impl Pos {
    fn new(i: usize, j: usize) -> Self {
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

#[derive(Debug, Clone)]
struct Map {
    start: Pos,
    end: Pos,
    cells: [Vec<Direction>; WIDTH * HEIGHT],
}

impl Index<&Pos> for Map {
    type Output = Vec<Direction>;
    fn index(&self, index: &Pos) -> &Self::Output {
        self.cells.index(index.i * WIDTH + index.j)
    }
}

impl IndexMut<&Pos> for Map {
    fn index_mut(&mut self, index: &Pos) -> &mut Self::Output {
        self.cells.index_mut(index.i * WIDTH + index.j)
    }
}

impl FromIterator<u8> for Map {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        let mut it = iter.into_iter();
        let mut start = Pos::new(0, 0);
        let mut end = Pos::new(0, 0);
        let cells: [Vec<Direction>; WIDTH * HEIGHT] = (0..WIDTH * HEIGHT)
            .map(|k| {
                let mut c = it.next().expect("end of input");
                if c == b'\n' {
                    c = it.next().expect("end of input");
                }
                let i = k / WIDTH;
                let j = k % WIDTH;
                match c {
                    b'.' if i == 0 => {
                        start = Pos::new(i, j);
                        vec![]
                    }
                    b'.' if i == HEIGHT - 1 => {
                        end = Pos::new(i, j);
                        vec![]
                    }
                    b'#' | b'.' => {
                        vec![]
                    }
                    b'^' => vec![Direction::N],
                    b'v' => vec![Direction::S],
                    b'<' => vec![Direction::W],
                    b'>' => vec![Direction::E],
                    _ => panic!("unexpected char"),
                }
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Map { start, end, cells }
    }
}

impl Map {
    fn step(&self) -> Self {
        let mut cells_next: Vec<Vec<Direction>> = Vec::new();
        cells_next.resize(WIDTH * HEIGHT, Vec::new());

        let mut map_next = Map {
            start: self.start,
            end: self.end,
            cells: cells_next.try_into().unwrap(),
        };

        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let pos = Pos { i, j };
                for bliz in self[&pos].iter() {
                    let pos_next = bliz.step(&pos);
                    map_next[&pos_next].push(*bliz);
                }
            }
        }
        map_next
    }

    fn distance_to_end(&self, pos: &Pos) -> usize {
        pos.i.abs_diff(self.end.i) + pos.j.abs_diff(self.end.j)
    }

    fn is_free(&self, pos: &Pos) -> bool {
        if pos != &self.start
            && pos != &self.end
            && (pos.i == 0 || pos.i == HEIGHT - 1 || pos.j == 0 || pos.j == WIDTH - 1)
        {
            false
        } else {
            self[pos].is_empty()
        }
    }

    fn swap_start_end(&mut self) {
        mem::swap(&mut self.start, &mut self.end);
    }
}

#[allow(unused)]
fn show(map: &Map, pos: &Pos) {
    for (k, cell) in map.cells.iter().enumerate() {
        let i = k / WIDTH;
        let j = k % WIDTH;

        if (i == 0 && j != map.start.j || i == HEIGHT - 1 && j != map.end.j)
            || j == 0
            || j == WIDTH - 1
        {
            print!("#")
        } else {
            match cell[..] {
                [] => print!("."),
                [bliz] => print!("{}", bliz),
                [_, ..] => print!("{}", cell.len()),
            }
        }
        if j == WIDTH - 1 {
            println!();
        }
    }
}

#[derive(Debug, Clone)]
struct SearchNode {
    t: usize,
    pos: Pos,
    min_cost_to_goal: usize,
}

impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.pos == other.pos
    }
}

impl Eq for SearchNode {}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.evaluate().cmp(&other.evaluate()).reverse()
    }
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for SearchNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.t.hash(state);
        self.pos.hash(state);
        self.min_cost_to_goal.hash(state);
    }
}

impl SearchNode {
    fn new(map: &Map) -> Self {
        let min_cost_to_goal = map.distance_to_end(&map.start);
        Self {
            t: 0,
            pos: map.start,
            min_cost_to_goal,
        }
    }

    fn evaluate(&self) -> usize {
        self.t + self.min_cost_to_goal
    }

    fn expand(&self, map_t: &mut Vec<Map>) -> Vec<SearchNode> {
        let map_cur = &map_t[self.t];
        let t_next = self.t + 1;
        let mut children = Vec::new();

        // goal
        if self.pos == map_cur.end {
            return children;
        }

        // generate next timestep
        if map_t.len() < t_next + 1 {
            map_t.push(map_cur.step());
        }
        let map_next = &mut map_t[t_next];

        // actions
        let next_steps = [(-1, 0), (1, 0), (0, -1), (0, 1), (1, 0), (0, 0)]
            .iter()
            .filter_map(|(di, dj)| {
                let i = self.pos.i.checked_add_signed(*di)?;
                let j = self.pos.j.checked_add_signed(*dj)?;
                let pos_next = Pos { i, j };
                if i < HEIGHT && j < WIDTH && map_next.is_free(&pos_next) {
                    Some(pos_next)
                } else {
                    None
                }
            });
        for dest in next_steps {
            let min_cost_to_goal = map_next.distance_to_end(&dest);
            let node_next = SearchNode {
                t: t_next,
                pos: dest,
                min_cost_to_goal,
            };
            children.push(node_next);
        }

        children
    }
}

impl Display for SearchNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SearchNode(t: {}, pos: {:?}, he: {}, eval: {})",
            self.t,
            self.pos,
            self.min_cost_to_goal,
            self.evaluate()
        )
    }
}

fn search_sol(map: Map) -> (Map, usize) {
    let mut map_t = vec![map];

    let mut frontier: BinaryHeap<SearchNode> = BinaryHeap::new();
    let mut visited: HashSet<SearchNode> = HashSet::new();

    // Initial node
    frontier.push(SearchNode::new(&map_t[0]));

    // Search
    let mut cost: usize = usize::MAX;
    while let Some(node) = frontier.pop() {
        if visited.contains(&node) {
            continue;
        }
        // println!("{}", node);

        // goal
        if node.pos == map_t[0].end {
            println!("Solution: {}", node);
            cost = node.t;
            break;
        }

        // expand
        for child in node.expand(&mut map_t) {
            if !visited.contains(&child) {
                frontier.push(child);
            }
        }
        visited.insert(node);
    }
    (map_t.remove(cost), cost)
}

fn main() {
    let f = File::open("input/day24.txt").unwrap();
    let read = BufReader::new(f);

    let it = read.bytes().map(|b| b.unwrap());

    let map: Map = it.collect();

    let (mut map, trip_1) = search_sol(map);
    map.swap_start_end();
    let (mut map, trip_2) = search_sol(map);
    map.swap_start_end();
    let (_, trip_3) = search_sol(map);
    
    println!("{}", trip_1 + trip_2 + trip_3);
}
