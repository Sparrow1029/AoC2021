use std::env::var;
use std::fmt;
use std::fs;
use std::{thread, time};

const DEBUG: bool = false;
static ADJACENT: [(isize, isize); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

/// Allows for printing animated grid state inside each cycle
fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}

#[derive(Debug, Default, Clone, Copy)]
struct Node {
    x: usize,
    y: usize,
    val: u8,
}

impl Node {
    /// Return all neighboring (x, y) coordinates relative to this Nodes x & y position.
    fn get_surrounding_coords(&self, w_bound: usize, h_bound: usize) -> Vec<(usize, usize)> {
        let mut surrounding = vec![];
        for (dx, dy) in ADJACENT {
            let y1 = self.y as isize + dy;
            let x1 = self.x as isize + dx;
            if y1 >= 0 && x1 >= 0 && y1 < h_bound as isize && x1 < w_bound as isize {
                surrounding.push((x1 as usize, y1 as usize))
            }
        }
        surrounding
    }

    /// "Flash" this node and return whether or not a flash occured
    fn flash(&mut self) -> bool {
        if self.val > 9 {
            self.val = 0;
            return true;
        }
        false
    }

    /// Mutate internal state by incrementing value += 1
    fn charge(&mut self) {
        self.val += 1;
    }
}

/// Rust lifetimes, Rc, RefCell<T>, and Vec<Rc<RefCell<T>>> are difficult to
/// wrap one's head around. So, this implementation is a Grid which **OWNS** all `Node` struct
/// objects in its 2D Vector, and references those nodes by index[y][x] position.
///
/// When the state of a `Node` needs to be mutated, it's retrieved via `&mut self.nodes[y][x]`.
struct Grid {
    w: usize,
    h: usize,
    nodes: Vec<Vec<Node>>,
}

/// Allow cloning so Part 1 & Part 2 can have fresh state.
impl Clone for Grid {
    fn clone(&self) -> Self {
        let mut new_nodes = vec![];
        for row in self.nodes.iter() {
            let mut new_row = vec![];
            for node in row {
                new_row.push(node.clone());
            }
            new_nodes.push(new_row);
        }
        Grid {
            w: self.w,
            h: self.h,
            nodes: new_nodes,
        }
    }
}

/// Create a grid of `Node` struct objects from input string.
impl From<String> for Grid {
    fn from(string: String) -> Self {
        let mut y = 0;
        let mut x = 0;
        let mut nodes = vec![];
        for line in string.lines() {
            let mut row = vec![];
            for c in line.chars() {
                let val = c.to_digit(10).unwrap() as u8;
                row.push(Node { x, y, val });
                x += 1;
            }
            nodes.push(row);
            y += 1;
            x = 0;
        }

        Grid {
            w: nodes[0].len(),
            h: nodes.len(),
            nodes,
        }
    }
}

/// Display the grid with all "flashed" (0) nodes as bold red.
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();
        for y in 0..self.h {
            for x in 0..self.w {
                let val = self.nodes[y][x].val;
                let bold = if val == 0 || val > 9 { "\x1b[0;91m\x1b[1m" } else { "" };
                if val <= 9 {
                    string.push_str(format!("{}{}\x1b[0m", bold, val).as_str());
                } else {
                    string.push_str(format!("{}#\x1b[0m", bold).as_str());
                }
            }
            string.push_str("\n");
        }
        write!(f, "{}\n", string)
    }
}

impl Grid {
    /// For number of `cycles` (steps), charge & flash Octopuses, counting total number of flashes.
    /// Short-circuit in the case that all octopuses flash in one cycle (synchronized), returning which
    /// step number that occured.
    fn count_flashes(&mut self, cycles: usize) -> usize {
        // DEBUG flag allows pretty-printing of the grid per-step -- fun to watch
        let milliseconds = time::Duration::from_millis(30);
        let seconds = time::Duration::from_millis(800);

        let mut total_flashes = 0;
        for i in 0..cycles {
            let mut flashes_this_cycle = 0;
            let mut to_flash = vec![];

            // first step: Increment value of all nodes by one
            for node in self.nodes.iter_mut().flatten() {
                node.charge();
                // if any node's value is > 9, it's gonna POP
                if node.val > 9 {
                    to_flash.push((node.x, node.y));
                }
            }

            // Pop a node (octopus) that is ready to flash out of the queue
            while let Some((x, y)) = to_flash.pop() {
                let cur_node = &mut self.nodes[y][x];
                if cur_node.val == 0 {
                    // already flashed
                    continue ;
                }
                if cur_node.flash() {
                    // Increment two counters:
                    // `flashes_this_cycle` is to determine whether or not all have flashed in sync (all zeroes)
                    flashes_this_cycle += 1;
                    total_flashes += 1;

                    // Increment value (charge) of each neighboring node, adding it into flash queue if it's value is > 9
                    for (x, y) in cur_node.get_surrounding_coords(self.w, self.h) {
                        let nxt_node = &mut self.nodes[y][x];
                        if nxt_node.val != 0 {
                            nxt_node.charge();
                            if nxt_node.val > 9 {
                                to_flash.push((x, y));
                            }
                        }
                    }
                }
                // Print out grid as you go
                if DEBUG {
                    println!("{}", self);
                    clear_screen();
                    thread::sleep(milliseconds);
                }
            }
            // Print out grid at end of step
            if DEBUG {
                println!("{}\nSTEP {} COMPLETE", self, i+1);
                clear_screen();
                thread::sleep(seconds);
            }
            if flashes_this_cycle == self.w * self.h {
                println!("SYNCHRONIZATION COMPLETE:\n{}STEP: {}", self, i+1);
                break;
            }

        }
        total_flashes
    }
}

fn main() {
    #[rustfmt::skip]
    let path = format!("{}/day11/src/input.txt", var("AOC_DIR").unwrap_or_else(|e| panic!("error: {} - {}", e, "AOC_DIR")));
    // let path = format!("{}/day11/src/input_example.txt", var("AOC_DIR").unwrap_or_else(|e| panic!("error: {} - {}", e, "AOC_DIR")));

    let grid = Grid::from(fs::read_to_string(path).expect("failed to read file"));
    println!("INITIAL GRID \n{}", grid);
    let seconds = time::Duration::from_millis(1500);
    thread::sleep(seconds);
    clear_screen();

    let mut part1 = grid.clone();
    let mut total_flashes = part1.count_flashes(100);
    println!("Part 1:\n\nFINAL GRID\n{}total flashes: {}\n", part1, total_flashes);

    let mut part2 = grid.clone();
    println!("Part 2:");
    total_flashes = part2.count_flashes(1000);
    println!("total flashes: {}", total_flashes);
}
