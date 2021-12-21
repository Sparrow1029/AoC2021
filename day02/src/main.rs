use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

const AOC_DIR: &str = "/Users/p2910482/Projects/rust/AoC2021";

fn read_input<R: Read>(io: R) -> Result<Vec<(String, i32)>, Error> {
    let br = BufReader::new(io);
    let mut values = vec![];
    for line in br.lines() {
        let line_inner = line.unwrap();
        let split: Vec<&str> = line_inner.split(' ').collect();
        values.push((split[0].to_string(), split[1].parse().expect("OH NOES")))
    }
    Ok(values)
}

#[derive(Default)]
struct Submarine {
    h_pos: i32,
    v_pos: i32,
    aim: i32,
}

impl Submarine {
    fn move_part1(&mut self, instruction: (String, i32)) {
        let (direction, distance) = instruction;
        match direction.as_str() {
            "forward" => self.h_pos += distance,
            "up" => self.v_pos -= distance,
            "down" => self.v_pos += distance,
            &_ => println!("Invalid instruction: ({:?}, {:?})", direction, distance)
        }
    }

    fn move_part2(&mut self, instruction: (String, i32)) {
        let (direction, distance) = instruction;
        match direction.as_str() {
            "forward" => {
                self.h_pos += distance;
                self.v_pos += self.aim * distance
            },
            "up" => self.aim -= distance,
            "down" => self.aim += distance,
            &_ => println!("Invalid instruction: ({:?}, {:?})", direction, distance),
        }
    }

    fn display_pos(&self) {
        println!("X/Y: {} -- Z: {}", self.h_pos, self.v_pos);
    }

    fn surface(&mut self) {
        self.h_pos = 0;
        self.v_pos = 0;
        self.aim = 0;
    }
}

fn main() -> Result<(), Error> {
    let input_path = format!("{}/day02/src/input.txt", AOC_DIR);
    let puzzle_input = read_input(File::open(input_path)?)?;
    // println!("{:?}", puzzle_input);
    let mut sub: Submarine = Default::default();

    println!("PART 1:");
    for instruction in &puzzle_input {
        // println!("Moving -> {:?}", instruction);
        sub.move_part1(instruction.clone());
    }
    sub.display_pos();
    println!("{}\n", sub.h_pos * sub.v_pos);

    sub.surface();

    println!("PART 2:");
    for instruction in puzzle_input {
        sub.move_part2(instruction);
    }
    sub.display_pos();
    println!("{}\n", sub.h_pos * sub.v_pos);

    Ok(())
}
