use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

const AOC_DIR: &str = "/Users/p2910482/Projects/rust/AoC2021";

fn read_input<R: Read>(io: R) -> Result<Vec<Vec<u32>>, Error> {
    let br = BufReader::new(io);
    let mut values = vec![];
    for line in br.lines() {
        let line_inner = line.unwrap();
        let nums: Vec<u32> = line_inner
            .chars()
            .map(|c| c.to_digit(10).expect("invalid char"))
            .collect();
        values.push(nums);
    }
    Ok(values)
}

// Take in HashMap tuple values, return binary strings representing filter by
// most significant bit & least significant bit.
fn get_bits_from_tuples(arr: &mut Vec<(usize, i32)>) -> String {
    let mut max_string = String::new();
    arr.sort_by(|a, b| a.0.cmp(&b.0));
    for tup in arr {
        if tup.1 > 0 {
            max_string.push('1');
        } else if tup.1 < 0 {
            max_string.push('0');
        }
    }
    max_string
}

// Part 1: Count bits from 2D array columns
fn count_bits_in_columns(array_2d: &Vec<Vec<u32>>, cols: usize) -> Vec<(usize, i32)> {
    let mut cnt_map = HashMap::new();
    for row in array_2d {
        for i in 0..cols {
            if row[i] == 1 {
                *cnt_map.entry(i).or_insert(0) += 1;
            } else if row[i] == 0 {
                *cnt_map.entry(i).or_insert(0) -= 1;
            }
        }
    }
    cnt_map.into_iter().collect()
}

// Part 2
// Get most or least common value (0 or 1) from a column of binary digits.
fn get_most_or_least_common_value(
    array_2d: &Vec<Vec<u32>>,
    column: usize,
    most_or_least: &str,
) -> u32 {
    let mut cnt_map = HashMap::new();
    for row in array_2d {
        *cnt_map.entry(row[column]).or_insert(0) += 1u32;
    }
    let mut items: Vec<(u32, u32)> = cnt_map.into_iter().collect();
    items.sort_by(|a, b| a.1.cmp(&b.1));
    match most_or_least {
        "most" => {
            if items[0].1 == items[1].1 {
                return 1u32;
            }
            items[1].0
        },
        "least" => {
            if items[0].1 == items[1].1 {
                return 0u32;
            }
            items[0].0
        },
        &_ => panic!("Invalid input: '{}' should be one of 'most' or 'least'", most_or_least)
    }
}

// Perform operations to solve part1 of day03 puzzle
fn part1(most_common: String, cols: usize) -> Result<u32, Error> {
    let gamma = u32::from_str_radix(most_common.as_str(), 2)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    // Perform bitwise NOT on gamma value to get inverse of all bits for epsilon value
    // 01101 -> 10010
    let epsilon = !gamma & 2u32.pow(cols as u32) - 1;
    println!(
        "gamma:   0b{0:0>cols$b} ({0})\nepsilon: 0b{1:0>cols$b} ({1})",
        gamma,
        epsilon,
        cols = cols
    );
    Ok(gamma * epsilon)
}

// Get oxygen generator rating || co2 scrubber rating for part 2 of day03 puzzle
fn part2(array_2d: &Vec<Vec<u32>>, cols: usize, o2_or_co2: &str) -> u32 {
    let mut filter_arr = array_2d.clone();
    for i in 0..cols {
        let choice = match o2_or_co2 {
            "o2" => "most",
            "co2" => "least",
            &_ => panic!("Invalid input: '{}' should be one of 'co2' or 'o2'", o2_or_co2)
        };
        let val = get_most_or_least_common_value(&filter_arr, i, choice);
        filter_arr.retain(|el| el[i] == val);
        if filter_arr.len() == 1 {
            break;
        }
    }
    let mut final_rating = 0u32;
    for i in &filter_arr[0] {
        final_rating <<= 1;
        final_rating |= i;
    }
    println!("{: >3} rating: {:0>cols$b}", o2_or_co2, final_rating, cols = cols);
    final_rating
}

fn main() -> Result<(), Error> {
    let input_path = format!("{}/day03/src/input.txt", AOC_DIR);
    let array_2d = read_input(File::open(input_path)?)?;
    let cols = array_2d[0].len();

    let mut items = count_bits_in_columns(&array_2d, cols);
    let max = get_bits_from_tuples(&mut items);
    println!("Part 1 result: {}\n", part1(max, cols)?);

    let o2_rating = part2(&array_2d, cols, "o2");
    let co2_rating = part2(&array_2d, cols, "co2");
    println!("Part 2 result: {}", o2_rating * co2_rating);

    Ok(())
}
