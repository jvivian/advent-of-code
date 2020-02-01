// See README for instructions

use std::fs::File;
use std::io::{BufRead, BufReader};

// Represents an Instruction provided by the IntCode computer
#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    m1: Mode,
    m2: Mode,
    m3: Mode,
}

impl From<i64> for Instruction {
    fn from(item: i64) -> Instruction {
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

// Represents an Input to the IntCode computer
// `arr` is the instruction/memory vector
// i is the instruction pointer
// offset is the offset used in "Relative" mode
// input is some input integer value
#[derive(Debug)]
pub struct Input {
    pub arr: Vec<i64>,
    pub i: usize,
    pub offset: i64,
    pub input: Option<i64>,
}

impl Input {
    pub fn new(arr_path: &str, input: Option<i64>) -> Input {
        Input {
            arr: parse_input(arr_path),
            i: 0,
            offset: 0,
            input,
        }
    }
}

// Enum for all possible Opcodes the IntCode computer can handle
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
    RelativeAdjust,
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
            9 => RelativeAdjust,
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
    Relative,
}

impl From<&str> for Mode {
    fn from(item: &str) -> Mode {
        let item = item.parse::<i32>().expect("Failed to parse integer");
        match item {
            0 => Mode::Parameter,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => unreachable!(),
        }
    }
}

// Takes in Instruction as integer and outputs padded String (which we need to parse)
fn pad_instruction(instruction: &i64) -> String {
    let mut new = instruction.to_string();
    let l = new.len();
    for _ in l..5 {
        new.insert(0, '0');
    }
    new.to_string()
}

// Fetches a value from our memory/instruction vector
fn get(arr: &mut Vec<i64>, mode: &Mode, i: usize, offset: &i64) -> i64 {
    match mode {
        Mode::Parameter => arr[arr[i] as usize],
        Mode::Immediate => arr[i],
        Mode::Relative => {
            let address = arr[i] + offset;
            arr[address as usize]
        }
    }
}

// Sets a value in our memory/instruction vector
fn set(arr: &mut Vec<i64>, mode: &Mode, i: usize, offset: &i64, val: i64) {
    match mode {
        Mode::Relative => {
            let address = &arr[i] + offset;
            arr[address as usize] = val;
        }
        _ => {
            let address = arr[i];
            arr[address as usize] = val
        }
    };
}

// Parse input file
pub fn parse_input(path: &str) -> Vec<i64> {
    let f = File::open(path).expect("Failed to open file");
    let mut f = BufReader::new(f);
    let mut input = String::new();
    f.read_line(&mut input).expect("Failed to read line");
    let mut input: Vec<i64> = input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    // Pad vector with zeros for "increased memory"
    if input.len() < 100 {
        input.extend(vec![0; 100]);
    } else {
        input.extend(vec![0; input.len() * 3]);
    }
    input
}

// Run the IntCode computer
pub fn run(inp: &mut Input) -> Option<i32> {
    loop {
        let ins: Instruction = inp.arr[inp.i].into();
        match ins.opcode {
            Opcode::Add => {
                let v1 = get(&mut inp.arr, &ins.m1, &inp.i + 1, &inp.offset);
                let v2 = get(&mut inp.arr, &ins.m2, &inp.i + 2, &inp.offset);
                set(&mut inp.arr, &ins.m3, &inp.i + 3, &inp.offset, v1 + v2);
                inp.i += 4;
            }
            Opcode::Multiply => {
                let v1 = get(&mut inp.arr, &ins.m1, &inp.i + 1, &inp.offset);
                let v2 = get(&mut inp.arr, &ins.m2, &inp.i + 2, &inp.offset);
                set(&mut inp.arr, &ins.m3, &inp.i + 3, &inp.offset, v1 * v2);
                inp.i += 4;
            }
            // If input is requested and there isn't any, something went wrong
            Opcode::Input => {
                if let Some(input) = inp.input {
                    set(&mut inp.arr, &ins.m1, &inp.i + 1, &inp.offset, input);
                    inp.input = None
                } else {
                    unreachable!(
                        "Hey! You're not supposed to be here! Returned NONE from an input opcode"
                    );
                }
                inp.i += 2;
            }
            Opcode::Output => {
                let val = get(&mut inp.arr, &ins.m1, &inp.i + 1, &inp.offset);
                inp.i += 2;
                // return Some(val);
                println!("{}", val);
            }
            Opcode::JumpTrue => {
                let v1 = get(&mut inp.arr, &ins.m1, &inp.i + 1, &inp.offset);
                let v2 = get(&mut inp.arr, &ins.m2, &inp.i + 2, &inp.offset);
                match v1 {
                    0 => inp.i += 3,
                    _ => inp.i = v2 as usize,
                }
            }
            Opcode::JumpFalse => {
                let v1 = get(&mut inp.arr, &ins.m1, &inp.i + 1, &inp.offset);
                let v2 = get(&mut inp.arr, &ins.m2, &inp.i + 2, &inp.offset);
                match v1 {
                    0 => inp.i = v2 as usize,
                    _ => inp.i += 3,
                }
            }
            Opcode::LessThan => {
                let v1 = get(&mut inp.arr, &ins.m1, &inp.i + 1, &inp.offset);
                let v2 = get(&mut inp.arr, &ins.m2, &inp.i + 2, &inp.offset);
                let mut val = 0;
                if v1 < v2 {
                    val = 1
                }
                set(&mut inp.arr, &ins.m3, &inp.i + 3, &inp.offset, val);
                inp.i += 4;
            }
            Opcode::Equals => {
                let v1 = get(&mut inp.arr, &ins.m1, &inp.i + 1, &inp.offset);
                let v2 = get(&mut inp.arr, &ins.m2, &inp.i + 2, &inp.offset);
                let mut val = 0;
                if v1 == v2 {
                    val = 1
                }
                set(&mut inp.arr, &ins.m3, &inp.i + 3, &inp.offset, val);
                inp.i += 4;
            }
            Opcode::RelativeAdjust => {
                let v1 = get(&mut inp.arr, &ins.m1, &inp.i + 1, &inp.offset);
                inp.offset += v1;
                inp.i += 2;
            }
            Opcode::Halt => {
                println!("Halting!");
                break;
            }
        }
    }
    None
}
