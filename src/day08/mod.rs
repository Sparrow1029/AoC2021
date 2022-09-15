use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

/// Parse day08 input file (lines consisting of 7 space-separated strings of chars a-g, delimeter `'|'` and 4 space-separated strings of chars a-g)
/// representative of digital number display (when working correctly):
/// ```
///   0:      1:      2:      3:      4:
///   aaaa    ....    aaaa    aaaa    ....
///  b    c  .    c  .    c  .    c  b    c
///  b    c  .    c  .    c  .    c  b    c
///   ....    ....    dddd    dddd    dddd
///  e    f  .    f  e    .  .    f  .    f
///  e    f  .    f  e    .  .    f  .    f
///   gggg    ....    gggg    gggg    ....
///
///    5:      6:      7:      8:      9:
///   aaaa    aaaa    aaaa    aaaa    aaaa
///  b    .  b    .  .    c  b    c  b    c
///  b    .  b    .  .    c  b    c  b    c
///   dddd    dddd    ....    dddd    dddd
///  .    f  e    f  .    f  e    f  .    f
///  .    f  e    f  .    f  e    f  .    f
///   gggg    gggg    ....    gggg    gggg
/// ```
fn parse_input(path: &str) -> Result<Vec<(Vec<String>, Vec<String>)>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut input_output = vec![];

    for line in reader.lines() {
        let inner: Vec<String> = line?.split(" | ").map(|a| a.to_string()).collect();
        let input = inner[0]
            .split(' ')
            .map(|word| word.chars().sorted().collect::<String>())
            .collect();
        let output = inner[1]
            .split(' ')
            .map(|word| word.chars().sorted().collect::<String>())
            .collect();
        input_output.push((input, output));
    }
    Ok(input_output)
}

fn part1(outputs: &Vec<Vec<String>>) {
    let mut counter = HashMap::new();
    for output in outputs {
        for key in output {
            *counter.entry(key).or_insert(0) += 1;
        }
    }

    let mut total_uniq = 0;
    let nums: [usize; 4] = [2, 3, 4, 7]; // 1, 4, 7, and 8
    for tup in counter.iter() {
        if nums.contains(&tup.0.len()) {
            total_uniq += tup.1;
        }
    }
    println!("Total unique outputs (Part 1): {}", total_uniq);
}

fn part2(inputs: &Vec<Vec<String>>, outputs: &Vec<Vec<String>>) {
    let mut total_sum = 0;
    for i in 0..inputs.len() {
        let (mut inp, out) = (inputs[i].to_owned(), outputs[i].to_owned());
        let mut num_map = HashMap::new();
        populate_num_map(&mut inp, &mut num_map);
        let final_num = u32::from_str_radix(
            out.iter()
                .map(|v| num_map.get(v).unwrap().to_string())
                .collect::<String>()
                .as_str(),
            10,
        )
        .unwrap();
        total_sum += final_num;
    }
    println!("Part 2 - total_sum: {}", total_sum);
}

/// Had to do this a really ugly way. Tried multiple other methods for filtering, and just couldn't
/// implement this like a smart programmer. Gist of this function is to:
/// - take in a line of "input" signals from the puzzle input
/// - use a `loop` construct with essentially a `goto` & hard-coded lengths to
///   1. consume each item using `.remove` as items are filtered...
///   2. using HashSet logic to determine which display number is mapped to each string
///   3. modify `num_map` HashMap in-place to create string->display digit mapping
fn populate_num_map(input: &mut Vec<String>, num_map: &mut HashMap<String, u8>) {
    input.sort_by_key(|word| word.len());
    // println!("Input sorted: {:?}", input);
    num_map.insert(input.pop().unwrap(), 8);
    let one = input.remove(0);
    num_map.insert(one.clone(), 1);
    num_map.insert(input.remove(0), 7);
    let four = input.remove(0);
    num_map.insert(four.clone(), 4);
    let mut six: String = String::new();
    let mut nine: String = String::new();
    'outer: loop {
        match input.len() {
            // search for 6, 9, 0
            6 => {
                // println!("input: {:?} - len: {}", input, input.len());
                for i in 3..6 {
                    if !HashSet::<char>::from_iter(one.chars())
                        .is_subset(&HashSet::<char>::from_iter(input[i].chars()))
                    {
                        six = input.remove(i);
                        num_map.insert(six.clone(), 6);
                        continue 'outer;
                    }
                }
            }
            5 => {
                for i in 3..5 {
                    if !HashSet::<char>::from_iter(four.chars())
                        .is_subset(&HashSet::<char>::from_iter(input[i].chars()))
                    {
                        num_map.insert(input.remove(i), 0);
                        continue 'outer;
                    }
                }
            }
            4 => {
                nine = input.pop().unwrap();
                num_map.insert(nine.clone(), 9);
                continue 'outer;
            }
            // search for 5, 3, 2
            3 => {
                for i in 0..3 {
                    if HashSet::<char>::from_iter(six.chars())
                        .is_superset(&HashSet::<char>::from_iter(input[i].chars()))
                    {
                        num_map.insert(input.remove(i), 5);
                        continue 'outer;
                    }
                }
            }
            2 => {
                for i in 0..2 {
                    if HashSet::<char>::from_iter(nine.chars())
                        .is_superset(&HashSet::<char>::from_iter(input[i].chars()))
                    {
                        num_map.insert(input.remove(i), 3);
                        continue 'outer;
                    }
                }
            }
            1 => {
                num_map.insert(input.pop().unwrap(), 2);
                break;
            }
            _ => panic!(),
        }
    }
}

pub fn run(example: bool) -> io::Result<()> {
    let file_path = if example == true {
        "inputs/day08_example.txt"
    } else {
        "inputs/day08.txt"
    };

    let display = parse_input(file_path).expect("Couldn't parse input file");

    let mut inputs = vec![];
    let mut outputs = vec![];
    for item in display {
        inputs.push(item.0);
        outputs.push(item.1);
    }

    part1(&outputs);

    part2(&inputs, &outputs);

    Ok(())
}
