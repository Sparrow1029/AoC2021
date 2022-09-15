mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day15;
mod day16;
mod day17;

use day01::run as day01_run;
use day02::run as day02_run;
use day03::run as day03_run;
use day04::run as day04_run;
use day05::run as day05_run;
use day06::run as day06_run;
use day07::run as day07_run;
use day08::run as day08_run;
use day09::run as day09_run;
use day10::run as day10_run;
use day11::run as day11_run;
use day12::run as day12_run;
use day13::run as day13_run;
use day15::run as day15_run;
use day16::run as day16_run;
use day17::run as day17_run;

use std::env;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct CustomError {
    details: String,
}
impl CustomError {
    fn new(msg: &str) -> CustomError {
        CustomError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for CustomError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let example: bool = args.contains(&String::from("example"));
    let query = args
        .iter()
        .nth(1)
        .expect("OH MY GOD IT'S ALL BREAKING APART");
    println!("\nRunning day {}...", query);
    let result = match query.as_str() {
        "1" => {
            day01_run()?;
            Ok(())
        }
        "2" => {
            day02_run()?;
            Ok(())
        }
        "3" => {
            day03_run()?;
            Ok(())
        }
        "4" => {
            day04_run(example)?;
            Ok(())
        }
        "5" => {
            let part = args
                .iter()
                .nth(2)
                .expect("Please enter part (1 or 2)")
                .parse()
                .unwrap();
            if args.len() == 4 {
                day05_run(part, args.iter().nth(3))?;
            } else {
                day05_run(part, None)?;
            }
            Ok(())
        }
        "6" => {
            let part = args
                .iter()
                .nth(2)
                .expect("Please enter part: 0 for test data, 1 or 2 for actual puzzle")
                .parse()
                .unwrap();
            if args.len() == 4 && args.iter().nth(3).unwrap() == "example" {
                day06_run(part, true);
            } else {
                day06_run(part, false);
            }
            Ok(())
        }
        "7" => {
            if args.len() == 3 {
                day07_run(args.iter().nth(2));
            } else {
                day07_run(None);
            }
            Ok(())
        }
        "8" => {
            day08_run(example)?;
            Ok(())
        }
        "9" => {
            day09_run(example);
            Ok(())
        }
        "10" => {
            day10_run(example);
            Ok(())
        }
        "11" => {
            if args.contains(&String::from("animate")) {
                env::set_var("DEBUG", "true")
            }
            day11_run(example);
            Ok(())
        }
        "12" => {
            if args.contains(&String::from("debug")) {
                env::set_var("DEBUG", "true");
            }
            day12_run(example);
            Ok(())
        }
        "13" => {
            day13_run(example);
            Ok(())
        }
        "15" => {
            if args.contains(&String::from("debug")) {
                env::set_var("DEBUG", "true");
            }
            day15_run(example);
            Ok(())
        }
        "16" => {
            day16_run();
            Ok(())
        }
        "17" => {
            day17_run(example);
            Ok(())
        }
        _ => {
            // eprintln!("Please enter integer between 1-25");
            return Err(Box::new(CustomError::new(
                "Please enter integer between 1-25",
            )));
        }
    };

    result
}
