use std::{
    cmp::{Ordering, Reverse},
    collections::{hash_map::Entry, BinaryHeap, HashMap},
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
        distances: &HashMap<String, HashMap<String, usize>>,
    ) -> Vec<SearchNode> {
        let mut children: Vec<SearchNode> = Vec::new();
        if self.t >= TIMESPAN {
            return children;
        }

        let valve = valves.get(&self.valve).expect("valve not found");
        let distances = &distances[&valve.name];

        for (dest, dist) in distances.iter() {
            // action: go to another valve and open it
            if !self.open_valves.contains(dest) {
                let dest = valves.get(dest).expect("valve not found");
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

#[derive(Debug, Clone)]
struct JointSearchNode(SearchNode, SearchNode);

impl JointSearchNode {
    fn initial(valve: &Valve) -> Self {
        let node_0 = SearchNode {
            t: 4,
            valve: valve.name.clone(),
            open_valves: Vec::new(),
            path_gain: 0,
        };
        let node_1 = node_0.clone();
        Self(node_0, node_1)
    }

    fn path_gain(&self) -> usize {
        self.0.path_gain + self.1.path_gain
    }

    fn heuristic(&self, sorted_valves: &[(String, usize)]) -> usize {
        self.0.heuristic(sorted_valves) + self.1.heuristic(sorted_valves)
    }

    fn evaluate(&self, sorted_valves: &[(String, usize)]) -> usize {
        self.path_gain() + self.heuristic(sorted_valves)
    }

    fn expand(
        &self,
        valves: &HashMap<String, Valve>,
        distances: &HashMap<String, HashMap<String, usize>>,
    ) -> Vec<JointSearchNode> {
        let mut children = Vec::new();

        let mut children_0 = self.0.expand(valves, distances);
        let mut children_1 = self.1.expand(valves, distances);

        // if there is no other option, allow one of the two to stall
        if children_0.is_empty() && children_1.is_empty() {
            // terminal state
            return children;
        }
        if children_0.is_empty() {
            children_0.push(self.0.clone());
        }
        if children_1.is_empty() {
            children_1.push(self.1.clone());
        }

        // create a child for each pair
        for c0 in children_0.iter() {
            for c1 in children_1.iter() {
                // don't open the same valve
                if c0.valve == c1.valve {
                    continue;
                }

                // update the list of open valves
                let mut c0 = c0.clone();
                let mut c1 = c1.clone();
                c0.open_valves.push(c1.valve.clone());
                c1.open_valves.push(c0.valve.clone());

                children.push(Self(c0, c1));
            }
        }

        children
    }
}

impl PartialEq for JointSearchNode {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1)
        || (self.0 == other.1 && self.1 == other.0)
    }
}

impl Eq for JointSearchNode {}

impl PartialOrd for JointSearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JointSearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path_gain().cmp(&other.path_gain())
    }
}

impl Display for JointSearchNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JointSearchNode\n\t{}\n\t{})", self.0, self.1)
    }
}

fn shortest_paths(valves: &HashMap<String, Valve>) -> HashMap<String, HashMap<String, usize>> {
    let mut dist = HashMap::new();

    // init
    for valve in valves.values() {
        let distances_v: &mut HashMap<String, usize> = dist
            .entry(valve.name.clone())
            .or_insert_with(|| [(valve.name.clone(), 0)].into_iter().collect());
        for dest in valve.neighbors.iter() {
            distances_v.insert(dest.clone(), 1);
        }
    }

    for k in valves.keys() {
        for i in valves.keys() {
            for j in valves.keys() {
                let Some(&dist_ik) = dist[i].get(k) else {
                    continue;
                };
                let Some(&dist_kj) = dist[k].get(j) else {
                    continue;
                };
                let &dist_ij = dist[i].get(j).unwrap_or(&usize::MAX);
                if dist_ij > dist_ik + dist_kj {
                    match dist.get_mut(i).unwrap().entry(j.clone()) {
                        Entry::Occupied(mut e) => {
                            e.insert(dist_ik + dist_kj);
                        }
                        Entry::Vacant(e) => {
                            e.insert(dist_ik + dist_kj);
                        }
                    };
                }
            }
        }
    }

    dist
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

    let mut sorted_valves: Vec<(String, usize)> = valves
        .values()
        .filter(|v| v.flow_rate > 0)
        .map(|v| (v.name.clone(), v.flow_rate))
        .collect();
    sorted_valves.sort_by_key(|(_, rate)| Reverse(*rate));

    // find shortest paths from each interesting source
    let mut distances = shortest_paths(&valves);
    distances.retain(|i, ds| {
        let valve = &valves[i];
        if valve.flow_rate > 0 || valve.name == INITIAL_VALVE {
            ds.retain(|j, _| j != i && valves[j].flow_rate > 0);
            true
        } else {
            false
        }
    });

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
        // prune
        if node.evaluate(&sorted_valves) < max_path_gain {
            // println!("pruning {} {}", node, node.evaluate(&sorted_valves));
            continue;
        }

        let children = node.expand(&valves, &distances);

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
            frontier.push(child);
        }
    }

    println!("{}", max_path_gain);

    // search (part 2)
    #[allow(clippy::mutable_key_type)]
    let mut frontier: BinaryHeap<JointSearchNode> = BinaryHeap::new(); // highest path_gain first
    let initial_node = JointSearchNode::initial(
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

        let children = node.expand(&valves, &distances);

        // terminal state
        if children.is_empty() {
            if node.path_gain() > max_path_gain {
                println!("New max {}", node.path_gain());
                println!("\t for {}", node);
                max_path_gain = node.path_gain();
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
