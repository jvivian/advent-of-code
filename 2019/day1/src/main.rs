// Fuel required to launch a given module is based on its mass.
// Specifically, to find the fuel required for a module, take its mass,
// divide by three, round down, and subtract 2.
//
// What is the sum of the fuel requirements for all of the modules on your spacecraft?

use std::io::{self, Read};

fn main() -> io::Result<()> {
    println!("Reading in input file from stdin");

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    // Calculate fuel totals
    let mut first_fuel = 0;
    let mut recursive_total = 0;
    for line in buffer.lines() {
        let module: i32 = line
            .trim()
            .parse()
            .expect("Failure to convert string to integer");
        first_fuel = first_fuel + module / 3 - 2;
        recursive_total = recursive_total + recursive_fuel(module);
    }

    println!("Sum of the first fuel requirements is {}", first_fuel);
    println!("Sum of the second fuel requirements is {}", recursive_total);

    Ok(())
}

fn recursive_fuel(input: i32) -> i32 {
    // So, for each module mass, calculate its fuel and add it to the total.
    // Then, treat the fuel amount you just calculated as the input mass and
    // repeat the process, continuing until a fuel requirement is zero or negative.
    let fuel: i32 = input / 3 - 2;
    if fuel <= 0 {
        0
    } else {
        fuel + recursive_fuel(fuel)
    }
}
