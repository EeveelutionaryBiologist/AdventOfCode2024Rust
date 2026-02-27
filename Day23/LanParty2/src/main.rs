use std::collections::{HashMap, HashSet};
use std::fs;

fn bron_kerbosch_v2(
    r: &HashSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
    g: &HashMap<String, HashSet<String>>,
    cliques: &mut Vec<Vec<String>>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > 2 {
            let mut clique: Vec<String> = r.iter().cloned().collect();
            clique.sort();
            cliques.push(clique);
        }
        return;
    }

    // Choose a pivot with the maximum degree in P ∪ X
    let pivot = p
        .union(x)
        .max_by_key(|v| g.get(*v).map_or(0, |neighbors| neighbors.len()))
        .cloned();

    if let Some(pivot_vertex) = pivot {
        let neighbors = g.get(&pivot_vertex).cloned().unwrap_or_default();
        let candidates: Vec<String> = p.difference(&neighbors).cloned().collect();

        for v in candidates {
            // New R is R ∪ {v}
            let mut new_r = r.clone();
            new_r.insert(v.clone());

            // New P is P ∩ N(v)
            let neighbors_v = g.get(&v).cloned().unwrap_or_default();
            let mut new_p = p
                .intersection(&neighbors_v)
                .cloned()
                .collect::<HashSet<String>>();

            // New X is X ∩ N(v)
            let mut new_x = x
                .intersection(&neighbors_v)
                .cloned()
                .collect::<HashSet<String>>();

            // Recursive call
            bron_kerbosch_v2(&new_r, &mut new_p, &mut new_x, g, cliques);

            // Move v from P to X
            p.remove(&v);
            x.insert(v);
        }
    }
}

fn parse_puzzle_input() -> Vec<(String, String)> {
    let input = fs::read_to_string("puzzle_input.txt").expect("Wo input?");

    let edges: Vec<(String, String)> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let node_a = line[0..2].to_string();
            let node_b = line[3..5].to_string();
            (node_a, node_b)
        })
        .collect();

    edges
}

fn main() {
    let edges = parse_puzzle_input();

    // Build the graph as an adjacency list
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    for (src, dest) in edges.iter() {
        graph.entry(src.clone()).or_default().insert(dest.clone());
        graph.entry(dest.clone()).or_default().insert(src.clone());
    }

    // Initialize R, P, X
    let r: HashSet<String> = HashSet::new();
    let mut p: HashSet<String> = graph.keys().cloned().collect();
    let mut x: HashSet<String> = HashSet::new();

    // Collect cliques
    let mut cliques: Vec<Vec<String>> = Vec::new();
    bron_kerbosch_v2(&r, &mut p, &mut x, &graph, &mut cliques);

    // We only want the biggest clique (assuming one exists)
    cliques.sort_by_key(|clique| clique.len());
    let max_clique = cliques.last().expect("Empty?");
    println!("{}", max_clique.join(","));
}
