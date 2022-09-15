#![allow(unused_variables)]
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

fn parse_input(
    string: &String,
    pair_map: &mut HashMap<String, char>,
    char_cnt: &mut HashMap<char, usize>,
) -> String {
    let split = string
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    for line in split[1].lines() {
        let pair_insert = line.clone().split(" -> ").collect::<Vec<&str>>();
        pair_map.insert(pair_insert[0].to_owned(), pair_insert[1].chars().next().unwrap());
    }
    split[0].chars().for_each(|c| {
        *char_cnt.entry(c).or_insert(0) += 1
    });
    split[0].to_owned()
}

fn get_diff_max_min(char_cnt: &HashMap<char, usize>) -> usize {
    let max = char_cnt.values().max().unwrap_or_else(|| panic!("error getting max"));
    let min = char_cnt.values().min().unwrap_or_else(|| panic!("error getting min"));
    max - min
}

fn insert_pairs(
    chars: Vec<char>,
    pair_map: &HashMap<String, char>,
    char_cnt: &mut HashMap<char, usize>,
    steps: usize,
) {
    for pair in chars[..].windows(2) {
        if steps == 0 {
            return
        }
        let pair_key = String::from_iter(pair);
        let mut new_chars = Vec::from(pair);
        // println!("{}", pair_key);
        let insert_char = *pair_map.get(&pair_key).unwrap_or_else(|| panic!("no key"));
        *char_cnt.entry(insert_char).or_insert(0) += 1;
        new_chars.insert(1, insert_char);
        // println!("{:?}", new_chars);
        insert_pairs(new_chars, pair_map, char_cnt, steps-1);
    }
}

fn main() {
    let path = format!(
        "{}/day14/src/input.txt",
        env::var("AOC_DIR").unwrap_or_else(|_| panic!("bad path"))
    );
    let string =
        read_to_string(path.as_str()).unwrap_or_else(|e| panic!("couldn't read file: {}", e));
    let mut pair_map = HashMap::new();
    let mut char_cnt = HashMap::new();
    let template = parse_input(&string, &mut pair_map, &mut char_cnt);
    // println!("Pair map: {:?}, Char count: {:?}", pair_map, char_cnt);
    insert_pairs(template.chars().collect::<Vec<char>>(), &pair_map, &mut char_cnt, 10);
    println!("Char count: {:?}", char_cnt);

    // Part 1
    println!("Part 1 - diff b/t max & min occuring char (10 steps): {}", get_diff_max_min(&char_cnt));

    // Part 2
    pair_map.clear();
    char_cnt.clear();
    let template = parse_input(&string, &mut pair_map, &mut char_cnt);
    insert_pairs(template.chars().collect::<Vec<char>>(), &pair_map, &mut char_cnt, 40);
    println!("Part 2 - diff b/t max & min occuring char (40 steps): {}", get_diff_max_min(&char_cnt));

}

#[test]
fn test_example_input() {
    let path = format!(
        "{}/day14/src/input_example.txt",
        env::var("AOC_DIR").unwrap_or_else(|_| panic!("bad path"))
    );
    let string =
        read_to_string(path.as_str()).unwrap_or_else(|e| panic!("couldn't read file: {}", e));
    let mut pair_map = HashMap::new();
    let mut char_cnt = HashMap::new();
    let template = parse_input(&string, &mut pair_map, &mut char_cnt);
    insert_pairs(template.chars().collect::<Vec<char>>(), &pair_map, &mut char_cnt, 10);
    assert!(*char_cnt.get(&'B').unwrap() == 1749);
    assert!(*char_cnt.get(&'C').unwrap() == 298);
    assert!(*char_cnt.get(&'H').unwrap() == 161);
    assert!(*char_cnt.get(&'N').unwrap() == 865);
    assert!(get_diff_max_min(&char_cnt) == 1588);
}
