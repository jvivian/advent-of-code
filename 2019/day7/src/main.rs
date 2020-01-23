use day7::{parse_input, run};
use itertools::Itertools;

fn main() {
    let arr = parse_input("data/input.txt");
    let mut largest = 0;
    for phase in (0..10).permutations(5) {
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
        }
        assert_eq!(input, 43210);
    }

    #[test]
    fn test_feedback_loop() {
        let arr = parse_input("data/input-test-p2.txt");
        let mut new_arr = arr.clone();
        let phase = [9,8,7,6,5];
        let mut input = 0;
        let mut i = 0;
        loop {
//            let mut new_arr = arr.clone();
            println!("Iteration: {}\tInput: {}\tPhase: {}", &i, &input, &phase[i % 5]);
            let mut vec = vec![input, phase[i % 5]];
            match run(&mut vec, &mut new_arr) {
                Some(output) => input = output,
                None => break
            }
            i += 1;
            if input > 139629729 { break }
        }
        println!("Final Output {}", input);
        assert_eq!(139629729, input)
    }
}
