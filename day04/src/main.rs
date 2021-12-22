use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader, Error, Read};

const AOC_DIR: &str = "/Users/p2910482/Projects/rust/AoC2021";

/// Trying to learn how to consume buffers instead of just converting it to
/// String on the heap and working with .split()...
fn read_input<R: Read>(io: R) -> Result<(Vec<u32>, Vec<String>), Error> {
    let mut br = BufReader::new(io);

    let mut first_line = String::new();
    br.read_line(&mut first_line)?;
    // skip the next line -- it's a \n
    br.read_line(&mut first_line)?;
    first_line = first_line.trim().to_string();

    let nums: Vec<u32> = first_line.split(',').map(|v| v.parse().unwrap()).collect();
    first_line.clear();

    let mut cur_board = String::new();
    let mut raw_boards: Vec<String> = vec![];
    for line in br.lines() {
        let inner = line.unwrap();
        if inner.len() == 0 {
            raw_boards.push(cur_board.clone());
            cur_board.clear();
        }
        cur_board += format!("{}\n", inner).as_str();
    }
    if cur_board.len() != 0 {
        // missing the last block. Rust is hard.
        raw_boards.push(cur_board);
    }
    Ok((nums, raw_boards))
}

/// Bingo board!
///
/// represented as a 5 x 5 2d array of tuples:
///
/// `(flag, value)`
/// - flag is 0 or 1 to denote a pip on that square
/// - value is the value of the square
#[derive(Clone, Copy)]
struct Board {
    squares: [[(u8, u32); 5]; 5],
    winner: bool,
    id: u64,
    winning_number: u32,
}

impl Board {
    fn new() -> Board {
        Board {
            squares: [[(0u8, 0u32); 5]; 5],
            winner: false,
            id: 0,
            winning_number: 0,
        }
    }

    /// Search board for number and flag that square's tuple
    fn update_board_with_number(&mut self, num: u32) {
        for row in &mut self.squares {
            for i in 0..row.len() {
                if row[i].1 == num {
                    row[i].0 = 1;
                }
            }
        }
    }

    /// Insert a tuple into board square
    fn insert(&mut self, x: usize, y: usize, val: u32) {
        self.squares[y][x] = (0u8, val);
    }

    /// Create a board from a newline-separated `&String` buffer.
    fn from(buf: &String) -> Board {
        let mut board = Board::new();
        let mut y = 0usize;
        let mut values = vec![];

        for line in buf.trim().split('\n') {
            let mut spl: Vec<&str> = line.trim().split(' ').collect::<Vec<&str>>();
            spl.retain(|c| c.len() != 0);
            let nums: Vec<u32> = spl.into_iter().map(|v| v.trim().parse().unwrap()).collect();
            for x in 0..5 {
                board.insert(x, y, nums[x]);
                values.push(nums[x]);
            }
            y += 1;
        }
        // update self.id with hash value
        let mut s = DefaultHasher::new();
        values.hash(&mut s);
        board.id = s.finish();
        board
    }

    /// Check board rows and then columns to determine win state.
    fn check_for_win(&mut self) {
        for row in self.squares {
            let pips_cnt = row.iter().filter(|tup| tup.0 == 1).count();
            if pips_cnt == 5 {
                self.winner = true;
            }
        }
        for i in 0..5 {
            let mut pips_cnt = 0u8;
            for row in self.squares {
                if row[i].0 == 1 {
                    pips_cnt += 1;
                }
            }
            if pips_cnt == 5 {
                self.winner = true;
            }
        }
    }

    fn calculate_board_product(&self) -> u32 {
        let mut unmarked = 0;
        for row in self.squares {
            unmarked += row
                .iter()
                .map(|tup| if tup.0 == 0 { tup.1 } else { 0 })
                .sum::<u32>();
        }
        // println!("unmarked sum: {}, winning_number: {}\nProduct: {}", unmarked, self.winning_number, unmarked * self.winning_number);
        unmarked * self.winning_number
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::from("\n.====================.\n");
        for row in self.squares {
            string.push('[');
            for i in row {
                let bingo: &str = match i.0 {
                    1 => "\x1b[0;91m\x1b[1m",
                    _ => "",
                };
                let num = format!("{}{: >3}\x1b[0m,", bingo, i.1);
                string.push_str(num.as_str());
            }
            string = string.trim_end_matches(',').to_string();
            string += " ]\n";
        }
        string += "´====================`";
        write!(f, "{}", string)
    }
}

/// Call `update_board_with_number` for all bingo boards & check for any new winners.
///
/// Returns a winning board if there is a new winner.
fn call_bingo(boards: &mut Vec<Board>, number: u32) -> Option<&mut Board> {
    // Though there may be multiple winners for a single round/number, part 2 only cares about the
    // last winning board, so it doesn't matter if this `mut winner` gets overwritten several times here
    let mut winner: Option<&mut Board> = None;

    for board in boards {
        if !board.winner == true {
            board.update_board_with_number(number);
            board.check_for_win();
            if board.winner == true {
                board.winning_number = number;
                winner = Some(board)
            }
        }
    }

    winner
}

fn main() -> Result<(), Error> {
    let input_path = format!("{}/day04/src/input.txt", AOC_DIR);
    let (nums, raw_boards) = read_input(File::open(input_path)?)?;
    let mut boards: Vec<Board> = raw_boards.iter().map(|b| Board::from(b)).collect();
    println!("PLAY BINGO!! *COUGH* *COUGH* (so much smoke in this submarine...)");

    let mut winner_ids: Vec<u64> = Vec::new();

    for number in &nums {
        let bingo = call_bingo(&mut boards, *number);
        match bingo {
            Some(board) => {
                if winner_ids.len() == 0 {
                    // Part 1
                    println!("First Win! {}", board);
                    let product = board.calculate_board_product();
                    println!("Part 1 answer: {}\n=============\n", product);
                }
                if !winner_ids.contains(&board.id) {
                    winner_ids.push(board.id)
                }
            }
            None => continue,
        }
    }
    // Part 2
    let last_board = boards
        .iter()
        .find(|b| &b.id == winner_ids.last().unwrap())
        .unwrap();
    println!("Last board to win: {}", last_board);
    println!("Part 2 answer: {}: ", last_board.calculate_board_product());

    Ok(())
}
