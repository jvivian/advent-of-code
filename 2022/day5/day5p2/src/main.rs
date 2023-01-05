use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::str;
use std::{num::ParseIntError, str::FromStr};
use thiserror::Error;
use type_cli::CLI;

#[derive(CLI)]
struct Input(String);

fn get_lines(path: &String) -> Lines<BufReader<File>> {
    BufReader::new(File::open(path).expect("Failed to open file")).lines()
}

// Storage Crates
#[derive(Debug, Default)]
struct Crates {
    // Holds stacks of crates
    stacks: Vec<Stacks>,
    // Number of lines to skip after stacks
    skip: usize,
}

// Represents one vertical stack of crates
type Stacks = Vec<char>;

// Holds one crate move
#[derive(Debug)]
struct Move {
    v1: usize,
    v2: usize,
    n: usize,
}

#[derive(Error, Debug)]
enum ParseMoveError {
    #[error("Expected integer")]
    InvalidInt(#[from] ParseIntError),
    #[error("Expected `move _ from _ to _` found {0}")]
    InvalidInput(String),
}

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse: Vec<&str> = s.split_whitespace().collect();
        if parse.len() != 6 {
            return Err(ParseMoveError::InvalidInput(parse.join(" ")));
        }
        Ok(Move {
            v1: parse[3].parse()?,
            v2: parse[5].parse()?,
            n: parse[1].parse()?,
        })
    }
}

// Move crate
fn move_crate(m: &Move, c: &mut Crates, n: usize) {
    let v1 = &mut c.stacks[m.v1 - 1];
    let ix = v1.len() - 1;
    let x = v1[ix];
    v1.remove(ix);
    let v2 = &mut c.stacks[m.v2 - 1];
    v2.push(x);
    if n > 1 {
        move_crate(m, c, n - 1)
    }
}

// Move crates as a block instead of 1 at a time
fn move_crates(m: &Move, c: &mut Crates) {
    let v1 = &mut c.stacks[m.v1 - 1];
    let xs = v1[v1.len() - m.n..].to_vec();
    for i in (v1.len() - m.n..v1.len()).rev() {
        v1.remove(i);
    }
    let v2 = &mut c.stacks[m.v2 - 1];
    v2.extend(xs);
}

// Parse row containing crate information
fn parse_crate_row(s: &str) -> Vec<char> {
    s.chars().skip(1).step_by(4).collect()
}

// Parse text representation of crate stacks
fn parse_crate_stacks(lines: Lines<BufReader<File>>) -> Crates {
    let mut crates = Crates::default();
    for (i, line) in lines.enumerate() {
        if let Ok(x) = line {
            // Reached end of crates, record position and break iter
            if x.starts_with(" 1") {
                crates.skip = i + 2;
                break;
            }
            let row = parse_crate_row(&x);
            // Instantiate stacks if not done so
            if crates.stacks.is_empty() {
                for _ in 0..row.len() {
                    crates.stacks.push(vec![])
                }
            }
            // Insert values from parsed row to stacks
            for i in 0..row.len() {
                if row[i] != ' ' {
                    crates.stacks[i].insert(0, row[i]);
                }
            }
        }
    }
    crates
}

fn main() {
    let Input(path) = Input::process();
    let mut crates = parse_crate_stacks(get_lines(&path));
    let lines = get_lines(&path);
    for line in lines.skip(crates.skip) {
        if let Ok(x) = line {
            let m = Move::from_str(&x).unwrap();
            move_crates(&m, &mut crates);
        }
    }
    // Print top of each stack
    for stack in crates.stacks.iter() {
        println!("{:?}", stack[stack.len() - 1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    fn test_path() -> String {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("test")
            .join("test.txt")
            .as_path()
            .display()
            .to_string()
    }

    #[test]
    fn test_parse_crate_str() {
        let s = "[M] [B] [F]         [P] [C] [H] [N]";
        let res = parse_crate_row(s);
        assert_eq!(vec!['M', 'B', 'F', ' ', ' ', 'P', 'C', 'H', 'N'], res);
    }

    #[test]
    fn test_parse_crates() {
        let path = test_path();
        let lines = get_lines(&path);
        let crates = parse_crate_stacks(lines);
        println!("{:?}", crates);
        println!("{:?}", crates.stacks[0]);
    }

    #[test]
    fn test_parse_move_str() {
        let s = "move 10 from 6 to 8";
        let m = Move::from_str(s).unwrap();
        assert_eq!(6, m.v1);
        assert_eq!(8, m.v2);
        assert_eq!(10, m.n);
    }

    #[test]
    fn test_parse_and_move_crate() {
        let path = test_path();
        let mut crates = parse_crate_stacks(get_lines(&path));
        let lines = get_lines(&path);
        for line in lines.skip(crates.skip) {
            if let Ok(x) = line {
                let m = Move::from_str(&x).unwrap();
                move_crate(&m, &mut crates, m.n);
            }
        }
        assert_eq!(vec!['C'], crates.stacks[0]);
        assert_eq!(vec!['M'], crates.stacks[1]);
    }

    #[test]
    fn test_move_crates() {
        let path = test_path();
        let mut crates = parse_crate_stacks(get_lines(&path));
        let lines = get_lines(&path);
        for line in lines.skip(crates.skip) {
            if let Ok(x) = line {
                let m = Move::from_str(&x).unwrap();
                move_crates(&m, &mut crates);
                println!("{:?}", m);
                println!("{:?}", crates);
            }
        }
        // assert_eq!(vec!['C'], crates.stacks[0]);
        // assert_eq!(vec!['M'], crates.stacks[1]);
    }
}
