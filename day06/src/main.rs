use std::fs::read_to_string;
use std::collections::HashMap;

const AOC_DIR: &str = "/Users/p2910482/Projects/rust/AoC2021";
const STATE_ARR: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 0];

/// Create line segments from Day 05 puzzle input.
fn parse_input(file_name: &str) -> Vec<u8> {
    let line = read_to_string(file_name).expect("file not found");
    line.trim().split(",").collect::<Vec<&str>>().iter().map(|v| v.parse().unwrap()).collect()
}

/// Recursive function to calculate lanternfish growth.
///
/// stolen from https://github.com/MaxMaxoff/AdventOfCode2021/blob/master/Day6_Lanternfish/Day6_Lanternfish.py
/// and re-written in Rust. My initial implementation was naïve--just store all the values in a vector, which would have gotten **massive**.
///
/// This algorithm stores current state of lanternfish as a `HashMap` where
/// - key == the value of a lanternfish's counter
/// - value == the number of fish in the list with their counter at that value.
///
/// On each iteration, all the counter values are updated like so:
/// - if there are any `0` values, some number of fish have a counter that reached zero, and new fish are born with counter set to `8`.
/// - we then update the `6` value in the map with the new "reset" fish counters
/// - for all other counters, we move the previous value down to the key-1 to simulate all counters counting down by one day.
///
/// In this way, values are just moved around, instead of growing a huge Vector.
fn calculate_grow(state: &mut HashMap<u8, u64>, days: u32) -> HashMap<u8, u64> {
    let mut new_state: HashMap<u8, u64> = HashMap::new();
    if days > 0 {
        for i in STATE_ARR {
            if i == 0 && state.get(&i).is_some() {
                // zero counters spawn new fish (8)
                new_state.insert(8, *state.get(&i).unwrap());
                if new_state.get(&6u8).is_some() {
                    // if there were any existing 6 counters, add the new resets to them
                    *new_state.entry(6u8).or_default() += *state.get(&i).unwrap();
                } else {
                    // otherwise, all the recent resets become 6 counters
                    *new_state.entry(6u8).or_default() = *state.get(&i).unwrap();
                }
            } else if i > 0 && state.get(&i).is_some() {
                // for values besides 0, shift all the previous day's counters down by one
                *new_state.entry(i-1u8).or_default() = *state.get(&i).unwrap();
            }
        }
        // On to the next day
        return calculate_grow(&mut new_state, days-1);
    }
    state.to_owned()
}

fn main() {
    let input_path = format!("{}/day06/src/input.txt", AOC_DIR);
    // let input_path = format!("{}/day06/src/input_example.txt", AOC_DIR);
    let lanternfish = parse_input(input_path.as_str());

    // let days = 18;   // Testing
    // let days = 80;   // Part 1
    let days = 256;  // Part2

    let mut initial_state: HashMap<u8, u64> = HashMap::new();

    for item in &lanternfish {
        *initial_state.entry(*item).or_insert(0) += 1;
    }

    let result_state = calculate_grow(&mut initial_state, days);
    // println!("final result_state - {:?}", result_state);
    let num_fish: u64 = result_state.values().sum();
    println!("Total fish after {} days: {:?}", days, num_fish);
}
