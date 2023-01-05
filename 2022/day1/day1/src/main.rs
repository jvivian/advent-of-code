use std::fs::File;
use std::io::{self, BufRead};
use type_cli::CLI;

#[derive(CLI)]
struct Calories(String);

fn main() {
    let Calories(input_path) = Calories::process();
    let file = File::open(&input_path).expect("Failed to open file");
    let lines = io::BufReader::new(file).lines();
    let mut calories: Vec<i32> = vec![];
    let mut current = vec![];
    for line in lines {
        if let Ok(x) = line {
            if x.is_empty() {
                calories.push(current.iter().sum());
                current.clear();
            } else {
                current.push(x.parse::<i32>().unwrap());
            }
        }
    }
    calories.sort();
    calories.reverse();
    let top_3: i32 = calories[..3].iter().sum();

    println!("The chumbos were {:?}", &calories[..3]);
    println!("Their total calorie total was {}", top_3);
}
