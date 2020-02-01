use day7::{initalize_amplifiers, parse_input, run, Amplifier};
use itertools::Itertools;

fn main() {
    // Part 1
    let mut largest = 0;
    for phases in (0..10).permutations(5) {
        let amps = initalize_amplifiers(phases, "data/input.txt");
        let mut input = 0;
        for mut amp in amps {
            amp.input = Some(input);
            input = run(&mut amp).expect("Failed to get input from amp");
            if input > largest {
                largest = input;
            }
        }
    }
    println!("The largest signal to the thrusters is: {}", largest);

    // Part 2
    let mut largest = 0;
    for phases in (5..10).permutations(5) {
        let mut amps = initalize_amplifiers(phases, "data/input.txt");
        let mut input = 0;
        let mut i = 0;
        loop {
            let mut amp = &mut amps[i % 5];
            amp.input = Some(input);
            match run(&mut amp) {
                Some(new_input) => input = new_input,
                None => break,
            }
            i += 1;
            if input > largest {
                largest = input;
            }
        }
    }
    println!("The largest signal to the thrusters is: {}", largest);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase() {
        let phases = vec![4, 3, 2, 1, 0];
        let amps = initalize_amplifiers(phases, "data/input-test.txt");
        let mut input = 0;
        for mut amp in amps {
            amp.input = Some(input);
            input = run(&mut amp).expect("Failed to get input from amp");
        }
        assert_eq!(input, 43210);
    }

    #[test]
    fn test_feedback_loop() {
        let phases = vec![9, 8, 7, 6, 5];
        let mut amps = initalize_amplifiers(phases, "data/input-test-p2.txt");
        let mut amp_input = 0;
        let mut i = 0;
        loop {
            let mut amp = &mut amps[i % 5];
            amp.input = Some(amp_input);
            match run(&mut amp) {
                Some(input) => amp_input = input,
                None => break,
            }
            i += 1;
        }
        assert_eq!(139629729, amp_input);
    }
}
