use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
    fmt::Display,
    fs::File,
    hash::{Hash, Hasher},
    io::{BufRead, BufReader},
};

const TIME_BUDGET: usize = 24;

const NUM_RES: usize = 4;

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

type Res = usize;

type ResVec = [usize; NUM_RES];

type Blueprint = [ResVec; NUM_RES];

#[derive(Debug)]
struct SearchNode {
    t: usize,
    robots: ResVec,
    resources: ResVec,
    upper_bound: usize,
}

impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.robots == other.robots && self.resources == other.resources
    }
}

impl Eq for SearchNode {}

impl Hash for SearchNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.t.hash(state);
        self.robots.hash(state);
        self.resources.hash(state);
    }
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.upper_bound.cmp(&other.upper_bound)
    }
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl SearchNode {
    fn initial(costs: &Blueprint) -> Self {
        Self {
            t: 0,
            robots: [1, 0, 0, 0],
            resources: [0; NUM_RES],
            upper_bound: compute_bound(0, &[0; NUM_RES], &[1, 0, 0, 0], costs),
        }
    }

    fn can_build(resources: &ResVec, cost: &ResVec) -> bool {
        resources
            .iter()
            .zip(cost)
            .map(|(avail, cost)| avail.checked_sub(*cost))
            .all(|x| x.is_some())
    }

    fn step_resources(resources: &mut ResVec, robots: &ResVec) {
        for i in 0..NUM_RES {
            resources[i] += robots[i];
        }
    }

    fn pay_cost(resources: &mut ResVec, cost: &ResVec) {
        for i in 0..NUM_RES {
            resources[i] -= cost[i];
        }
    }

    fn utility(&self) -> usize {
        self.resources[GEODE] as usize
    }

    fn expand(&self, costs: &Blueprint) -> Vec<SearchNode> {
        let mut children = Vec::new();
        if self.t == TIME_BUDGET {
            return children;
        }

        // action: wait until resources are available and build each type of robot
        let mut child_t = self.t;
        let mut cur_resources = self.resources;

        let mut rob_built = [false; NUM_RES]; // types of robots already accounted for

        loop {
            child_t += 1;
            // terminate
            if child_t == TIME_BUDGET {
                if children.is_empty() {
                    Self::step_resources(&mut cur_resources, &self.robots);
                    let child = SearchNode {
                        t: child_t,
                        resources: cur_resources,
                        robots: self.robots,
                        upper_bound: compute_bound(child_t, &cur_resources, &self.robots, costs),
                    };
                    children.push(child);
                }
                break;
            }

            // try to build a robot
            for (rob, cost) in costs.iter().enumerate() {
                // action: build robot
                if rob_built[rob] || !Self::can_build(&cur_resources, cost) {
                    continue;
                };
                let mut child_resources = cur_resources;
                Self::pay_cost(&mut child_resources, cost);
                Self::step_resources(&mut child_resources, &self.robots);
                let mut child_robots = self.robots;
                child_robots[rob] += 1;

                let child = SearchNode {
                    t: child_t,
                    resources: child_resources,
                    robots: child_robots,
                    upper_bound: compute_bound(child_t, &child_resources, &child_robots, costs),
                };
                rob_built[rob] = true;
                children.push(child);
            }

            // do nothing
            if rob_built != [true; NUM_RES] {
                Self::step_resources(&mut cur_resources, &self.robots); // do nothing for this step
            } else {
                break;
            }
        }

        children
    }
}

impl Display for SearchNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SearchNode(t: {}, res: {:?}, rob: {:?}, bound: {})",
            self.t, self.resources, self.robots, self.upper_bound
        )
    }
}

