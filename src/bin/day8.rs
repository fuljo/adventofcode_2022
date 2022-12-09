use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, Copy, Default)]
struct Tree {
    height: usize,
    visible: bool,
}

const W: usize = 99;
const H: usize = 99;

struct Grid<T> {
    cells: [T; W * H],
}

impl<T> Index<usize> for Grid<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cells[index]
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (i, j) = index;
        &self.cells[i * W + j]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (i, j) = index;
        &mut self.cells[i * W + j]
    }
}

fn main() {
    let f = File::open("input/day8.txt").unwrap();
    let read = BufReader::new(f);
    let lines = read.lines();

    let mut grid: Grid<Tree> = Grid {
        cells: [Tree::default(); W * H],
    };

    // read data
    for (i, line) in lines.enumerate() {
        for (j, c) in line.unwrap().chars().enumerate() {
            let tree = &mut grid[(i, j)];
            tree.height = c
                .to_digit(10)
                .expect("not a digit")
                .try_into()
                .expect("cannot convert");
        }
    }
    print_grid(&grid);
    println!();

    // check from left
    for i in 0..H {
        check_visibility(&mut grid, i*W..(i+1)*W);
    }
    // check from right
    for i in 0..H {
        check_visibility(&mut grid, (i*W..(i+1)*W).rev());
    }
    // check from up
    for j in 0..W {
        check_visibility(&mut grid, (j..j+(H*W)).step_by(W));
    }
    // check from down
    for j in 0..W {
        check_visibility(&mut grid, (j..j+(H*W)).step_by(W).rev());
    }

    print_grid(&grid);
    println!();

    let visible_count: usize = grid.cells.iter().filter(|t| t.visible).count();

    // Scenic score
    let best_score = (0..H*W)
        .map(|pos| scenic_score(&grid, pos))
        .max()
        .unwrap_or(0);

    println!("{}", visible_count);
    println!("{}", best_score);
}

fn check_visibility<I: Iterator<Item = usize>>(grid: &mut Grid<Tree>, indexes: I) {
    indexes
        .fold(None, |acc, i| {
            let tree = &mut grid[i];
            tree.visible |= match acc {
                None => true,
                Some(max_height) => tree.height > max_height
            };
            match acc {
                None => Some(tree.height),
                Some(max_height) => Some(max_height.max(tree.height))
            }
        });
}

fn scenic_score(grid: &Grid<Tree>, pos: usize) -> usize {
    let i = pos / W;
    let j = pos % W;
    let mut score: usize = 1;
    let height = grid[(i,j)].height;
    // check to right
    score *= view_distance(grid, height, pos+1..(i+1)*W);
    // check to left
    score *= view_distance(grid, height, (i*W..pos).rev());
    // check down
    score *= view_distance(grid, height, (pos+W..(j+H*W)).step_by(W));
    // check up
    score *= view_distance(grid, height, (j..pos).step_by(W).rev());
    score
}

fn view_distance<I: Iterator<Item = usize>>(grid: &Grid<Tree>, height: usize, indexes: I) -> usize {
    let mut dist: usize = 0;
    for i in indexes {
        dist += 1;
        if height <= grid[i].height {
            break;
        }
    }
    dist
}

fn print_grid(grid: &Grid<Tree>) {
    for i in 0..H {
        for j in 0..W {
            let tree = &grid[(i,j)];
            if tree.visible {
                print!("\u{001b}[31m{}\u{001b}[0m", tree.height);
            } else {
                print!("{}", tree.height);
            }
        }
        println!();
    }
}
