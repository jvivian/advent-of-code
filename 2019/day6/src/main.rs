// See README.md for details

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn parse_input(path: &str) -> Result<HashMap<String, String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut map = HashMap::new();
    for line in reader.lines() {
        let split: Vec<String> = line.unwrap().split(')').map(String::from).collect();
        map.insert(split[1].to_string(), split[0].to_string());
    }
    Ok(map)
}

// Calculate the total orbits for a key by traversing graph back to COM
fn calculate_orbits(key: String, map: HashMap<String, String>) -> i32 {
    match key.as_str() {
         "COM" => return 0,
        _ => return 1 + calculate_orbits(map[&key].clone(), map),
    }
}

fn main() {
    let map = parse_input("data/input.txt").unwrap();
    let mut total = 0;
    for val in map.keys() {
        total += calculate_orbits(val.clone(), map.clone())
    }
    println!("Total number of orbits: {}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let map = parse_input("data/input-p1-test.txt").unwrap();
        assert!(map.contains_key("B"))
    }

    #[test]
    fn test_calculate_orbits() {
        let map = parse_input("data/input-p1-test.txt").unwrap();
        let count = calculate_orbits("L".to_string(), map);
        assert_eq!(count, 7);
    }
}
