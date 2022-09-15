use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

static OPEN_SYMBOLS: [char; 4] = ['<', '(', '{', '['];
static CLOSE_SYMBOLS: [char; 4] = ['>', ')', '}', ']'];
static PAIRS: [(char, char); 4] = [('>', '<'), (')', '('), ('}', '{'), (']', '[')];

fn parse_input(path: &str) -> Vec<Vec<char>> {
    let reader =
        BufReader::new(File::open(path).unwrap_or_else(|e| panic!("Error opening file: {}", e)));
    let mut parsed = vec![];

    for line in reader.lines() {
        parsed.push(line.unwrap().chars().collect())
    }
    parsed
}

/// Swap key/value pairs of arbitrary type and return a new `HashMap`.
/// Could have just made all the maps static, but I'm learning about generics!
fn swap_hashmap<T: Copy, U: Eq + Hash + Copy>(map: &HashMap<T, U>) -> HashMap<U, T> {
    let mut new_map = HashMap::new();
    for (k, v) in map.iter() {
        new_map.insert(*v, *k);
    }
    new_map
}

/// Solve part one of Day 10 AoC 2021.
///
/// Returns a tuple of (incomplete lines: `Vec<Vec<char>>`, and symbols on which a corrupt line is stopped: Vec<char>).
fn part1(nav_sys: Vec<Vec<char>>, pairs: &HashMap<char, char>) -> (Vec<Vec<char>>, Vec<char>) {
    // let expected_symbols = swap_hashmap(&pairs);

    let mut incomplete = vec![];

    let mut corrupted = vec![];
    'lines: for line in &nav_sys {
        let mut open = vec![];
        for c in line {
            if OPEN_SYMBOLS.contains(c) {
                open.push(c);
            } else if CLOSE_SYMBOLS.contains(c) {
                let match_sym = match open.pop() {
                    Some(sym) => *sym,
                    None => {
                        println!("No symbols in queue");
                        continue;
                    }
                };
                let expected_sym = *pairs.get(&c).expect("Key error");
                if match_sym != expected_sym {
                    #[rustfmt::skip]
                    // println!("Expected '{}', but found '{}' instead.", expected_symbols.get(&match_sym).unwrap(), c);
                    corrupted.push(*c);
                    continue 'lines;
                }
            }
        }
        // if we get here with no errors, we have an incomplete
        incomplete.push(open.iter().map(|c| *c.to_owned()).collect::<Vec<char>>());
    }
    (incomplete, corrupted)
}

/// In `fn part1(...)`, only the remaining open brackets are returned, making it easier to match those pairs.
fn part2(incompletes: &mut Vec<Vec<char>>, pairs: &HashMap<char, char>) -> u64 {
    let mut scores: Vec<u64> = vec![];
    while !incompletes.is_empty() {
        let mut closers: Vec<char> = vec![];
        let mut line = incompletes.pop().unwrap();
        while !line.is_empty() {
            let c = line.pop().unwrap();
            closers.push(*pairs.get(&c).unwrap());
        }
        scores.push(calculate_autocomplete_score(closers))
    }
    scores.sort();
    scores[scores.len() / 2]
}

fn calculate_autocomplete_score(symbols: Vec<char>) -> u64 {
    let mut total_score = 0;
    for c in symbols {
        total_score *= 5;
        total_score += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("invalid character: '{}'", c),
        }
    }
    total_score
}

pub fn run(example: bool) {
    let path = if example == true {
        "inputs/day10_example.txt"
    } else {
        "inputs/day10.txt"
    };
    let navigation = parse_input(path);

    // Part 1
    let mut pairs = HashMap::from(PAIRS);
    let (mut incomplete, corrupted) = part1(navigation, &pairs);
    let sum: u32 = corrupted
        .iter()
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("invalid char: '{}'", c),
        })
        .sum();
    println!("Part 1 - corrupt character score: {}", sum);

    // Part 2
    pairs = swap_hashmap(&pairs); // swap '>': '<' for '<': '>'
    let score = part2(&mut incomplete, &pairs);
    println!("Part 2 - autocompleter score: {}", score);
}
