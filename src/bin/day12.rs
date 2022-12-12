use std::{
    collections::{VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

const W: usize = 136;
const H: usize = 41;

struct Cell {
    height: usize,
    path_cost: usize,
}

struct SearchNode {
    path_cost: usize,
    pos: (usize, usize),
}

fn main() {
    let f = File::open("input/day12.txt").unwrap();
    let read = BufReader::new(f);
    let mut lines = read.lines();

    let mut cells: Vec<Cell> = vec![];
    let mut start_pos: (usize, usize) = (0, 0);
    let mut end_pos: (usize, usize) = (0, 0);

    // Parse
    let mut pos: usize = 0;
    loop {
        let Some(Ok(line)) = lines.next() else {
            break;
        };

        assert_eq!(line.len(), W);

        for c in line.chars() {
            let height: usize = match c {
                'S' => {
                    start_pos = to_cartesian(pos);
                    0
                }
                'E' => {
                    end_pos = to_cartesian(pos);
                    25
                }
                'a'..='z' => c as usize - 'a' as usize,
                _ => panic!("invalid char"),
            };
            cells.push(Cell {
                height,
                path_cost: usize::MAX,
            });
            pos += 1;
        }
    }

    // Build SPT backwards
    let mut queue: VecDeque<SearchNode> = VecDeque::new();
    queue.push_back(SearchNode {
        path_cost: 0,
        pos: end_pos,
    });

    while let Some(node) = queue.pop_front() {
        // decide whether to visit
        if cells[to_flat(node.pos)].path_cost > node.path_cost {
            cells[to_flat(node.pos)].path_cost = node.path_cost;
        } else {
            continue;
        }

        let (i, j) = node.pos;
        let height = cells[to_flat(node.pos)].height;
        // expand
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            // move
            let i_succ = i as isize + dx;
            let j_succ = j as isize + dy;
            if i_succ < 0 || i_succ >= H as isize || j_succ < 0 || j_succ >= W as isize {
                continue;
            }
            let pos_succ = (i_succ as usize, j_succ as usize);

            // check feasibility
            let height_succ = cells[to_flat(pos_succ)].height;
            if height as isize - height_succ as isize > 1 {
                continue;
            }

            // enqueue node
            let succ = SearchNode {
                path_cost: node.path_cost + 1,
                pos: pos_succ,
            };
            queue.push_back(succ);
        }
    }

    // part 1
    let min_cost = cells[to_flat(start_pos)].path_cost;
    println!("{}", min_cost);

    // part 2
    let min_cost = cells
        .iter()
        .filter(|c| c.height == 0)
        .map(|c| c.path_cost)
        .min()
        .unwrap();
    println!("{}", min_cost)
}

fn to_flat(pos: (usize, usize)) -> usize {
    pos.0 * W + pos.1
}

fn to_cartesian(pos: usize) -> (usize, usize) {
    (pos / W, pos % W)
}
