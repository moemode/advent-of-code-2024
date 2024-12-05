use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

/// Parses the input file into pairs and updates.
///
/// # Arguments
///
/// * `file_path` - Path to the input file.
///
/// # Returns
///
/// A tuple containing:
/// * A vector of `(i64, i64)` pairs.
/// * A vector of `Vec<i64>` updates.
fn parse_input(file_path: &str) -> (Vec<(i64, i64)>, Vec<Vec<i64>>) {
    let input_str = fs::read_to_string(file_path).expect("Failed to read input file");
    let mut lines = input_str.lines();
    let mut pairs = Vec::new();
    let mut updates = Vec::new();
    // Parse the pairs
    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        let mut parts = line.split('|');
        let left = parts.next().unwrap().parse::<i64>().unwrap();
        let right = parts.next().unwrap().parse::<i64>().unwrap();
        pairs.push((left, right));
    }
    // Parse the updates
    for line in lines {
        let update = line
            .split(',')
            .map(|num| num.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        updates.push(update);
    }
    (pairs, updates)
}

/// Constructs a dependency map from pairs of dependencies.
///
/// # Arguments
///
/// * `pairs` - A vector of `(i64, i64)` pairs representing dependencies.
///
/// # Returns
///
/// A `HashMap` where each key is an item and the value is a `HashSet` of items that depend on the key.
fn graph(edges: &Vec<(i64, i64)>) -> HashMap<i64, HashSet<i64>> {
    let mut result = HashMap::new();
    for (left, right) in edges {
        result.entry(*left).or_insert(HashSet::new()).insert(*right);
    }
    result
}

/// Checks if an update obeys the dependency rules.
///
/// # Arguments
///
/// * `update` - A vector of `i64` representing the update sequence.
/// * `dependents` - A `HashMap` of dependencies.
///
/// # Returns
///
/// `true` if the update obeys the dependency rules, `false` otherwise.
fn update_obeys_deps(update: &Vec<i64>, dependents: &HashMap<i64, HashSet<i64>>) -> bool {
    let mut seen = HashSet::new();
    for &num in update {
        if let Some(deps) = dependents.get(&num) {
            if !deps.is_disjoint(&seen) {
                return false;
            }
        }
        seen.insert(num);
    }
    true
}

/// Performs a depth-first search on the graph.
///
/// # Arguments
///
/// * `g` - The graph represented as a `HashMap`.
/// * `node` - The starting node for the DFS.
/// * `unvisited` - A mutable reference to a `HashSet` of unvisited nodes.
/// * `reversed_finishing` - A mutable reference to a `VecDeque` to store the nodes in reverse finishing order.
fn dfs(
    g: &HashMap<i64, HashSet<i64>>,
    node: i64,
    unvisited: &mut HashSet<i64>,
    reversed_finishing: &mut VecDeque<i64>,
) {
    unvisited.remove(&node);
    if let Some(neighbors) = g.get(&node) {
        for neighbor in neighbors {
            if unvisited.contains(neighbor) {
                dfs(g, *neighbor, unvisited, reversed_finishing);
            }
        }
    }
    reversed_finishing.push_front(node);
}

/// Performs a full depth-first search on the graph.
///
/// # Arguments
///
/// * `g` - The graph represented as a `HashMap`.
///
/// # Returns
///
/// A `VecDeque` containing the nodes in reverse finishing order.
/// When the graph is acyclic, this is a topological order.
fn full_dfs(g: &HashMap<i64, HashSet<i64>>) -> VecDeque<i64> {
    let mut reversed_finishing = VecDeque::new();
    let mut unvisited: HashSet<i64> = g.keys().cloned().collect();
    while let Some(&next) = unvisited.iter().next() {
        dfs(g, next, &mut unvisited, &mut reversed_finishing);
    }
    reversed_finishing
}

/// Constructs a subgraph containing only the specified nodes.
///
/// # Arguments
///
/// * `g` - The original graph represented as a `HashMap`.
/// * `nodes` - A vector of nodes to include in the subgraph.
///
/// # Returns
///
/// A `HashMap` representing the subgraph.
fn subgraph(g: &HashMap<i64, HashSet<i64>>, nodes: &Vec<i64>) -> HashMap<i64, HashSet<i64>> {
    let mut result = HashMap::new();
    for node in nodes {
        if let Some(neighbors) = g.get(node) {
            result.insert(
                *node,
                nodes
                    .iter()
                    .filter(|n| neighbors.contains(n))
                    .cloned()
                    .collect(),
            );
        }
    }
    result
}

fn main() {
    let (deps, updates) = parse_input("input.txt");
    let g = graph(&deps);
    let mut sum = 0;
    // add up middle numbers of valid updates
    for update in updates
        .iter()
        .filter(|update| !update_obeys_deps(update, &g))
        .map(|update| full_dfs(&subgraph(&g, update)))
    {
        sum += update[update.len() / 2];
    }
    println!("Sum of middle numbers of valid updates: {}", sum);
}
