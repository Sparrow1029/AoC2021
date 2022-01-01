/// Copied from https://github.com/McSick/AdventOfCode2021/blob/main/12/tree-pathfind/src/main.rs
use std::collections::{HashMap, HashSet};
use std::env::var;
use std::fs;
use std::hash::Hash;

const MAX_VTEX: usize = 13;

/// Gonna use this for printing the path
fn swap_hashmap<T: Clone, U: Eq + Hash + Copy>(map: &HashMap<T, U>) -> HashMap<U, T> {
    let mut new_map = HashMap::new();
    for (k, v) in map.iter() {
        new_map.insert(*v, k.clone());
    }
    new_map
}

#[derive(Debug)]
pub struct Graph {
    vindex: usize,
    adj_matrix: [[i32; MAX_VTEX]; MAX_VTEX],
    vertex_map: HashMap<String, usize>,
    str_vx_map: HashMap<usize, String>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            vindex: 0,
            adj_matrix: [[0; MAX_VTEX]; MAX_VTEX],
            vertex_map: HashMap::new(),
            str_vx_map: HashMap::new(),
        }
    }

    fn print(&self) {
        println!("Adj matrix:");
        for row in self.adj_matrix {
            print!("|");
            for i in row {
                print!("{: >2} ", i);
            }
            print!("|\n");
        }
    }

    fn add_vertex(&mut self, vtex: &str) {
        if !self.vertex_map.contains_key(&vtex.to_string()) {
            self.vertex_map.insert(vtex.to_string(), self.vindex);
            self.vindex += 1;
        }
    }

    fn get_vertex(&self, vtex: &str) -> usize {
        match self.vertex_map.get(&vtex.to_string()) {
            Some(vindex) => *vindex,
            None => panic!("No vertex found for '{}'", vtex),
        }
    }

    // fn get_vertex_str(&self, vtex: usize) -> &str {
    //     self.str_vx_map.get(&vtex).unwrap().as_str()
    // }

    fn add_edge(&mut self, from: &str, to: &str) {
        let from_idx = self.get_vertex(from);
        let to_idx = self.get_vertex(to);
        let mut from_val = 1;
        let mut to_val = 1;
        // Identify edges to/from a "small cave" in the adjency matrix
        // by marking the edge with -1 instead of 1. "start" & "end" handled below
        if from.chars().all(|c| c.is_lowercase()) {
            from_val = -1;
        }
        if to.chars().all(|c| c.is_lowercase()) {
            to_val = -1;
        }
        if to == "start" || from == "end" {
            // Don't go back "to" start or "from" end
            // only one direction for edges adjacent to those vertices
            self.adj_matrix[to_idx][from_idx] = from_val;
        } else {
            // Otherwise edges are bi-directional
            self.adj_matrix[to_idx][from_idx] = from_val;
            self.adj_matrix[from_idx][to_idx] = to_val;
        }
    }

    fn get_edge(&self, from: usize, to: usize) -> i32 {
        self.adj_matrix[from][to]
    }

    /// To traverse the graph use DFS and an adjacency matrix recurse through the connected vertices
    /// keeping count of final paths.
    ///
    /// Use
    fn traverse(&mut self, from: usize, mut visited: HashSet<usize>, hit_twice: bool) -> i32 {
        visited.insert(from);
        let paths = self.adj_matrix[from];
        let mut count = 0;
        for to in 0..paths.len() {
            let edge = self.get_edge(from, to);

            if edge == 1 || edge == -1 {
                if self.get_vertex("start") == to {
                    continue;
                } else if self.get_vertex("end") == to {
                    // for v in &visited {
                    //     print!("{}-", self.get_vertex_str(*v));
                    // }
                    // println!("end");
                    count += 1;
                } else if edge > 0 || !visited.contains(&to) {
                    count += self.traverse(to, visited.clone(), hit_twice)
                } else if visited.contains(&to) && !hit_twice {
                    count += self.traverse(to, visited.clone(), true)
                }
            }
        }
        count
    }
}

impl From<String> for Graph {
    fn from(string: String) -> Self {
        let mut graph = Graph::new();
        for line in string.lines() {
            let split = line.split('-').collect::<Vec<&str>>();
            let from = split[0];
            let to = split[1];
            graph.add_vertex(from);
            graph.add_vertex(to);
            graph.add_edge(from, to);
        }
        graph.str_vx_map = swap_hashmap(&graph.vertex_map);
        graph
    }
}

fn main() {
    let path = format!(
        "{}/day12/src/input.txt",
        var("AOC_DIR").unwrap_or_else(|e| panic!("error: {} - {}", e, "AOC_DIR"))
    );
    // Part 1
    let mut graph = Graph::from(
        fs::read_to_string(path).unwrap_or_else(|e| panic!("error opening file: {}", e)),
    );
    // graph.print();
    let result = graph.traverse(graph.get_vertex("start"), HashSet::new(), true);
    println!("Part 1: {}", result);

    // Part 2
    let result = graph.traverse(graph.get_vertex("start"), HashSet::new(), false);
    println!("Part 2: {}", result);
}

#[test]
fn test_example() {
    let test_string = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end
"#;
    let mut graph = Graph::from(test_string.to_string());
    println!("Map: {:?}", graph);
    graph.print();
    let result = graph.traverse(graph.get_vertex("start"), HashSet::new(), true);
    assert!(result == 10);
}

#[test]
fn test_example_2() {
    let test_string = r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
"#;
    let graph = Graph::from(test_string.to_string());
    println!("Map: {:?}", graph);
    // graph.print();
    assert!(true);
}
