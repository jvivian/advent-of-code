use std::collections::HashSet;
use std::num::ParseIntError;
use std::path::PathBuf;
use std::str::FromStr;
use thiserror::Error;
/// Advent of Code
/// Day 9 - Part 1
/// Author: John Vivian

/// Solve by tracking the coords of both the (H)ead and the (T)ail
/// After a given move by the Head, determine if the Tail needs to move
/// If not, don't do shit
/// If so, follow these rules for moving
/// - If the head is ever two steps directly up, down, left, or right from the tail, the tail must also move one step in that direction so it remains close enough:
/// - Otherwise, if the head and tail aren't touching and aren't in the same row or column, the tail always moves one step diagonally to keep up:
/// Store Tail moves in a HashSet, count total

/// Iterate over lines in file
/// Convert line -> MOVE enum  [x]
/// for n in N moves
/// - Move (H) Coord
/// - Given H and T Coord -> Tail Action
/// -- Overlap => None
/// -- Adjacent => None
/// -- AlignedGap => Move_Cardinal
/// -- UnalignedGap => Move_Diagonal_Adjacent

#[derive(Debug, PartialEq)]
enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

#[derive(Error, Debug, PartialEq)]
enum ParseMoveError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Failed to parse number of moves")]
    IntError(#[from] ParseIntError),
    #[error("Failed to parse direction {0}")]
    DirError(String),
}

impl FromStr for Move {
    type Err = ParseMoveError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (d, n) = s
            .split_once(' ')
            .ok_or(ParseMoveError::InvalidInput(s.into()))?;
        match d {
            "U" => Ok(Move::Up(n.parse()?)),
            "D" => Ok(Move::Down(n.parse()?)),
            "L" => Ok(Move::Left(n.parse()?)),
            "R" => Ok(Move::Right(n.parse()?)),
            _ => Err(ParseMoveError::DirError(d.into())),
        }
    }
}

struct Coord(i32, i32);

impl Coord {
    fn _move(self, m: Move) -> Self {
        match m {
            Move::Up(n) => Coord(self.0, self.1 + n),
            Move::Down(n) => Coord(self.0, self.1 - n),
            Move::Left(n) => Coord(self.0 - n, self.1),
            Move::Right(n) => Coord(self.0 + n, self.1),
        }
    }
}

struct Rope {
    head: Coord,
    tail: Coord,
    // Store Tail positions
    tpos: HashSet<Coord>,
}

impl Rope {
    // Move Rope according to input
    fn move_head(self, m: Move) -> Self {}
}

// enum TailAction {
//     Overlap,
//     Adjacent,
//     AlignedGap,
//     UnalignedGap,
// }

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.txt")
    }

    #[test]
    fn test_coord_parse() {
        assert_eq!(Move::from_str("R 4"), Ok(Move::Right(4)));
    }
}
