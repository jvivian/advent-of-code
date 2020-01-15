// See README for instructions

use std::fs::File;
use std::io::stdin;
use std::io::{BufRead, BufReader};
use std::mem::replace;

// Struct to hold each Instruction
#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    m1: Mode,
    m2: Mode,
    m3: Mode,
}

impl From<i32> for Instruction {
    fn from(item: i32) -> Instruction {
        let item = pad_instruction(&item);
        assert_eq!(5, item.len());
        Instruction {
            opcode: item[3..].into(),
            m1: item[2..3].into(),
            m2: item[1..2].into(),
            m3: item[0..1].into(),
        }
    }
}

// Defines the operation code / task to perform
#[derive(Debug)]
enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    Halt,
}

impl From<&str> for Opcode {
    fn from(item: &str) -> Opcode {
        use Opcode::*;

        let item = item.parse::<i32>().expect("Failed to parse integer");
        match item {
            1 => Add,
            2 => Multiply,
            3 => Input,
            4 => Output,
            99 => Halt,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Mode {
    Parameter,
    Immediate,
}

impl From<&str> for Mode {
    fn from(item: &str) -> Mode {
        let item = item.parse::<i32>().expect("Failed to parse integer");
        match item {
            0 => Mode::Parameter,
            1 => Mode::Immediate,
            _ => unreachable!(),
        }
    }
}

// Takes in Instruction as integer and outputs padded String (which we need to parse)
fn pad_instruction(instruction: &i32) -> String {
    let mut new = instruction.to_string();
    let l = new.len();
    for _ in l..5 {
        new.insert(0, '0');
    }
    new.to_string()
}

// Parse provided input file
fn parse_input(path: &str) -> Vec<i32> {
    let f = File::open(path).expect("Failed to open file");
    let mut f = BufReader::new(f);
    let mut input = String::new();
    f.read_line(&mut input).expect("Failed to read line");
    input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect()
}

// Given an array, mode, and index, return the appropriate value
fn fetch(arr: &mut Vec<i32>, mode: &Mode, i: usize) -> i32 {
    match mode {
        Mode::Parameter => arr[arr[i] as usize],
        Mode::Immediate => arr[i],
    }
}

fn main() {
    let mut arr = parse_input("src/input.txt");

    let a: Instruction = 1.into();
    println!("Input: {:?} ", a);

    let mut i = 0;
    loop {
        let ins: Instruction = arr[i].into();
        match ins.opcode {
            Opcode::Add => {
                let v1 = fetch(&mut arr, &ins.m1, &i + 1);
                let v2 = fetch(&mut arr, &ins.m2, &i + 2);
                let store = arr[&i + 3];
                replace(&mut arr[store as usize], v1 + v2);
                i += 4;
            }
            Opcode::Multiply => {
                let v1 = fetch(&mut arr, &ins.m1, &i + 1);
                let v2 = fetch(&mut arr, &ins.m2, &i + 2);
                let store = arr[&i + 3];
                replace(&mut arr[store as usize], v1 * v2);
                i += 4;
            }
            Opcode::Input => {
                println!("Please provide input: ");
                let mut input = String::new();
                stdin().read_line(&mut input).expect("Failed to read stdin");
                let input = input
                    .trim()
                    .parse()
                    .expect("Failed to parse input as integer");
                let store = arr[i + 1];
                replace(&mut arr[store as usize], input);
                i += 2;
            }
            Opcode::Output => {
                let address = arr[i + 1];
                println!("Output: {}", arr[address as usize]);
                i += 2;
            }
            Opcode::Halt => {
                println!("Halting program");
                break;
            }
        }
    }
}
