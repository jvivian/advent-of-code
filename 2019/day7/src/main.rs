use day7::{parse_input, run};
use itertools::Itertools;

fn main() {
    let arr = parse_input("data/input.txt");
    let mut largest = 0;
    for phase in (0..9).permutations(5) {
        let mut new_arr = arr.clone();
        let mut input = 0;
        for p in &phase {
            let mut vec = vec![input, p.clone()];
            input = run(&mut vec, &mut new_arr).expect("Failed to collect output")
        }
        if input > largest {
            println!("New largest signal found: {} for phase {:?}", input, &phase);
            largest = input;
        }
    }
    println!("The largest signal to the thrusters is: {}", largest);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase() {
        let mut arr = parse_input("data/input-test.txt");
        let phase = [4, 3, 2, 1, 0];
        let mut input = 0;
        for p in phase.iter() {
            let mut vec = vec![input, p.clone()];
            input = run(&mut vec, &mut arr).expect("Failed to collect output");
            println!("Input: {}", &input);
        }
        assert_eq!(input, 43210);
    }
}
