use std::collections::{BTreeMap, HashMap};
use std::env::var;
use std::fs::read_to_string;
use std::hash::Hash;

type RuleMap = BTreeMap<String, char>;
type CharArr = [isize; 26];
type Counter = BTreeMap<char, isize>;

// fn char_to_id(ch: char) -> usize {
//     let char_id = ch as usize - b'A' as usize;
//     println!("char id for {}: {}", ch, char_id);
//     char_id
// }

fn counter_from_str<S: Into<String>>(s: S) -> Counter {
    let mut counter = BTreeMap::new();
    s.into().chars().for_each(|ch| {
        *counter.entry(ch).or_insert(0) += 1;
    });
    counter
}

fn char_windows<'a>(src: &'a str, win_size: usize) -> impl Iterator<Item = &'a str> {
    src.char_indices().flat_map(move |(from, _)| {
        src[from..]
            .char_indices()
            .skip(win_size - 1)
            .next()
            .map(|(to, c)| &src[from..from + to + c.len_utf8()])
    })
}

fn memoize<A, R, F>(
    cache: &mut HashMap<(A, usize), R>,
    func: F,
    arg: A,
    rules: &RuleMap,
    steps: usize,
) -> R
where
    A: Eq + Hash + Clone,
    R: Clone,
    F: Fn(&mut HashMap<(A, usize), R>, A, &RuleMap, usize) -> R,
{
    match cache.get(&(arg.clone(), steps)).map(|x| x.clone()) {
        Some(result) => result,
        None => {
            let result = func(cache, arg.clone(), rules, steps);
            cache.insert((arg, steps), result.clone());
            result
        }
    }
}

fn counter_diff(a: Counter, b: Counter) -> Counter {
    let mut counter: Counter = BTreeMap::new();
    for (k, v) in a.iter() {
        counter.insert(*k, *v);
        let b_val = b.get(&k).unwrap_or(&0_isize);
        *counter.entry(*k).or_insert(0) -= b_val;
    }
    println!("counter before retain{:?}", counter);
    counter.retain(|_, v| *v > 0);
    println!("DIFF_RES {:?}", counter);
    counter
}

fn counter_sum(a: Counter, b: Counter) -> Counter {
    let mut counter: Counter = BTreeMap::new();
    println!("SUMMING a: {:?}, b: {:?}", a, b);
    for (k, v) in a.iter() {
        counter.insert(*k, *v);
        let b_val = b.get(&k).unwrap_or(&0_isize);
        *counter.entry(*k).or_insert(0) += v + b_val;
    }
    println!("SUM RES: {:?}", counter);
    counter
}

fn parse_input(filename: &str) -> (String, RuleMap, Counter) {
    let mut rule_map: RuleMap = BTreeMap::new();
    let path = format!(
        "{}/day14/src/{}",
        var("AOC_DIR").unwrap_or_else(|e| panic!("Env var error 'AOC_DIR': {}", e)),
        filename
    );
    let string =
        read_to_string(path.as_str()).unwrap_or_else(|e| panic!("error opening file: {}", e));
    let (template, rules) = string.split_once("\n\n").unwrap();
    rules.lines().into_iter().for_each(|line| {
        let (pair, ch) = line.split_once(" -> ").unwrap();
        rule_map.insert(pair.to_owned(), ch.chars().next().unwrap());
    });
    (template.to_string(), rule_map, counter_from_str(template))
}

fn count(
    cache: &mut HashMap<(String, usize), Counter>,
    poly: String,
    rules: &RuleMap,
    steps: usize,
) -> Counter {
    if steps == 0 {
        return counter_from_str(poly);
    };

    if poly.len() > 2 {
        let vec = poly.chars().map(|c| c.to_string()).collect::<Vec<String>>();
        let middle = &vec[1..vec.len() - 1].join("");
        for pair in char_windows(poly.as_str(), 2) {
            let hit = match cache.get(&(pair.to_string(), steps)) {
                Some(result) => result.clone(),
                None => memoize(cache, count, pair.to_string(), rules, steps - 1)
            };
            return counter_diff(
                counter_sum(
                    hit,
                    BTreeMap::new(),
                ),
                counter_from_str(middle),
            );
        }
    }
    let new_char = rules.get(poly.as_str()).unwrap().to_string();
    let p1 = format!("{}{}", poly.chars().next().unwrap(), &new_char);
    println!("p1: {}", p1);
    let p2 = format!("{}{}", poly.chars().last().unwrap(), &new_char);
    println!("p2: {}", p2);
    let result = counter_diff(
        counter_sum(
            memoize(cache, count, p1, rules, steps-1),
            memoize(cache, count, p2, rules, steps-1),
        ), counter_from_str(new_char.to_string()));
    cache.insert((poly, steps-1), result.clone());
    println!("SUM DIFF: {:?}", result);
    result
}

fn main() {
    let (template, rules, initial_counter) = parse_input("input_example.txt");
    println!(
        "template: {}\nrule map: {:?}\nchar arr counts: {:?}",
        template, rules, initial_counter
    );
    let mut cache = HashMap::new();
    let result = count(&mut cache, template, &rules, 6);
    println!("result: {:?}", result);
    println!(": {:?}", cache);
}
