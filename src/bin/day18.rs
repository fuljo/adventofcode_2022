use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Face {
    N = 0,
    S = 1,
    E = 2,
    W = 3,
    U = 4,
    D = 5,
}

type AdjacentFaceCollection = &'static [&'static [((isize, isize, isize), Face)]];

impl Face {
    fn adjacent_faces(&self) -> AdjacentFaceCollection {
        match self {
            // 90* angle, 180* angle, 270* angle
            Self::N => &[
                &[
                    ((1, 1, 0), Self::W),
                    ((1, 0, 0), Self::N),
                    ((0, 0, 0), Self::E),
                ],
                &[
                    ((-1, 1, 0), Self::E),
                    ((-1, 0, 0), Self::N),
                    ((0, 0, 0), Self::W),
                ],
                &[
                    ((0, 1, 1), Self::D),
                    ((0, 0, 1), Self::N),
                    ((0, 0, 0), Self::U),
                ],
                &[
                    ((0, 1, -1), Self::U),
                    ((0, 0, -1), Self::N),
                    ((0, 0, 0), Self::D),
                ],
            ],
            Self::S => &[
                &[
                    ((1, -1, 0), Self::W),
                    ((1, 0, 0), Self::S),
                    ((0, 0, 0), Self::E),
                ],
                &[
                    ((-1, -1, 0), Self::E),
                    ((-1, 0, 0), Self::S),
                    ((0, 0, 0), Self::W),
                ],
                &[
                    ((0, -1, 1), Self::D),
                    ((0, 0, 1), Self::S),
                    ((0, 0, 0), Self::U),
                ],
                &[
                    ((0, -1, -1), Self::U),
                    ((0, 0, -1), Self::S),
                    ((0, 0, 0), Self::D),
                ],
            ],
            Self::E => &[
                &[
                    ((1, 1, 0), Self::S),
                    ((0, 1, 0), Self::E),
                    ((0, 0, 0), Self::N),
                ],
                &[
                    ((1, -1, 0), Self::N),
                    ((0, -1, 0), Self::E),
                    ((0, 0, 0), Self::S),
                ],
                &[
                    ((1, 0, 1), Self::D),
                    ((0, 0, 1), Self::E),
                    ((0, 0, 0), Self::U),
                ],
                &[
                    ((1, 0, -1), Self::U),
                    ((0, 0, -1), Self::E),
                    ((0, 0, 0), Self::D),
                ],
            ],
            Self::W => &[
                &[
                    ((-1, 1, 0), Self::S),
                    ((0, 1, 0), Self::W),
                    ((0, 0, 0), Self::N),
                ],
                &[
                    ((-1, -1, 0), Self::N),
                    ((0, -1, 0), Self::W),
                    ((0, 0, 0), Self::S),
                ],
                &[
                    ((-1, 0, 1), Self::D),
                    ((0, 0, 1), Self::W),
                    ((0, 0, 0), Self::U),
                ],
                &[
                    ((-1, 0, -1), Self::U),
                    ((0, 0, -1), Self::W),
                    ((0, 0, 0), Self::D),
                ],
            ],
            Self::U => &[
                &[
                    ((1, 0, 1), Self::W),
                    ((1, 0, 0), Self::U),
                    ((0, 0, 0), Self::E),
                ],
                &[
                    ((-1, 0, 1), Self::E),
                    ((-1, 0, 0), Self::U),
                    ((0, 0, 0), Self::W),
                ],
                &[
                    ((0, 1, 1), Self::S),
                    ((0, 1, 0), Self::U),
                    ((0, 0, 0), Self::N),
                ],
                &[
                    ((0, -1, 1), Self::N),
                    ((0, -1, 0), Self::U),
                    ((0, 0, 0), Self::S),
                ],
            ],
            Self::D => &[
                &[
                    ((1, 0, -1), Self::W),
                    ((1, 0, 0), Self::D),
                    ((0, 0, 0), Self::E),
                ],
                &[
                    ((-1, 0, -1), Self::E),
                    ((-1, 0, 0), Self::D),
                    ((0, 0, 0), Self::W),
                ],
                &[
                    ((0, 1, -1), Self::S),
                    ((0, 1, 0), Self::D),
                    ((0, 0, 0), Self::N),
                ],
                &[
                    ((0, -1, -1), Self::N),
                    ((0, -1, 0), Self::D),
                    ((0, 0, 0), Self::S),
                ],
            ],
        }
    }
}

type FacesIndex = HashSet<((isize, isize, isize), Face)>;

fn main() {
    let f = File::open("input/day18.txt").unwrap();
    let read = BufReader::new(f);
    let mut lines = read.lines();

    let mut open_faces = FacesIndex::new();
    let mut xrange: (isize, isize) = (0, 0);
    let mut yrange: (isize, isize) = (0, 0);
    let mut zrange: (isize, isize) = (0, 0);

    // iterate over cubes
    while let Some(Ok(line)) = lines.next() {
        // parse
        let mut it = line.split(',');
        let x: isize = it.next().unwrap().parse().unwrap();
        let y: isize = it.next().unwrap().parse().unwrap();
        let z: isize = it.next().unwrap().parse().unwrap();

        xrange = (xrange.0.min(x), xrange.1.max(x));
        yrange = (yrange.0.min(y), yrange.1.max(y));
        zrange = (zrange.0.min(z), zrange.1.max(z));

        // check each face: occlude opposite face or mark visible
        if !open_faces.remove(&((x, y + 1, z), Face::S)) {
            open_faces.insert(((x, y, z), Face::N));
        }
        if !open_faces.remove(&((x, y - 1, z), Face::N)) {
            open_faces.insert(((x, y, z), Face::S));
        }
        if !open_faces.remove(&((x + 1, y, z), Face::W)) {
            open_faces.insert(((x, y, z), Face::E));
        }
        if !open_faces.remove(&((x - 1, y, z), Face::E)) {
            open_faces.insert(((x, y, z), Face::W));
        }
        if !open_faces.remove(&((x, y, z + 1), Face::D)) {
            open_faces.insert(((x, y, z), Face::U));
        }
        if !open_faces.remove(&((x, y, z - 1), Face::U)) {
            open_faces.insert(((x, y, z), Face::D));
        }
    }

    // part 1
    let open_faces_count: usize = open_faces.len();

    println!("{}", open_faces_count);

    // find an external face to start from
    let res = open_faces
        .iter()
        .filter(|(_, of)| of == &Face::U)
        .max_by_key(|((_, _, z), _)| z);

    let Some(&initial) = res else {
        panic!("No starting point");
    };

    let mut frontier: VecDeque<((isize, isize, isize), Face)> = VecDeque::new();
    let mut expanded: FacesIndex = FacesIndex::new();
    frontier.push_back(initial);

    while let Some(node) = frontier.pop_front() {
        if !expanded.insert(node) {
            continue;
        }

        // visit adjacent faces
        let ((x, y, z), f) = node;

        for &group in f.adjacent_faces() {
            // in each group, visit the first face that exists: the others are covered
            for ((dx, dy, dz), af) in group {
                let adj = ((x + dx, y + dy, z + dz), *af);
                if open_faces.contains(&adj) {
                    if !expanded.contains(&adj) {
                        frontier.push_back(adj);
                    }
                    break;
                }
            }
        }
    }

    let external_faces_count = expanded.len();
    println!("{}", external_faces_count);
}
