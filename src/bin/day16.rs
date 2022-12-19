use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use scanf::sscanf;

const TIMESPAN: usize = 30;
const INITIAL_VALVE: &str = "AA";

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: usize,
    neighbors: Vec<String>,
}

impl FromStr for Valve {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once("; ").ok_or("no separator ;")?;

        let mut name = String::new();
        let mut flow_rate: usize = 0;
        sscanf!(first, "Valve {} has flow rate={}", name, flow_rate)
            .map_err(|_| "cannot parse first part")?;

        let second = second
            .trim_start_matches("tunnel leads to valve ")
            .trim_start_matches("tunnels lead to valves ");

        let out_edges: Vec<String> = second.split(", ").map(|s| s.to_string()).collect();

        Ok(Self {
            name,
            flow_rate,
            neighbors: out_edges,
        })
    }
}

#[derive(Debug, Clone)]
struct SearchNode {
    t: usize,                 // time
    valve: String,            // current valve
    open_valves: Vec<String>, // valves currently open
    path_gain: usize,
}

impl SearchNode {
    fn initial(valve: &Valve) -> Self {
        Self {
            t: 0,
            valve: valve.name.clone(),
            open_valves: Vec::new(),
            path_gain: 0,
        }
    }

    fn heuristic(&self, sorted_valves: &[(String, usize)]) -> usize {
        sorted_valves
            .iter()
            .filter(|(n, _)| !self.open_valves.contains(n))
            .enumerate()
            .map(|(i, (_, rate))| {
                let t = self.t + 2 * (i + 1);
                if t > TIMESPAN {
                    0
                } else {
                    rate * (TIMESPAN - t)
                }
            })
            .sum()
    }

    fn evaluate(&self, sorted_valves: &[(String, usize)]) -> usize {
        self.path_gain + self.heuristic(sorted_valves)
    }

    fn expand(
        &self,
        valves: &HashMap<String, Valve>,
        routes: &HashMap<String, HashMap<String, (usize, String)>>,
    ) -> Vec<SearchNode> {
        let mut children: Vec<SearchNode> = Vec::new();
        if self.t >= TIMESPAN {
            return children;
        }

        let valve = valves.get(&self.valve).expect("valve not found");
        let routes = routes.get(&valve.name).expect("cannot find routes");

        for (dest, (dist, _)) in routes.iter() {
            // action: go to another valve and open it
            if !self.open_valves.contains(dest) {
                let dest = valves.get(dest).expect("valve not found");
                if dest.flow_rate == 0 {
                    // useless
                    continue;
                }
                let t = self.t + dist + 1;
                if t > TIMESPAN {
                    continue;
                }
                let mut open_valves = self.open_valves.clone();
                open_valves.push(dest.name.clone());
                let path_gain = self.path_gain + dest.flow_rate * (TIMESPAN - t);
                let next_node = SearchNode {
                    t,
                    valve: dest.name.clone(),
                    open_valves,
                    path_gain,
                };

                children.push(next_node);
            }
        }
        children
    }
}

impl Display for SearchNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SearchNode(t: {}, v: {:?}, path: {:?}, pg: {})",
            self.t, self.valve, self.open_valves, self.path_gain
        )
    }
}

impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.valve == other.valve && self.open_valves == other.open_valves
    }
}

impl Eq for SearchNode {}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path_gain.cmp(&other.path_gain)
    }
}

fn bfs(source: &Valve, valves: &HashMap<String, Valve>) -> HashMap<String, (usize, String)> {
    let mut routes: HashMap<String, (usize, String)> = HashMap::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<String> = VecDeque::new();

    queue.push_back(source.name.clone());
    visited.insert(source.name.clone());

    while let Some(name) = queue.pop_front() {
        let valve = valves.get(&name).expect("cannot find valve");
        for n in valve.neighbors.iter() {
            if !visited.contains(n) {
                // update distance
                let dist = routes.get(&name).map(|x| x.0).unwrap_or(0);
                routes.insert(n.clone(), (dist + 1, name.clone()));
                // enqueue node
                queue.push_back(n.clone());
                visited.insert(n.clone());
            }
        }
    }

    routes
}

fn main() {
    let f = File::open("input/day16.txt").unwrap();
    let read = BufReader::new(f);
    let lines = read.lines();

    let valves: HashMap<String, Valve> = lines
        .map(|l| {
            let valve: Valve = l.expect("cannot read line").parse().expect("cannot parse");
            (valve.name.clone(), valve)
        })
        .collect();
    let mut routes: HashMap<String, HashMap<String, (usize, String)>> = HashMap::new();

    let mut sorted_valves: Vec<(String, usize)> = valves
        .values()
        .filter(|v| v.flow_rate > 0)
        .map(|v| (v.name.clone(), v.flow_rate))
        .collect();
    sorted_valves.sort_by_key(|(_, rate)| Reverse(*rate));

    // find shortest paths from each interesting source
    for source in valves
        .values()
        .filter(|v| v.name == INITIAL_VALVE || v.flow_rate > 0)
    {
        let src_routes = bfs(source, &valves);
        routes.insert(source.name.clone(), src_routes);
    }

    // show routes
    // for (s, rs) in routes.iter() {
    //     println!("Routes from {}", s);
    //     for (dest, (dist, via)) in rs.iter() {
    //         println!("\t{}: {} via {}", dest, dist, via);
    //     }
    // }

    // search (part 1)
    #[allow(clippy::mutable_key_type)]
    let mut frontier: BinaryHeap<SearchNode> = BinaryHeap::new(); // highest path_gain first
    let initial_node = SearchNode::initial(
        valves
            .get(INITIAL_VALVE)
            .expect("cannot find initial valve"),
    );
    frontier.push(initial_node);

    let mut max_path_gain: usize = 0;
    while let Some(node) = frontier.pop() {
        if max_path_gain >= 2112 {
            // println!("{}", node);
        }

        // prune
        if node.evaluate(&sorted_valves) < max_path_gain {
            // println!("pruning {} {}", node, node.evaluate(&sorted_valves));
            continue;
        }

        let children = node.expand(&valves, &routes);

        // terminal state
        if children.is_empty() {
            if node.path_gain > max_path_gain {
                println!("New max {}", node.path_gain);
                println!("\t for {}", node);
                max_path_gain = node.path_gain;
            }
            continue;
        }

        for child in children {
            if max_path_gain >= 2112 {
                // println!("\t{}", child.borrow_mut());
            }
            frontier.push(child);
        }
    }

    println!("{}", max_path_gain);
}
