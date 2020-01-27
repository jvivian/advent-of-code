// See README for instructions

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::replace;

// Holds one instruction
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
// `i` is the instruction pointer, `phase` holds the initial input phase
// and `input` contains the primary input to use after the phase signal is applied
#[derive(Debug)]
pub struct Amplifier {
    pub arr: Vec<i32>,
    pub i: usize,
    pub phase: Option<i32>,
    pub input: Option<i32>,
}

// Enum to hold the operation code to be performed
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

// Enum for monitoring which MODE a given parameter is in
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

// Creates a vector of Amplifiers given a vector of phases and an instruction path
pub fn initalize_amplifiers(phases: Vec<i32>, arr_path: &str) -> Vec<Amplifier> {
    let arr = parse_input(arr_path);
    let mut amps = vec![];
    for p in phases {
        amps.push(Amplifier {
            arr: arr.clone(),
            i: 0,
            phase: Some(p),
            input: None,
        });
    }
    amps
}

// Runs an Amplifier
pub fn run(amp: &mut Amplifier) -> Option<i32> {
    loop {
        let ins: Instruction = amp.arr[amp.i].into();
        match ins.opcode {
            Opcode::Add => {
                let v1 = fetch(&amp.arr, &ins.m1, &amp.i + 1);
                let v2 = fetch(&amp.arr, &ins.m2, &amp.i + 2);
                let store = amp.arr[&amp.i + 3] as usize;
                replace(&mut amp.arr[store], v1 + v2);
                amp.i += 4;
            }
            Opcode::Multiply => {
                let v1 = fetch(&amp.arr, &ins.m1, &amp.i + 1);
                let v2 = fetch(&amp.arr, &ins.m2, &amp.i + 2);
                let store = amp.arr[&amp.i + 3] as usize;
                replace(&mut amp.arr[store], v1 * v2);
                amp.i += 4;
            }
            // First check for phase to use as input
            // Then, check for input value
            // Finally, if no input available, return None
            Opcode::Input => {
                let store = amp.arr[&amp.i + 1] as usize;
                if let Some(phase) = amp.phase {
                    replace(&mut amp.arr[store], phase);
                    amp.phase = None
                } else if let Some(input) = amp.input {
                    replace(&mut amp.arr[store], input);
                    amp.input = None
                } else {
                    unreachable!(
                        "Hey! You're not supposed to be here! Returned NONE from an input opcode"
                    );
                }
                amp.i += 2;
            }
            Opcode::Output => {
                let val = fetch(&amp.arr, &ins.m1, &amp.i + 1);
                amp.i += 2;
                return Some(val);
            }
            Opcode::JumpTrue => {
                let v1 = fetch(&amp.arr, &ins.m1, &amp.i + 1);
                let v2 = fetch(&amp.arr, &ins.m2, &amp.i + 2);
                match v1 {
                    0 => amp.i += 3,
                    _ => amp.i = v2 as usize,
                }
            }
            Opcode::JumpFalse => {
                let v1 = fetch(&amp.arr, &ins.m1, &amp.i + 1);
                let v2 = fetch(&amp.arr, &ins.m2, &amp.i + 2);
                match v1 {
                    0 => amp.i = v2 as usize,
                    _ => amp.i += 3,
                }
            }
            Opcode::LessThan => {
                let v1 = fetch(&amp.arr, &ins.m1, &amp.i + 1);
                let v2 = fetch(&amp.arr, &ins.m2, &amp.i + 2);
                let store = amp.arr[&amp.i + 3] as usize;
                match v1 < v2 {
                    true => replace(&mut amp.arr[store], 1),
                    false => replace(&mut amp.arr[store], 0),
                };
                amp.i += 4;
            }
            Opcode::Equals => {
                let v1 = fetch(&amp.arr, &ins.m1, &amp.i + 1);
                let v2 = fetch(&amp.arr, &ins.m2, &amp.i + 2);
                let store = amp.arr[&amp.i + 3] as usize;
                match v1 == v2 {
                    true => replace(&mut amp.arr[store], 1),
                    false => replace(&mut amp.arr[store], 0),
                };
                amp.i += 4;
            }
            Opcode::Halt => {
                println!("Halting Amplifier!");
                break;
            }
        }
    }
    None
}
