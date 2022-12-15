use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Index, IndexMut},
};

use scanf::sscanf;

#[derive(Debug, PartialEq, Default, Clone, Copy)]
enum Cell {
    #[default]
    Air,
    Rock,
    Sand,
    Source,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Air => write!(f, "."),
            Self::Rock => write!(f, "#"),
            Self::Sand => write!(f, "o"),
            Self::Source => write!(f, "+"),
        }
    }
}

struct Grid {
    width: usize,
    height: usize,

    rock_x_min: usize,
    rock_x_max: usize,
    rock_y_min: usize,
    rock_y_max: usize,

    source: (usize, usize),

    cells: Vec<Cell>,
}

impl Grid {
    fn new((width, height): (usize, usize), source: (usize, usize)) -> Self {
        let mut grid = Self {
            width,
            height,
            cells: vec![Cell::default(); width * height],
            rock_x_min: width,
            rock_x_max: 0,
            rock_y_min: height,
            rock_y_max: 0,
            source,
        };
        grid[(source.0, source.1)] = Cell::Source;
        grid
    }

    #[allow(unused)]
    fn print_sub(&self, x_min: usize, x_max: usize, y_min: usize, y_max: usize) {
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                print!("{}", self[(x,y)])
            }
            println!()
        }
    }

    fn draw_path<'a, I: Iterator<Item = &'a (usize, usize)>>(&mut self, mut path: I) {
        let mut from = *path.next().expect("path with no first point");
        for &to in path {
            // Draw line
            let (x_0, y_0) = from;
            let (x_1, y_1) = to;
            if y_0 == y_1 {
                let y = y_0;
                // move horizontally
                let (x_0, x_1) = if x_0 < x_1 { (x_0, x_1) } else { (x_1, x_0) };
                for x in x_0..=x_1 {
                    self[(x, y)] = Cell::Rock;
                }
                self.rock_x_min = self.rock_x_min.min(x_0);
                self.rock_x_max = self.rock_x_max.max(x_1);
                self.rock_y_min = self.rock_y_min.min(y);
                self.rock_y_max = self.rock_y_max.max(y);
            } else if x_0 == x_1 {
                let x = x_0;
                let (y_0, y_1) = if y_0 < y_1 { (y_0, y_1) } else { (y_1, y_0) };
                // move horizontally
                for y in y_0..=y_1 {
                    self[(x, y)] = Cell::Rock;
                }
                self.rock_y_min = self.rock_y_min.min(y_0);
                self.rock_y_max = self.rock_y_max.max(y_1);
                self.rock_x_min = self.rock_x_min.min(x);
                self.rock_x_max = self.rock_x_max.max(x);
            }
            from = to;
        }
    }

    fn sand_fall(&mut self) -> Option<(usize, usize)> {
        if self[self.source] == Cell::Sand {
            return None; // source occluded
        }
        let (mut x, mut y) = self.source;
        let rest_pos = loop {
            let y_next = y + 1;
            if y_next > self.rock_y_max {
                break None; // fall indefinetly
            }
            match self[(x,y_next)] {
                Cell::Air => y = y_next, // move down
                Cell::Rock | Cell::Sand => {
                    let x_next = x - 1;
                    if x_next < self.rock_x_min {
                        break None; // fall indefinetly
                    }
                    match self[(x_next, y_next)] {
                        Cell::Air => (x, y) = (x_next, y_next), // move down-left
                        Cell::Rock | Cell::Sand => {
                            let x_next = x + 1;
                            if x_next > self.rock_x_max {
                                break None; // fall indefinetly
                            }
                            match self[(x_next, y_next)] {
                                Cell::Air => (x, y) = (x_next, y_next), // move down-right
                                Cell::Rock | Cell::Sand => break Some((x, y)), // come to a rest
                                Cell::Source => panic!("source"), // block source
                            }
                        }
                        Cell::Source => panic!("source")
                    }
                }
                Cell::Source => panic!("source"),
            }
        };
        if let Some(pos) = rest_pos {
            self[pos] = Cell::Sand;
        }
        rest_pos
    }

    fn clear_sand(&mut self) {
        for cell in self.cells.iter_mut() {
            if Cell::Sand == *cell {
                *cell = Cell::Air;
            }
        }
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Cell;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.cells[y * self.width + x]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.cells[y * self.width + x]
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self[(x,y)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let f = File::open("input/day14.txt").unwrap();
    let read = BufReader::new(f);
    let mut lines = read.lines();

    let mut paths: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;

    // parse
    while let Some(Ok(line)) = lines.next() {
        let path: Vec<(usize, usize)> = line
            .split(" -> ")
            .map(|s| {
                let mut x: usize = 0;
                let mut y: usize = 1;
                sscanf!(s, "{},{}", x, y).expect("cannot parse pair");
                max_x = max_x.max(x);
                max_y = max_y.max(y);
                (x, y)
            })
            .collect();
        paths.push(path);
    }

    let mut grid = Grid::new((max_x.max(500 + max_y + 3) , max_y + 3), (500, 0));

    for path in paths {
        grid.draw_path(path.iter());
    }

    // part 1
    let mut sand_count: usize = 0;
    while grid.sand_fall().is_some() {
        sand_count += 1;
    }
    println!("{}", sand_count);

    // part 2
    grid.clear_sand();
    grid.draw_path([(0, grid.height - 1), (grid.width - 1, grid.height - 1)].iter());

    let mut sand_count: usize = 0;
    while grid.sand_fall().is_some() {
        sand_count += 1;
    }
    println!("{}", sand_count);
    
    // grid.print_sub(470, 530, 0, 11);
}
