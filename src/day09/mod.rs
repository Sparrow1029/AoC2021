use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

// Example data
// const ARR_COLS: usize = 10;
// const ARR_ROWS: usize = 5;
// Puzzle data
const ARR_COLS: usize = 100;
const ARR_ROWS: usize = 100;
const ADJACENT: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
type LavaGrid = [[u8; ARR_COLS]; ARR_ROWS];

fn parse_input(path: &str) -> Result<LavaGrid, Error> {
    let mut lava_grid = [[0; ARR_COLS]; ARR_ROWS];
    let mut f = BufReader::new(
        File::open(path).unwrap_or_else(|e| panic!("file open failed: {} - {}", e, path)),
    );

    let mut row = 0;
    let mut buf = String::new();
    loop {
        match f.read_line(&mut buf) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }
                let digits: Vec<u8> = buf
                    .trim()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                    .collect();
                for i in 0..ARR_COLS {
                    lava_grid[row][i] = digits[i];
                }
                row += 1;
                buf.clear();
            }
            Err(err) => return Err(err),
        };
    }
    Ok(lava_grid)
}

fn get_adjacent(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut coords = vec![];
    for pt in ADJACENT {
        let (new_x, new_y) = (x as i32 + pt.0, y as i32 + pt.1);
        if new_x < 0 || new_y < 0 || new_x >= ARR_COLS as i32 || new_y >= ARR_ROWS as i32 {
            // edge or corner
            continue;
        }
        coords.push((new_x as usize, new_y as usize));
    }

    coords
}

fn find_low_points(lava_grid: &LavaGrid) -> Vec<u32> {
    let mut low_pts = vec![];
    for y in 0..ARR_ROWS {
        for x in 0..ARR_COLS {
            let cur_val = lava_grid[y][x];
            let adjacent = get_adjacent(x, y);
            let mut is_low_point: bool = true;
            for (pt_x, pt_y) in adjacent {
                if lava_grid[pt_y][pt_x] <= cur_val {
                    is_low_point = false;
                    break;
                }
            }
            if is_low_point {
                low_pts.push(cur_val as u32);
            }
        }
    }
    low_pts
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Node {
    x: usize,
    y: usize,
    val: u8,
    filled: bool,
}

impl Node {
    fn new(x: usize, y: usize, val: u8) -> Self {
        Node {
            x,
            y,
            val,
            filled: false,
        }
    }

    fn fill(&mut self) {
        self.filled = true;
    }

    fn is_boundary(&self) -> bool {
        self.val == 9
    }

    fn get_adjacent(&self) -> Vec<(usize, usize)> {
        get_adjacent(self.x, self.y)
    }
}

#[derive(Debug)]
struct Grid {
    grid: [[Node; ARR_COLS]; ARR_ROWS],
    basins: HashSet<Vec<u8>>,
}

impl Grid {
    fn from_lava_grid(arr: &LavaGrid) -> Self {
        let mut grid = [[Node::new(0, 0, 0); ARR_COLS]; ARR_ROWS];
        for y in 0..ARR_ROWS {
            for x in 0..ARR_COLS {
                grid[y][x] = Node::new(x, y, arr[y][x]);
            }
        }
        Grid {
            grid,
            basins: HashSet::new(),
        }
    }

    fn fill_basin(&mut self, x: usize, y: usize, basin: &mut Vec<u8>) {
        let cur_node = self.get_node_at_xy(x, y);
        if !cur_node.filled && !cur_node.is_boundary() {
            basin.push(cur_node.val);
            cur_node.fill();
            for (pt_x, pt_y) in cur_node.get_adjacent() {
                self.fill_basin(pt_x, pt_y, basin)
            }
        } else {
            return ();
        }
    }

    fn get_node_at_xy(&mut self, x: usize, y: usize) -> &mut Node {
        &mut self.grid[y][x]
    }

    fn find_basins(&mut self) {
        let mut visited: HashSet<Node> = HashSet::new();
        for y in 0..ARR_ROWS {
            for x in 0..ARR_COLS {
                let node = self.get_node_at_xy(x, y);
                if visited.contains(node) || node.is_boundary() {
                    continue;
                }
                visited.insert(self.get_node_at_xy(x, y).clone());
                let mut basin = vec![];
                self.fill_basin(x, y, &mut basin);
                // not sure why ending up with some empty basin vectors...
                if !basin.is_empty() {
                    self.basins.insert(basin.to_owned());
                }
            }
        }
    }
}

pub fn run(example: bool) {
    let path = if example == true {
        "inputs/day09_example.txt"
    } else {
        "inputs/day09.txt"
    };
    let lava_grid = parse_input(path).expect("Error reading puzzle input");

    // Part 1
    let low_pts = find_low_points(&lava_grid);
    let sum_low_pts: u32 = low_pts.iter().map(|v| v + 1).sum();
    println!(
        "Part 1 - weighted (+1) sum of low points:   {}",
        sum_low_pts
    );

    // Part 2
    let mut grid = Grid::from_lava_grid(&lava_grid);
    grid.find_basins();

    let mut product = 1;
    for basin in grid
        .basins
        .into_iter()
        .sorted_by_key(|f| f.len())
        .rev()
        .take(3)
    {
        product *= basin.len();
    }
    println!("Part 2 - product of 3 largest basin values: {}", product);
}
