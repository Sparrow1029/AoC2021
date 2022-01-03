use std::collections::HashMap;
use std::env::var;
use std::fs::read_to_string;
use priority_queue::DoublePriorityQueue;

type Coord = (isize, isize);
type PathMap = HashMap<Coord, Coord>;
type CostMap = HashMap<Coord, f64>;

fn a_star_search(graph: &WeightedGraph, start: Coord, goal: Coord) -> (PathMap, CostMap) {
    let mut frontier = DoublePriorityQueue::new();
    frontier.push(start, 0);
    let mut came_from: PathMap = HashMap::new();
    let mut cost_so_far: CostMap = HashMap::new();
    came_from.insert(start, (-1, -1));
    cost_so_far.insert(start, 0.0);

    while !frontier.is_empty() {
        let current = frontier.pop_min().unwrap().0;
        // println!("Current: {:?}", current);

        if current == goal {
            // println!("MADE IT");
            break;
        }

        for next in graph.neighbors(current) {
            // println!("Next: {:?}, cost: {}", next, graph.cost(current, next));
            // println!("cost_so_far[current] = {:?}", cost_so_far.get(&current));
            let new_cost = cost_so_far.get(&current).unwrap() + graph.cost(current, next);
            // println!("New Cost: {}", new_cost);
            if !cost_so_far.contains_key(&next) || new_cost < *cost_so_far.get(&next).unwrap() {
                cost_so_far.insert(next, new_cost);
                let priority = new_cost + heuristic(next, goal);
                frontier.push(next, priority as isize);
                came_from.insert(next, current);
            }
        }
    }
    (came_from, cost_so_far)
}

fn reconstruct_path(came_from: &PathMap, start: Coord, goal: Coord) -> Vec<Coord> {
    let mut current = goal;
    let mut path = vec![];
    while current != start {
        println!("Current in path: {:?}", current);
        path.push(current);
        current = *came_from.get(&current).unwrap();
   }
    path.push(start);
    path = path.iter().rev().map(|v| *v).collect();
    path
}

fn heuristic(a: Coord, b: Coord) -> f64 {
    let (x1, y1) = a;
    let (x2, y2) = b;
    (x1 - x2).abs() as f64 + (y1 - y2).abs() as f64
}

#[derive(Debug)]
struct WeightedGraph {
    width: isize,
    height: isize,
    weights: HashMap<Coord, f64>,
}

impl WeightedGraph {
    fn cost(&self, _: Coord, to_node: Coord) -> f64 {
        *self.weights.get(&to_node).unwrap_or(&1.0)
    }

    fn in_bounds(&self, id: Coord) -> bool {
        let (x, y) = id;
        0 <= x && x <= self.width as isize && 0 <= y && y <= self.height as isize
    }

    fn neighbors(&self, id: Coord) -> Vec<Coord> {
        let (x, y) = id;
        let mut neighbors: Vec<Coord> = Vec::from([(x + 1, y), (x -1, y), (x, y - 1), (x, y + 1)]);
        if (x + y) % 2 == 0 {
            for (i, val) in neighbors.clone().iter().enumerate() {
                neighbors[i] = *val;
            }
        }
        neighbors.retain(|pt| self.in_bounds(*pt));
        neighbors
    }

    fn display(&self) {
        for y in 0..=self.height {
            for x in 0..=self.width {
                print!("{}", self.weights.get(&(x, y)).unwrap());
            }
            println!();
        }
    }

    fn display_with_path(&self, path: Vec<Coord>) {
        for y in 0..=self.height {
            for x in 0..=self.width {
                let weight = self.weights.get(&(x, y)).unwrap();
                // let color = match path.contains
                //     v
                // }
                print!("{}", self.weights.get(&(x, y)).unwrap());
            }
            println!();
        }

    }
}

/// Get the coordinate for the lower right-hand corner of the grid
fn get_max_xy(pts: &Vec<Coord>) -> Coord {
    let max_x = pts.iter().map(|pt| pt.0).max().unwrap();
    let max_y = pts.iter().map(|pt| pt.1).max().unwrap();
    (max_x, max_y)
}

fn parse_input(path: &str) -> WeightedGraph {
    // let mut graph = HashMap::new();
    let mut pt_vec: Vec<((isize, isize), f64)> = vec![];
    let input_path = format!("{}/day15/src/{}", var("AOC_DIR").unwrap_or_else(|e| panic!("{}", e)), path);
    let file = read_to_string(input_path.as_str()).unwrap_or_else(|e| panic!("error opening file: {}", e));
    let mut y = 0;
    let mut x = 0;
    for line in file.lines() {
        line.chars().for_each(|c| {
            pt_vec.push(((x, y), c.to_digit(10).unwrap() as f64));
            x += 1
        });
        y += 1;
        x = 0;
    }
    let (width, height) = get_max_xy(&pt_vec.iter().map(|pt| pt.0).collect::<Vec<Coord>>());
    WeightedGraph {
        width, height, weights: HashMap::from_iter(pt_vec)
    }
}

fn get_total_cost(path: &Vec<Coord>, costs: &CostMap) -> f64 {
    let mut sum = 0.0;
    path.iter().for_each(|pt| sum += costs.get(pt).unwrap());
    sum
}

fn main() {
    let graph = parse_input("input.txt");
    graph.display();
    let goal = (graph.width, graph.height);
    // println!("GOAL: {:?}", goal);
    let (came_from, costs) = a_star_search(&graph, (0, 0), goal);
    let reconstructed = reconstruct_path(&came_from, (0, 0), goal);
    println!("Total cost: {}", get_total_cost(&reconstructed, &costs));
    println!("Path reconstructed: {:?}", reconstruct_path(&came_from, (0,0), goal));
    println!("last cost? {:?}", costs.get(&(graph.width, graph.height)).unwrap());
}
