use std::{collections::HashSet, fs};
use std::env::var;

// So easy to define type aliases in rust!
type Point = (isize, isize);

/// Take the point defining a fold line & the number of currently visible points
/// and return the points visible after executing the fold
fn fold(fold_pt: Point, points: &Vec<Point>) -> Vec<Point> {
    let mut keep: HashSet<Point> = HashSet::new();
    let mut to_fold: Vec<Point> = Vec::new();
    // Keep any points on the "upper" side of the paper for a fold along the x-axis.
    // Conversely keep any points on the "left" side of the paper for a fold along the y-axis.
    points.iter().for_each(|pt| match fold_pt.0 {
        0 => {
            if pt.1 < fold_pt.1 {
                keep.insert(*pt);
            } else {
                to_fold.push(*pt);
            }
        }
        _ => {
            if pt.0 < fold_pt.0 {
                keep.insert(*pt);
            } else {
                to_fold.push(*pt);
            }
        }
    });
    for change_pt in to_fold {
        keep.insert(get_new_point(change_pt, fold_pt));
    }
    keep.into_iter().collect::<Vec<Point>>()
}

/// To get the new x/y coords of a point after a fold:
/// get the absolute value of the difference between x1, x2*2 || y1, y2*2.
///
/// For example:
/// If the point is (2, 8) and the fold line is (0, 7), then the result is
/// ```
/// -> (|(x1 - x2*2)|, |(y1 - y2*2)|)
/// -> (|(2 - 0*2)|, |(8 - 7*2)|)
/// -> (|2 - 0|, |8 - 14|)
/// -> (|2|, |-6|)
/// -> (2, 6)
/// ```
fn get_new_point(pt: Point, fold_pt: Point) -> Point {
    ((pt.0 - fold_pt.0 * 2).abs(), (pt.1 - fold_pt.1 * 2).abs())
}

/// Get the coordinate for the lower right-hand corner of the grid
fn get_max_xy(pts: &Vec<Point>) -> Point {
    let max_x = pts.iter().map(|pt| pt.0).max().unwrap();
    let max_y = pts.iter().map(|pt| pt.1).max().unwrap();
    (max_x, max_y)
}

/// Read puzzle input into a vector of two  vectors of coordinate points:
/// visible dots, and fold lines
fn parse_input(path: &str) -> (Vec<Point>, Vec<Point>) {
    let mut points: Vec<Point> = vec![];
    let mut folds = vec![];
    let instructions = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("couldn't open file: {}", e))
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    for line in instructions[0].lines() {
        let split = line.split(',').map(|i| isize::from_str_radix(i, 10).unwrap()).collect::<Vec<isize>>();
        points.push((split[0], split[1]));
    }
    for line in instructions[1].lines() {
        // a fold line is represented by a single x, y coordinate
        // y=7 -> (0, 7), x=5 -> (5, 0)
        let split: Vec<&str> = line.split(' ').collect();
        let fold: Vec<&str> = split[2].split('=').collect();
        match fold[0] {
            "x" => {
                folds.push((isize::from_str_radix(fold[1], 10).unwrap(), 0))
            },
            "y" => {
                folds.push((0, isize::from_str_radix(fold[1], 10).unwrap()))
            },
            _ => { panic!("WUT") }
        }
    }
    (points, folds)
}

/// Display a grid using '.' for empty spaces and '#' for visible points
/// from an input vector of (x, y) coordinates.
fn print_pt_grid(pts: Vec<Point>) {
    let max_xy = get_max_xy(&pts);
    for y in 0..=max_xy.1 {
        for x in 0..=max_xy.0 {
            if pts.contains(&(x, y)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }
}

fn main() {
    let path = format!("{}/day13/src/input.txt", var("AOC_DIR").unwrap_or_else(|_| "no path".to_string()));
    let (mut points, folds) = parse_input(path.as_str());

    // Part 1
    points = fold(*folds.first().unwrap(), &points);
    println!("Part 1 - number of visible points after single fold: {}\n", points.len());

    // Part 2
    for f in folds {
        points = fold(f, &points);
    }
    println!("Part 2 - result of folds:");
    print_pt_grid(points);
}
