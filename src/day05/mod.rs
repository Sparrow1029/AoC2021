use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

/// Create line segments from Day 05 puzzle input.
fn read_input<R: Read>(io: R) -> Result<Vec<LineSegment>, Error> {
    let br = BufReader::new(io);
    let mut line_segments: Vec<LineSegment> = vec![];
    for line in br.lines() {
        let inner = line.unwrap();
        let parts = inner.split(" -> ").into_iter().collect::<Vec<&str>>();
        line_segments.push(LineSegment::from_points(
            Point::from_str(&parts[0]),
            Point::from_str(&parts[1]),
        ))
    }

    Ok(line_segments)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    /// Return an instance of `Point` from string formatted like `"9,2"`
    fn from_str(string: &str) -> Self {
        let coords = string
            .split(',')
            .collect::<Vec<&str>>()
            .iter()
            .map(|v| v.parse().expect("invalid"))
            .collect::<Vec<i32>>();
        Point {
            x: coords[0],
            y: coords[1],
        }
    }

    /// Return a tuple (i32, i32) representing this points coordinates.
    fn as_tup(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

#[derive(Debug)]
struct LineSegment {
    p1: Point,
    p2: Point,
}

impl LineSegment {
    /// Create line from two `Point` structs, sorting the points left->right by x value
    fn from_points(pt1: Point, pt2: Point) -> Self {
        let mut pts = vec![pt1, pt2];
        pts.sort_by_key(|p| p.x);
        LineSegment {
            p1: pts[0],
            p2: pts[1],
        }
    }

    /// Get line slope using classic `(y1 - y2) / (x1 - x2)` formula.
    ///
    /// Returns `None` if slope is undefined because line segment is vertical (x1 == x2).
    fn slope(&self) -> Option<i32> {
        if self.p1.x - self.p2.x == 0 {
            // avoid division by 0
            return None;
        }
        Some((self.p1.y - self.p2.y) / (self.p1.x - self.p2.x))
    }

    /// Return y-intercept for a `LineSegment` (if line is not vertical).
    ///
    /// Solves for `y = mx + b` as `b = y - mx`.
    fn y_intercept(&self) -> Option<i32> {
        match self.slope() {
            Some(m) => {
                if m == 0 {
                    Some(self.p1.y)
                } else {
                    Some(self.p1.y - (m * self.p1.x))
                }
            }
            None => None,
        }
    }

    /// Return a Vector of `Point` objects representing integer coordinates along a line segment.
    ///
    /// Uses `y = mx + b` formula where applicable.
    fn gen_coords(&self) -> Vec<Point> {
        let mut coords: Vec<Point> = vec![];

        let m = match self.slope() {
            Some(m) => m,
            None => {
                let (y1, y2) = (min(self.p1.y, self.p2.y), max(self.p1.y, self.p2.y));
                for y in y1..=y2 {
                    coords.push(Point::new(self.p1.x, y));
                }
                return coords;
            }
        };

        // Though `.y_intercept()` method returns `Option<i32>`, we've handled case of
        // undefined slope above in binding `m` (slope) on match statement
        let b = self.y_intercept().unwrap();
        for x in self.p1.x..=self.p2.x {
            coords.push(Point::new(x, m * x + b));
        }
        coords
    }
}

pub fn run(part: usize, input: Option<&String>) -> Result<(), Error> {
    let input_path = match input {
        Some(arg) => {
            if arg == "example" {
                "inputs/day05_example.txt"
            } else {
                println!(
                    "{:?} is not a valid arg (try 'example'). Using default input.",
                    arg
                );
                "inputs/day05.txt"
            }
        }
        _ => "inputs/day05.txt",
    };
    let line_segments = read_input(File::open(input_path)?)?;
    let mut part1_counter = HashMap::new();

    for line in &line_segments {
        // Part 1 only wants horizontal/vertical lines
        // Part 2 wants _all_ lines
        match line.slope() {
            Some(m) => {
                if m == 0 {
                    (); // note: () in control flow like this is essentially emulating a "noop"
                } else {
                    if part == 1 {
                        continue;
                    }
                    ();
                }
            }
            None => (),
        };
        for coord in line.gen_coords() {
            *part1_counter.entry(coord.as_tup()).or_insert(0) += 1;
        }
    }
    let more_than_1_intersection = part1_counter.iter().filter(|&(_, v)| *v > 1).count();
    println!(
        "Part {}\n  Number of points where at least 2 lines overlap: {}",
        part, more_than_1_intersection
    );

    Ok(())
}
