mod day01;
mod day02;
mod day03;

use day01::run as day01_run;
use day02::run as day02_run;
use day03::run as day03_run;

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
        _ => {
            // eprintln!("Please enter integer between 1-25");
            return Err(Box::new(CustomError::new(
                "Please enter integer between 1-25",
            )));
        }
    };

    result
}
