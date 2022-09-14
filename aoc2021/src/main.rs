mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day16;

use day01::run as day01_run;
use day02::run as day02_run;
use day03::run as day03_run;
use day04::run as day04_run;
use day05::run as day05_run;
use day06::run as day06_run;
use day07::run as day07_run;
use day08::run as day08_run;
use day09::run as day09_run;
use day16::run as day16_run;

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
            if args.len() == 3 {
                day04_run(args.iter().nth(2))?;
            } else {
                day04_run(None)?;
            }
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
            if args.len() == 3 && args[2] == "example" {
                day08_run(true)?;
            } else {
                day08_run(false)?;
            }
            Ok(())
        }
        "9" => {
            if args.len() == 3 && args[2] == "example" {
                day09_run(true);
            } else {
                day09_run(false);
            }
            Ok(())
        }
        "16" => {
            day16_run();
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
