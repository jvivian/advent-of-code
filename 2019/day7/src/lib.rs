// See README for instructions

use std::fs::File;
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

// Amplifier representation. `arr` is the instruction state,
// `i` is the instruction pointer, and `input` contains inputs to `pop` as used
struct Amplifier {
    arr: Vec<i32>,
    i: i32,
    input: Vec<i32>
}

// Defines the operation code / task to perform
#[derive(Debug)]
enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    JumpTrue,
    JumpFalse,
    LessThan,
    Equals,
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
            5 => JumpTrue,
            6 => JumpFalse,
            7 => LessThan,
            8 => Equals,
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

// Given an array, mode, and index, return the appropriate value
fn fetch(arr: &Vec<i32>, mode: &Mode, i: usize) -> i32 {
    match mode {
        Mode::Parameter => arr[arr[i] as usize],
        Mode::Immediate => arr[i],
    }
}

// Parse provided input file
pub fn parse_input(path: &str) -> Vec<i32> {
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

pub fn run(input: &mut Vec<i32>, arr: &mut Vec<i32>) -> Option<i32> {
    let mut i = 0;
    loop {
        let ins: Instruction = arr[i].into();
        match ins.opcode {
            Opcode::Add => {
                let v1 = fetch(&arr, &ins.m1, &i + 1);
                let v2 = fetch(&arr, &ins.m2, &i + 2);
                let store = arr[&i + 3] as usize;
                replace(&mut arr[store], v1 + v2);
                i += 4;
            }
            Opcode::Multiply => {
                let v1 = fetch(&arr, &ins.m1, &i + 1);
                let v2 = fetch(&arr, &ins.m2, &i + 2);
                let store = arr[&i + 3] as usize;
                replace(&mut arr[store], v1 * v2);
                i += 4;
            }
            Opcode::Input => {
                let value = input.pop().expect("Failed to pop input, likely empty");
                let store = arr[i + 1] as usize;
                replace(&mut arr[store], value);
                i += 2;
            }
            Opcode::Output => {
                let val = fetch(&arr, &ins.m1, &i + 1);
                return Some(val)
            }
            Opcode::JumpTrue => {
                let v1 = fetch(&arr, &ins.m1, &i + 1);
                let v2 = fetch(&arr, &ins.m2, &i + 2);
                match v1 {
                    0 => i += 3,
                    _ => i = v2 as usize,
                }
            }
            Opcode::JumpFalse => {
                let v1 = fetch(&arr, &ins.m1, &i + 1);
                let v2 = fetch(&arr, &ins.m2, &i + 2);
                match v1 {
                    0 => i = v2 as usize,
                    _ => i += 3,
                }
            }
            Opcode::LessThan => {
                let v1 = fetch(&arr, &ins.m1, &i + 1);
                let v2 = fetch(&arr, &ins.m2, &i + 2);
                let store = arr[&i + 3] as usize;
                match v1 < v2 {
                    true => replace(&mut arr[store], 1),
                    false => replace(&mut arr[store], 0),
                };
                i += 4;
            }
            Opcode::Equals => {
                let v1 = fetch(&arr, &ins.m1, &i + 1);
                let v2 = fetch(&arr, &ins.m2, &i + 2);
                let store = arr[&i + 3] as usize;
                match v1 == v2 {
                    true => replace(&mut arr[store], 1),
                    false => replace(&mut arr[store], 0),
                };
                i += 4;
            }
            Opcode::Halt => {
                println!("Halting program");
                break;

            }
        }
    }
    None
}
