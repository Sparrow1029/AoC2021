use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

const AOC_DIR: &str = "/Users/p2910482/Projects/rust/AoC2021";


fn read_ints<R: Read>(io: R) -> Result<Vec<usize>, Error> {
    let br = BufReader::new(io);
    let mut values = vec![];
    for line in br.lines() {
        values.push(line?
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?);
    }
    Ok(values)
}

fn main() -> Result<(), Error> {
    let input_path = format!("{}/day01/src/input.txt", AOC_DIR);
    let puzzle_input = read_ints(File::open(input_path)?)?;

    let mut cur_depth: usize = puzzle_input[0];
    let mut count_increases: usize = 0;

    for line in &puzzle_input {
        if line > &cur_depth {
            count_increases += 1;
        }
        cur_depth = *line;
    }
    println!("Part 1 -- Total depth increases: {}", count_increases);

    // println!("len puzzle_input: {}", puzzle_input.len());
    count_increases = 0;
    let mut prev_sum = 0;
    for window in puzzle_input.windows(3) {
        let cur_sum = window.into_iter().sum::<usize>();
        if prev_sum == 0 {
            // naïve way to get first sum. Rust is hard.
            prev_sum = cur_sum;
            continue
        }
        if cur_sum > prev_sum {
            count_increases += 1;
        }
        prev_sum = cur_sum;
        // print!("WINDOW {:?}", window);
        // println!(" -- SUM {:?}", &window.into_iter().sum::<usize>());
    }
    println!("Part 2 -- Total depth increases (sliding window of 3 values): {}", count_increases);
    Ok(())
}
