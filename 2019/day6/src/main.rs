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
fn calculate_orbits(start: &str, stop: &str, map: &HashMap<String, String>) -> i32 {
    //    match start {
    //        stop => return 0,
    //        _ => return 1 + calculate_orbits(&map[start], stop, &map),
    //    }
    if start == stop {
        return 0;
    } else {
        return 1 + calculate_orbits(&map[start], stop, &map);
    }
}

// Returns a vector or the "path" back to "COM" for a key
fn get_path(key: &str, map: &HashMap<String, String>) -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    let mut key = &map[key];
    while key != "COM" {
        vec.push(key.to_string());
        key = &map[key];
    }
    vec
}

// Given two paths calculate the closet node, which will be the first occurrence of an
// element in one list existing in the other
fn get_closest_node(path1: &Vec<String>, path2: &Vec<String>) -> Option<String> {
    for p in path1.iter() {
        if path2.contains(p) {
            return Some(p.to_string());
        }
    }
    None
}

fn main() {
    let map = parse_input("data/input.txt").unwrap();
    let mut total = 0;
    for val in map.keys() {
        total += calculate_orbits(val, "COM", &map)
    }
    println!("Total number of orbits: {}", total);

    // Calculate total transfers between YOU and SAN
    let you_path = get_path("YOU", &map);
    let san_path = get_path("SAN", &map);
    let closest_node = get_closest_node(&you_path, &san_path).unwrap();
    let you_steps = calculate_orbits("YOU", &closest_node, &map) - 1;
    let san_steps = calculate_orbits("SAN", &closest_node, &map) - 1;
    println!(
        "Number of steps between YOU and SAN is: {}",
        you_steps + san_steps
    );
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
        let count = calculate_orbits("L", "COM", &map);
        assert_eq!(count, 7);
    }

    #[test]
    fn test_get_path() {
        let map = parse_input("data/input-p1-test.txt").unwrap();
        let vec = get_path("D", &map);
        assert_eq!(vec!["C", "B"], vec)
    }

    #[test]
    fn test_get_closest_node() {
        let map = parse_input("data/input-p1-test.txt").unwrap();
        let fpath = get_path("F", &map);
        let lpath = get_path("L", &map);
        assert_eq!("E", get_closest_node(&fpath, &lpath).unwrap())
    }
}
