use std::fs::read_to_string;

const AOC_DIR: &str = "/Users/p2910482/Projects/rust/AoC2021";

/// Parse day07 input file (single line of comma-separated integers)
fn parse_input(file_name: &str) -> Vec<i32> {
    let line = read_to_string(file_name).expect("file not found");
    line.trim().split(",").map(|v| v.parse().unwrap()).collect()
}

fn get_median(vec: &mut Vec<i32>) -> i32 {
    if vec.is_empty() {
        return 0;
    }
    vec.sort();

    let index = vec.len() / 2;

    if vec.len() % 2 == 1 {
        vec[index]
    } else {
        (vec[index - 1] + vec[index]) / 2
    }
}

fn get_mean(vec: &mut Vec<i32>) -> i32 {
    if vec.is_empty() {
        return 0;
    }
    vec.sort();
    let float_mean: f32 = vec.iter().sum::<i32>() as f32 / vec.len() as f32;
    // println!("{} rounds to -> {}", float_mean, float_mean.round());
    float_mean.round() as i32
}

/// Calculate total distance (fuel consumed) for all crabs to a given position
/// using Gaussian summation method
fn gaussian_distance(pos: i32, crabs: &Vec<i32>) -> i32 {
    crabs
        .iter()
        .map(|c| (c - pos).abs())
        .map(|n| (n * (n + 1)) / 2)
        .sum::<i32>()
}

/// Solution adapted from [here](https://www.ericburden.work/blog/2021/12/07/advent-of-code-2021-day-7/)
/// which was written in Julia (cool syntax)
/// Their solution used the gaussian method, which is a slick way to solve this problem. Good 'ol Gauss...
///
/// Find average position as starting point, then calculate the total sum of gaussian
/// distances for each crab to that position.
/// Then we move the position left or right depending for which direction the total fuel/gaussian sum is lower
/// until we see the total fuel start to increase again.
fn part2(crabs: &mut Vec<i32>) -> i32 {
    let pos = get_mean(crabs);
    let mut min_fuel = gaussian_distance(pos, &crabs);

    // Find which direction we need to move from the mean to get to a lower fuel cost (if there is one)
    let (leftdist, rightdist) = (
        gaussian_distance(pos - 1, &crabs),
        gaussian_distance(pos + 1, &crabs),
    );
    let (mut pos, mut nextdist, step) = match leftdist < rightdist {
        true => (pos - 1, leftdist, -1),
        false => (pos + 1, rightdist, 1),
    };
    loop {
        // println!(
        //     "position: {}, current min fuel: {}, fuel to next position: {}",
        //     pos, min_fuel, nextdist
        // );
        if nextdist < min_fuel {
            min_fuel = nextdist;
            pos += step;
            nextdist = gaussian_distance(pos, crabs)
        } else {
            break;
        }
    }
    min_fuel
}

fn main() {
    let input_path = format!("{}/day07/src/input.txt", AOC_DIR);
    // let input_path = format!("{}/day07/src/input_example.txt", AOC_DIR);
    let mut positions = parse_input(input_path.as_str());

    // Part 1
    // Find median position and calculate sum of all crabs movement to that position
    let median = get_median(&mut positions);
    let fuel_amt: i32 = positions.iter().map(|i| (i - median).abs()).sum();
    println!("Part 1 - fuel amount: {}", fuel_amt);

    // Part 2
    let min_fuel = part2(&mut positions);
    println!("Part 2 - fuel amount: {}", min_fuel);
}