fn compute_bound(t: usize, resources: &ResVec, robots: &ResVec, costs: &Blueprint) -> usize {
    let dt = TIME_BUDGET - t;

    let spec_ore = resources[ORE] + dt * robots[ORE];
    let spec_ore = spec_ore + (spec_ore / costs[ORE][ORE]).min(dt) / 2;
    let spec_clay =
        resources[CLAY] + dt * robots[CLAY] + dt * (spec_ore / costs[CLAY][ORE]).min(dt) / 2;
    let spec_obs = resources[OBSIDIAN]
        + dt * robots[OBSIDIAN]
        + dt * usize::min(
            spec_ore / costs[OBSIDIAN][ORE],
            spec_clay / costs[OBSIDIAN][CLAY],
        )
        .min(dt) / 2;
    let spec_geode = resources[GEODE]
        + dt * robots[GEODE]
        + dt * usize::min(
            spec_ore / costs[GEODE][ORE],
            spec_obs / costs[GEODE][OBSIDIAN],
        )
        .min(dt) / 2;

    spec_geode as usize
}

fn parse_res(s: &str) -> Res {
    match s {
        "ore" => ORE,
        "clay" => CLAY,
        "obsidian" => OBSIDIAN,
        "geode" => GEODE,
        _ => panic!(),
    }
}

fn parse_blueprint(line: &str) -> Blueprint {
    let mut blue = Blueprint::default();
    let Some(s) = line.split(": ").nth(1) else {
        panic!();
    };

    for s in s.split(". ") {
        let s = s.trim_end_matches('.');
        let s = s.trim_start_matches("Each ");

        let (rob, s) = s.split_once(' ').unwrap();
        let rob: Res = parse_res(rob);

        let s = s.trim_start_matches("robot costs ");
        for s in s.split("and ") {
            let (n, s) = s.split_once(' ').unwrap();
            let n: usize = n.parse().unwrap();

            let res = s.trim_end();
            let res: Res = parse_res(res);

            blue[rob][res] = n;
        }
    }
    blue
}

