use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;
use type_cli::CLI;

#[derive(CLI)]
struct Input(String);

// Throws
enum Throw {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct ParseThrowError;

impl FromStr for Throw {
    type Err = ParseThrowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Throw::Rock),
            // "X" => Ok(Throw::Rock),
            "B" => Ok(Throw::Paper),
            // "Y" => Ok(Throw::Paper),
            "C" => Ok(Throw::Scissors),
            // "Z" => Ok(Throw::Scissors),
            _ => Err(ParseThrowError),
        }
    }
}

#[derive(Debug)]
struct ParseThrowResultError;

impl FromStr for ThrowResult {
    type Err = ParseThrowResultError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(ThrowResult::Lose),
            "Y" => Ok(ThrowResult::Draw),
            "Z" => Ok(ThrowResult::Win),
            _ => Err(ParseThrowResultError),
        }
    }
}

enum ThrowResult {
    Lose,
    Draw,
    Win,
}

fn score_result(r: &ThrowResult) -> i32 {
    match r {
        ThrowResult::Lose => 0,
        ThrowResult::Draw => 3,
        ThrowResult::Win => 6,
    }
}

fn score_throw(throw: &Throw) -> i32 {
    match throw {
        Throw::Rock => 1,
        Throw::Paper => 2,
        Throw::Scissors => 3,
    }
}

fn result_from_throws(me: &Throw, op: &Throw) -> ThrowResult {
    match me {
        Throw::Rock => match op {
            Throw::Rock => ThrowResult::Draw,
            Throw::Paper => ThrowResult::Lose,
            Throw::Scissors => ThrowResult::Win,
        },
        Throw::Paper => match op {
            Throw::Rock => ThrowResult::Win,
            Throw::Paper => ThrowResult::Draw,
            Throw::Scissors => ThrowResult::Lose,
        },
        Throw::Scissors => match op {
            Throw::Rock => ThrowResult::Lose,
            Throw::Paper => ThrowResult::Win,
            Throw::Scissors => ThrowResult::Draw,
        },
    }
}

fn throw_from_result_throw(op: &Throw, res: &ThrowResult) -> Throw {
    match res {
        ThrowResult::Lose => match op {
            Throw::Rock => Throw::Scissors,
            Throw::Paper => Throw::Rock,
            Throw::Scissors => Throw::Paper,
        },
        ThrowResult::Draw => match op {
            Throw::Rock => Throw::Rock,
            Throw::Paper => Throw::Paper,
            Throw::Scissors => Throw::Scissors,
        },
        ThrowResult::Win => match op {
            Throw::Rock => Throw::Paper,
            Throw::Paper => Throw::Scissors,
            Throw::Scissors => Throw::Rock,
        },
    }
}

fn score_from_throws(me: &Throw, op: &Throw) -> i32 {
    let result = result_from_throws(me, op);
    score_throw(me) + score_result(&result)
}

fn main() {
    let Input(path) = Input::process();
    let file = File::open(&path).expect("Failed to open file");
    let lines = io::BufReader::new(file).lines();
    let mut score = 0;
    for line in lines {
        if let Ok(x) = line {
            let input: Vec<&str> = x.split(" ").collect();
            let op: Throw = input[0].parse().unwrap();
            let res: ThrowResult = input[1].parse().unwrap();
            let throw = throw_from_result_throw(&op, &res);
            score += score_throw(&throw) + score_result(&res);

            // Part 1 code
            // .iter()
            // .map(|&x| x.parse::<Throw>().unwrap())
            // .collect();
            // let test: Vec<Throw> = foo.iter().map(|&x| x.parse::<Throw>().unwrap()).collect();
            // score += score_from_throws(&test[1], &test[0]);
        }
    }
    println!("The final score achieved was {}", score);
}
