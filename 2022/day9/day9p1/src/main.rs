use aoclib::get_lines;
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;
use type_cli::CLI;
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

#[derive(CLI)]
struct Input(String);

#[derive(Debug, PartialEq)]
struct Move {
    n: i32,
    dir: Direction,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Error, Debug, PartialEq)]
enum ParseMoveError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Failed to parse number of moves")]
    IntError(#[from] ParseIntError),
    #[error("Failed to parse direction {0}")]
    InvalidDirection(String),
}

impl FromStr for Move {
    type Err = ParseMoveError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (d, n) = s
            .split_once(' ')
            .ok_or(ParseMoveError::InvalidInput(s.into()))?;
        match d {
            "U" => Ok(Move {
                n: n.parse()?,
                dir: Direction::Up,
            }),
            "D" => Ok(Move {
                n: n.parse()?,
                dir: Direction::Down,
            }),
            "L" => Ok(Move {
                n: n.parse()?,
                dir: Direction::Left,
            }),
            "R" => Ok(Move {
                n: n.parse()?,
                dir: Direction::Right,
            }),
            _ => Err(ParseMoveError::InvalidDirection(d.into())),
        }
    }
}

// State of the rope after a Move
#[derive(PartialEq, Debug)]
enum State {
    Adjacent,
    CardinalGap,
    DiagonalGap,
}

#[derive(Debug, PartialEq)]
struct StateError;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Knot(i32, i32);

impl Knot {
    // Euclidean distance
    fn distance(&self, k: &Knot) -> f32 {
        (((self.0 - k.0) as f32).powf(2.0) + ((self.1 - k.1) as f32).powf(2.0)).sqrt()
    }

    // Step the knot in a given direction
    fn step(self, dir: &Direction) -> Self {
        match dir {
            Direction::Up => Knot(self.0, self.1 + 1),
            Direction::Down => Knot(self.0, self.1 - 1),
            Direction::Left => Knot(self.0 - 1, self.1),
            Direction::Right => Knot(self.0 + 1, self.1),
        }
    }
}

#[derive(Debug)]
struct Rope {
    head: Knot,
    tail: Knot,
    // Store Tail positions
    tset: HashSet<Knot>,
}

impl Rope {
    fn new() -> Self {
        Rope {
            head: Knot(0, 0),
            tail: Knot(0, 0),
            tset: HashSet::new(),
        }
    }
    // Determine what state the rope is in
    fn get_state(&self) -> Result<State, StateError> {
        match self.head.distance(&self.tail) {
            d if d < 2.0 => Ok(State::Adjacent),
            d if d == 2.0 => Ok(State::CardinalGap),
            d if d > 2.0 => Ok(State::DiagonalGap),
            _ => Err(StateError),
        }
    }
    // Move rope given an input such as `R 4`
    fn move_rope(&mut self, m: &Move) {
        for _ in 0..m.n {
            self.head = self.head.step(&m.dir);
            match self.get_state().unwrap() {
                State::Adjacent => {}
                State::CardinalGap => self.move_tail_cardinal(),
                State::DiagonalGap => self.move_tail_diagonal(),
            }
            self.tset.insert(self.tail);
        }
    }

    // Move tail cardinally adjacent to head
    // Ex: T = (0, 0), H = (2, 0)
    fn move_tail_cardinal(&mut self) {
        if self.head.0 == self.tail.0 {
            self.tail.1 = (self.tail.1 + self.head.1) / 2;
        } else {
            self.tail.0 = (self.tail.0 + self.head.0) / 2;
        }
    }

    // Move tail diagonally adjacent to head
    // Ex: T = (0, 0), H = (1, 2), newT = (1, 1)
    fn move_tail_diagonal(&mut self) {
        if (self.tail.0 - self.head.0).abs() == 1 {
            self.tail.0 = self.head.0;
            self.tail.1 = (self.tail.1 + self.head.1) / 2;
        } else {
            self.tail.1 = self.head.1;
            self.tail.0 = (self.tail.0 + self.head.0) / 2;
        }
    }

    // Knots traverse the Rope based on an input list
    fn traverse(mut self, path: &str) -> Result<Self, ParseMoveError> {
        for line in get_lines(path).map(|x| x.unwrap()).collect::<Vec<String>>() {
            self.move_rope(&Move::from_str(&line)?)
        }
        Ok(self)
    }
}

fn main() {
    let Input(path) = Input::process();
    let rope = Rope::new().traverse(&path).expect("Failed traversing rope");
    println!(
        "--- Rope ---\nHead: {:?}\nTail: {:?}\nNumtail: {}",
        rope.head,
        rope.tail,
        rope.tset.len()
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    fn test_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.txt")
    }

    #[test]
    fn test_rope_traverse() {
        let r = Rope::new().traverse(test_path().to_str().unwrap()).unwrap();
        println!("{:?}", r);
    }

    #[test]
    fn test_rope_move() {
        let mut r = Rope::new();
        r.move_rope(&Move {
            n: 5,
            dir: Direction::Right,
        });
        println!("{:?}", r);
    }

    #[test]
    fn test_rope_get_state() {
        let mut r = Rope::new();
        assert_eq!(r.get_state(), Ok(State::Adjacent));
        r.head = Knot(2, 0);
        assert_eq!(r.get_state(), Ok(State::CardinalGap));
        r.head = Knot(2, 1);
        assert_eq!(r.get_state(), Ok(State::DiagonalGap));
    }

    #[test]
    fn test_knot_distance() {
        let k0 = Knot(0, 0);
        let ka1 = Knot(1, 0);
        let ka2 = Knot(-1, -1);
        let kna1 = Knot(2, 0);
        let knad1 = Knot(1, -2);
        assert_eq!(k0.distance(&k0), 0.0);
        assert_eq!(k0.distance(&ka1), 1.0);
        assert_eq!(k0.distance(&ka2).round(), 1.0);
        assert_eq!(k0.distance(&kna1), 2.0);
        assert_eq!(k0.distance(&knad1).round(), 2.0);
    }

    #[test]
    fn test_coord_parse() {
        assert_eq!(
            Move::from_str("R 4"),
            Ok(Move {
                n: 4,
                dir: Direction::Right
            })
        );
    }
}
