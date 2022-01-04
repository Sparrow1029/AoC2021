use std::collections::HashMap;
use std::env::var;
use std::fs::read_to_string;
use std::{thread, time};
use priority_queue::DoublePriorityQueue;

const SLEEP: time::Duration = time::Duration::from_millis(100);

type Coord = (isize, isize);
type PathMap = HashMap<Coord, Coord>;
type CostMap = HashMap<Coord, f64>;

///// Allows for printing animated grid state inside each cycle
fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}

/// Thank you, [Redblob Games](https://www.redblobgames.com/pathfinding/a-star/implementation.html) for the excellent implementation
/// explanations for Dijkstra & A* search.
///
/// This implementation is very much like the Python one shown on Redblob games. Uses `priority_queue::DoublePriorityQueue` for popping the
/// minimum-weighted value on each step. Because of how the values are distributed in this particular graph, _most_ of the nodes
/// (generally, _ALL_ of them for these puzzle inputs...) are visited in the quest to find the least-cost path through to the
/// bottom-right corner of the grid.
fn a_star_search(graph: &WeightedGraph, start: Coord, goal: Coord, animate: bool) -> (PathMap, CostMap) {
    let mut frontier = DoublePriorityQueue::new();
    frontier.push(start, 0);
    let mut came_from: PathMap = HashMap::new();
    let mut cost_so_far: CostMap = HashMap::new();
    came_from.insert(start, (-1, -1));
    cost_so_far.insert(start, 0.0);

    while !frontier.is_empty() {
        let current = frontier.pop_min().unwrap().0;
        if animate {
            let path_so_far = reconstruct_path(&came_from, start, current);
            graph.display_with_path(&path_so_far, &cost_so_far);
            thread::sleep(SLEEP);
        }

        if current == goal {
            break;
        }

        // Visit all E W N S neighbors of the grid
        // get the total cost so far to visit this node
        // if the neighbor's total cost is not in the CostMap
        // OR it has been visited before, and its new edge cost is less than its previous value
        // - insert it into the CostMap
        // - tag it with a priority of its total_cost + cost returned by the heuristic
        // - put it into the `priority_queue::DoublePriorityQueue`
        // - add the new k, v pair to the PathMap
        for next in graph.neighbors(current) {
            let new_cost = cost_so_far.get(&current).unwrap() + graph.cost(current, next);
            if !cost_so_far.contains_key(&next) || new_cost < *cost_so_far.get(&next).unwrap() {
                cost_so_far.insert(next, new_cost);
                let priority = new_cost + heuristic(next, goal);
                frontier.push(next, priority as isize);
                came_from.insert(next, current);
            }
        }
        if animate {
            clear_screen();
        }
    }
    (came_from, cost_so_far)
}

/// Reconstruct the lowest-cost path found by the A* search.
///
/// Iterates through the `HashMap` of (x, y) -> (x, y) pairs (goal -> start)
/// then reverses the path so it is returned (start -> goal)
fn reconstruct_path(came_from: &PathMap, start: Coord, goal: Coord) -> Vec<Coord> {
    let mut current = goal;
    let mut path = vec![];
    while current != start {
        path.push(current);
        current = *came_from.get(&current).unwrap();
   }
    path.push(start);
    path = path.iter().rev().map(|v| *v).collect();
    path
}

/// The heuristic for the 2-D `WeightedGrid` (N,E,S,W) is the manhattan distance from the
/// current (x, y) coordinate to the goal (x, y). Shorter is better (min value)
fn heuristic(a: Coord, b: Coord) -> f64 {
    let (x1, y1) = a;
    let (x2, y2) = b;
    (x1 - x2).abs() as f64 + (y1 - y2).abs() as f64
}

/// Graph struct with width, height, and a map of (x, y) coordinates -> weight value
#[derive(Debug)]
struct WeightedGraph {
    width: isize,
    height: isize,
    weights: HashMap<Coord, f64>,
}

impl WeightedGraph {
    /// convenience function to return a value from `self.weights`
    fn get(&self, id: Coord) -> Option<f64> {
        match self.weights.get(&id) {
            Some(result) => Some(*result),
            None => None
        }
    }

    /// Get the (x, y) coordinate representing the bottom-right corner of the grid
    fn bottom_right(&self) -> Coord {
        (self.width-1, self.height-1)
    }

    /// Added the twist to the cost (though mostly unecessary for AoC2021 graph input) from
    /// Redblob games' implementation [here](https://www.redblobgames.com/pathfinding/a-star/implementation.html#troubleshooting-ugly-path)
    /// helps make paths "pretty"; even though the cost is the lowest, it may not be the visually/logically "straightest" path to a human.
    fn cost(&self, from_node: Coord, to_node: Coord) -> f64 {
        let prev_cost = *self.weights.get(&to_node).unwrap_or(&1.0);
        let mut nudge = 0.0;
        let (x1, y1) = from_node;
        let (x2, y2) = to_node;
        if ((x1 + y1) % 2 == 0) && x2 != x1 {
            nudge = 1.0;
        } else if ((x1 + y1) % 2 == 1) && y2 != y1 {
            nudge = 1.0;
        }
        prev_cost + 0.001 * nudge
    }