fn main() {
    let f = File::open("input/day19.txt").unwrap();
    let read = BufReader::new(f);
    let lines = read.lines();

    let mut blueprints = Vec::<Blueprint>::new();

    for line in lines {
        let b = parse_blueprint(&line.unwrap());
        blueprints.push(b);
    }

    // dbg!(compute_bound(
    //     0,
    //     &[0, 0, 0, 0],
    //     &[1, 0, 0, 0],
    //     &blueprints[0]
    // ));
    // dbg!(compute_bound(
    //     1,
    //     &[1, 0, 0, 0],
    //     &[1, 0, 0, 0],
    //     &blueprints[0]
    // ));
    // dbg!(compute_bound(
    //     2,
    //     &[2, 0, 0, 0],
    //     &[1, 0, 0, 0],
    //     &blueprints[0]
    // ));
    // dbg!(compute_bound(
    //     3,
    //     &[1, 0, 0, 0],
    //     &[1, 1, 0, 0],
    //     &blueprints[0]
    // ));
    // dbg!(compute_bound(
    //     4,
    //     &[2, 1, 0, 0],
    //     &[2, 1, 0, 0],
    //     &blueprints[0]
    // ));
    // dbg!(compute_bound(
    //     5,
    //     &[1, 2, 0, 0],
    //     &[2, 2, 0, 0],
    //     &blueprints[0]
    // ));
    // dbg!(compute_bound(
    //     6,
    //     &[2, 4, 0, 0],
    //     &[2, 2, 0, 0],
    //     &blueprints[0]
    // ));
    // dbg!(compute_bound(
    //     7,
    //     &[1, 6, 0, 0],
    //     &[2, 3, 0, 0],
    //     &blueprints[0]
    // ));
    // dbg!(compute_bound(
    //     8,
    //     &[2, 9, 0, 0],
    //     &[2, 3, 0, 0],
    //     &blueprints[0]
    // ));
    // dbg!(compute_bound(
    //     9,
    //     &[3, 12, 0, 0],
    //     &[2, 3, 0, 0],
    //     &blueprints[0]
    // ));
    // dbg!(compute_bound(
    //     10,
    //     &[4, 15, 0, 0],
    //     &[2, 3, 0, 0],
    //     &blueprints[0]
    // ));
    // dbg!(compute_bound(
    //     11,
    //     &[2, 4, 0, 0],
    //     &[2, 3, 1, 0],
    //     &blueprints[0]
    // ));
    // dbg!(compute_bound(
    //     12,
    //     &[1, 7, 1, 0],
    //     &[2, 4, 1, 0],
    //     &blueprints[0]
    // ));
    // dbg!(compute_bound(
    //     13,
    //     &[2, 11, 2, 0],
    //     &[2, 4, 1, 0],
    //     &blueprints[0]
    // ));
    // dbg!(compute_bound(
    //     14,
    //     &[3, 15, 3, 0],
    //     &[2, 4, 1, 0],
    //     &blueprints[0]
    // ));
    // dbg!(compute_bound(
    //     15,
    //     &[1, 5, 4, 0],
    //     &[2, 4, 2, 0],
    //     &blueprints[0]
    // ));
    // dbg!(compute_bound(
    //     16,
    //     &[2, 9, 6, 0],
    //     &[2, 4, 2, 0],
    //     &blueprints[0]
    // ));
    // dbg!(compute_bound(
    //     17,
    //     &[3, 13, 8, 0],
    //     &[2, 4, 2, 0],
    //     &blueprints[0]
    // ));
    // dbg!(compute_bound(
    //     18,
    //     &[2, 17, 3, 0],
    //     &[2, 4, 2, 1],
    //     &blueprints[0]
    // ));
    // dbg!(compute_bound(
    //     19,
    //     &[3, 21, 5, 1],
    //     &[2, 4, 2, 1],
    //     &blueprints[0]
    // ));
    // dbg!(compute_bound(
    //     20,
    //     &[4, 25, 7, 2],
    //     &[2, 4, 2, 1],
    //     &blueprints[0]
    // ));
    // dbg!(compute_bound(
    //     21,
    //     &[3, 29, 2, 3],
    //     &[2, 4, 2, 2],
    //     &blueprints[0]
    // ));
    // dbg!(compute_bound(
    //     22,
    //     &[4, 33, 4, 5],
    //     &[2, 4, 2, 2],
    //     &blueprints[0]
    // ));
    // dbg!(compute_bound(
    //     23,
    //     &[5, 37, 6, 7],
    //     &[2, 4, 2, 2],
    //     &blueprints[0]
    // ));
    // dbg!(compute_bound(
    //     24,
    //     &[6, 41, 8, 9],
    //     &[2, 4, 2, 2],
    //     &blueprints[0]
    // ));

    let mut quality_levels = Vec::<usize>::new();

    for (i, blueprint) in blueprints.iter().enumerate() {
        println!("Blueprint {}", i+1);
        let mut min_bound: usize = 0;

        // DFS
        let mut frontier = VecDeque::<SearchNode>::new();
        let mut expanded = HashSet::<SearchNode>::new();

        let initial_node = SearchNode::initial(blueprint);
        println!("Initial {}", initial_node);
        frontier.push_front(initial_node);

        while let Some(node) = frontier.pop_front() {
            if node.upper_bound < min_bound {
                continue;
            }
            // println!("{}", node);

            // expand
            let children = node.expand(blueprint);

            // terminal state
            if children.is_empty() && node.utility() > min_bound {
                println!("New max {} for {}", node.utility(), node);
                min_bound = node.utility();
            }

            for child in children {
                if !expanded.contains(&child) && child.upper_bound > min_bound {
                    // println!("\t-> {}", child);
                    frontier.push_front(child);
                } else {
                    // println!("\t-- {}", child);
                }
            }

            expanded.insert(node);
        }

        quality_levels.push((i+1) * min_bound);    
    }

    let ql_sum: usize = quality_levels.iter().sum();
    println!("{}", ql_sum);
}
