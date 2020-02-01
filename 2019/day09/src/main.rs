use day9::{run, Input};

fn main() {
    // Part 1
    let mut input = Input::new("data/input.txt", Some(1));
    run(&mut input);

    // Part 2
    let mut input = Input::new("data/input.txt", Some(2));
    run(&mut input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_adjust() {
        let mut input = Input::new("data/input-t1.txt", None);
        run(&mut input);
    }

    #[test]
    fn test_big_number() {
        let mut input = Input::new("data/input-t2.txt", None);
        run(&mut input);
    }

    #[test]
    fn test_big_number_2() {
        let mut input = Input::new("data/input-t3.txt", None);
        run(&mut input);
    }

    #[test]
    fn test_rel() {
        let mut input = Input::new("data/input-t4.txt", None);
        run(&mut input);
    }
}
