// Opcode 1 adds together numbers read from two positions and stores the result in a third position.
// The three integers immediately after the opcode tell you these three positions -
// the first two indicate the positions from which you should read the input values,
// and the third indicates the position at which the output should be stored.

// To do this, before running the program, replace position 1 with the value 12 and
// replace position 2 with the value 2. What value is left at position 0 after the program halts?

// "With terminology out of the way, we're ready to proceed. To complete the gravity assist,
// you need to determine what pair of inputs produces the output 19690720."

use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    // Read input from stdin
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed to read");

    // Place into Hashmap
    let mut codes: HashMap<i32, i32> = HashMap::new();
    let input = buffer.lines().next().unwrap().trim();
    for (i, val) in input.split(',').enumerate() {
        codes.insert(i as i32, val.parse().expect("Failed to parse to integer"));
    }

    // P1: Run opcode with original instructions
    let opcode = run_opcode(&mut codes, 12, 2).expect("Failed to run opcode");
    println!("The value at position 0 is {} for inputs 12 and 2", opcode);

    // P2: Brute force opcode computer to get 19690720
    let goal = 19690720;
    for i in 1..codes.len() {
        for j in 1..codes.len() {
            let result = run_opcode(&mut codes, i as i32, j as i32).expect("Failed to get result");
            if result == goal {
                let answer = i * 100 + j;
                println!("By jove, you've done it! p1: {}\tp2: {} = {}", i, j, answer);
            } else if result > goal {
                break;
            }
        }
    }
}

fn run_opcode(input: &mut HashMap<i32, i32>, p1: i32, p2: i32) -> Option<i32> {
    // We don't want to change our input, so we make a clone to modify
    let mut codes = input.clone();
    // Modify map based on our input parameters
    codes.insert(1, p1);
    codes.insert(2, p2);

    // Opcode operation
    for key in 0..codes.len() as i32 {
        if key % 4 == 0 {
            let opcode = codes[&(key)];
            let pos1 = codes[&(key + 1)];
            let pos2 = codes[&(key + 2)];
            let x = codes[&pos1];
            let y = codes[&pos2];
            let store = codes[&(key + 3)];

            if opcode == 1 {
                codes.insert(store, x + y);
            } else if opcode == 2 {
                codes.insert(store, x * y);
            } else if opcode == 99 {
                return Some(codes[&0]);
            }
        }
    }
    None
}
