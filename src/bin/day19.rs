use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
    fmt::Display,
    fs::File,
    hash::{Hash, Hasher},
    io::{BufRead, BufReader},
};

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
    time_budget: usize,
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
    fn initial(time_budget: usize, costs: &Blueprint) -> Self {
        Self {
            t: 0,
            time_budget,
            robots: [1, 0, 0, 0],
            resources: [0; NUM_RES],
            upper_bound: compute_bound(time_budget, &[0; NUM_RES], &[1, 0, 0, 0], costs),
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
        self.resources[GEODE]
    }

    fn expand(&self, costs: &Blueprint) -> Vec<SearchNode> {
        let mut children = Vec::new();
        if self.t == self.time_budget {
            return children;
        }

        // action: wait until resources are available and build each type of robot
        let mut child_t = self.t;
        let mut cur_resources = self.resources;

        let mut rob_built = [false; NUM_RES]; // types of robots already accounted for

        loop {
            child_t += 1;
            // terminate
            if child_t == self.time_budget {
                if children.is_empty() {
                    Self::step_resources(&mut cur_resources, &self.robots);
                    let child = SearchNode {
                        t: child_t,
                        time_budget: self.time_budget,
                        resources: cur_resources,
                        robots: self.robots,
                        upper_bound: compute_bound(
                            self.time_budget - child_t,
                            &cur_resources,
                            &self.robots,
                            costs,
                        ),
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
                    time_budget: self.time_budget,
                    resources: child_resources,
                    robots: child_robots,
                    upper_bound: compute_bound(
                        self.time_budget - child_t,
                        &child_resources,
                        &child_robots,
                        costs,
                    ),
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

fn compute_bound(
    remaining_time: usize,
    resources: &ResVec,
    robots: &ResVec,
    costs: &Blueprint,
) -> usize {
    let dt = remaining_time;

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
        .min(dt)
            / 2;
    let spec_geode = resources[GEODE]
        + dt * robots[GEODE]
        + dt * usize::min(
            spec_ore / costs[GEODE][ORE],
            spec_obs / costs[GEODE][OBSIDIAN],
        )
        .min(dt)
            / 2;

    #[allow(clippy::let_and_return)]
    spec_geode
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

fn solve_blueprint(time_budget: usize, costs: &Blueprint) -> usize {
    let mut min_bound: usize = 0;

    // DFS
    let mut frontier = VecDeque::<SearchNode>::new();
    let mut expanded = HashSet::<SearchNode>::new();

    let initial_node = SearchNode::initial(time_budget, costs);
    println!("Initial {}", initial_node);
    frontier.push_front(initial_node);

    while let Some(node) = frontier.pop_front() {
        if node.upper_bound < min_bound {
            continue;
        }
        // println!("{}", node);

        // expand
        let children = node.expand(costs);

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

    min_bound
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

    // part 1
    println!("Part 1");
    let ql_sum: usize = blueprints
        .iter()
        .enumerate()
        .map(|(i, costs)| {
            println!("Blueprint {}", i);

            let sol = solve_blueprint(24, costs);
            sol * i
        })
        .sum();

    println!("{}", ql_sum);

    // part 2
    println!("Part 2");
    let sol_prod: usize = blueprints
        .iter()
        .take(3)
        .enumerate()
        .map(|(i, costs)| {
            println!("Blueprint {}", i);
            solve_blueprint(32, costs)
        })
        .product();

    println!("{}", sol_prod);
}