    /// Determine if a coordinate (x, y) is in the bounds of the Grid. Used by `neighbors` function.
    fn in_bounds(&self, id: Coord) -> bool {
        let (x, y) = id;
        0 <= x && x < self.width as isize && 0 <= y && y < self.height as isize
    }

    /// Return all E, W, N, S neighbors for a given coordinate (x, y) point.
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

    /// Print a visual representation of the grid with weights.
    fn display(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get((x, y)).unwrap());
            }
            println!();
        }
    }

    /// Similar to `display`, except with terminal color codes:
    /// - Cyan      - A visited node in the least-cost path
    /// - Dark Grey - A visited node NOT in the least-cost path
    /// - White     - just a regular old node
    fn display_with_path(&self, path: &Vec<Coord>, costs: &CostMap) {
        let visited_nodes = costs.iter().map(|t| *t.0).collect::<Vec<Coord>>();
        for y in 0..self.height {
            for x in 0..self.width {
                let color = if path.contains(&(x, y)) {
                    "\x1b[1;36m"
                } else if visited_nodes.contains(&(x, y)) {
                    "\x1b[1;30m"
                } else {
                    ""
                };
                print!("{}{}\x1b[0m", color, self.get((x, y)).unwrap());
            }
            println!();
        }
    }

    /// Expand grid as tiles by factor of `factor`.
    /// For each expanded tile, weights increase by one unless they are > 9.0, which resets weight to 1.0.
    ///
    /// Grid tile
    /// ```
    /// 1 4 8
    /// 1 1 1
    /// 3 9 1
    /// ```
    /// `expand`ed by a factor of 2 Becomes
    /// ```
    /// 1 4 8 | 2 5 9
    /// 1 1 1 | 2 2 2
    /// 3 9 1 | 4 1 2
    /// -------------
    /// 2 5 9 | 3 6 1
    /// 2 2 2 | 3 3 3
    /// 4 1 2 | 5 2 3
    /// ```
    fn expand(&mut self, factor: isize) {
        // Expand horizontally first
        for y in 0..self.height {
            for f in 0..=factor {
                for x in 0..self.width {
                    let new_x = (f+1) * self.width + x;
                    let old_x = f * self.width + x;
                    let cur_val = self.get((old_x, y)).unwrap();
                    let new_val = cur_val + 1.0;

                    if new_val > 9.0 {
                        self.weights.insert((new_x, y), 1.0);
                    } else {
                        self.weights.insert((new_x, y), new_val);
                    }
                }
            }
        }
        self.width *= factor;

        // Expand vertically
        for f in 0..=factor {
            for y in 0..self.height {
                for x in 0..self.width {
                    let new_y = (f+1) * self.height + y;
                    let old_y = f * self.height + y;
                    let cur_val = self.get((x, old_y)).unwrap();
                    let new_val = cur_val + 1.0;

                    if new_val > 9.0 {
                        self.weights.insert((x, new_y), 1.0);
                    } else {
                        self.weights.insert((x, new_y), new_val);
                    }
                }
            }
        }
        self.height *= factor;
    }
}

/// Get the coordinate for the lower right-hand corner of the grid
fn get_max_xy(pts: &Vec<Coord>) -> Coord {
    let max_x = pts.iter().map(|pt| pt.0).max().unwrap();
    let max_y = pts.iter().map(|pt| pt.1).max().unwrap();
    (max_x, max_y)
}

/// Parse input file into a `WeightedGraph` struct
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
        width: width + 1, height: height + 1, weights: HashMap::from_iter(pt_vec)
    }
}

/// Unused function to determine cost of all nodes visited, but kept the code around...
// fn get_total_cost(path: &Vec<Coord>, costs: &CostMap) -> f64 {
//     let mut sum = 0.0;
//     path.iter().for_each(|pt| sum += costs.get(pt).unwrap());
//     sum
// }

fn main() {
    let mut graph = parse_input("input_example.txt");

    // Part 1
    let goal = graph.bottom_right();
    let (came_from, costs) = a_star_search(&graph, (0, 0), goal, true);
    let reconstructed = reconstruct_path(&came_from, (0, 0), goal);
    graph.display_with_path(&reconstructed, &costs);
    let final_cost = costs.get(&graph.bottom_right()).unwrap().round();
    println!("Final cost: {:?}", final_cost);

    // Part 2
    graph.expand(3);
    let goal = graph.bottom_right();
    let (_, costs) = a_star_search(&graph, (0, 0), goal, false);
    // YIKES... The `.display_with_path` function was toooo heavy for a 500 x 500 grid...
    // would need to implement a more efficient lookup for visited nodes, etc
    // let reconstructed = reconstruct_path(&came_from, (0, 0), goal);
    // graph.display_with_path(&reconstructed, &costs);
    let final_cost = costs.get(&graph.bottom_right()).unwrap().round();
    println!("Final cost {:?}", final_cost);
}
