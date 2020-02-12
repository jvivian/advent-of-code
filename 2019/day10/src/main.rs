// Strategy: For each asteroid, compute the slope and intercept for every other asteroid
// Store a HashSet of the slope/intercept. Asteroids on the same line will have the same slope/intercept
// and proxy as "blocking" the view of one behind it.
// The asteroid with the largest hashset is the winner

use itertools::Itertools;
use num::integer::gcd;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Clone)]
struct Asteroid {
    x: i32,
    y: i32,
}

// Stores delta between two asteroids for x/y
#[derive(Debug, Hash, Eq, PartialEq)]
struct Delta {
    x: i32,
    y: i32,
}

// Converts input file to vector of asteroids
fn parse_input(path: &str) -> Vec<Asteroid> {
    let f = File::open(path).expect("Failed to open file");
    let mut f = BufReader::new(f);
    let mut input = String::new();
    let mut vec = vec![];
    for (y, line) in f.lines().into_iter().enumerate() {
        for (x, element) in line.expect("Failed to parse line").chars().enumerate() {
            if element == '#' {
                vec.push(Asteroid {
                    x: x as i32,
                    y: y as i32,
                })
            }
        }
    }
    vec
}

// Distance between two asteroids
fn distance(a1: &Asteroid, a2: &Asteroid) -> f32 {
    (((a2.x - a1.x).pow(2) + (a2.y - a1.y).pow(2)) as f32).sqrt()
}

// Compute a map of all deltas for a given asteroid
fn delta_map(a1: &Asteroid, asteroids: &Vec<Asteroid>) -> HashMap<Delta, Vec<Asteroid>> {
    let mut map = HashMap::new();
    for a2 in asteroids {
        if a1 == a2 {
            continue;
        }
        let mut x = a1.x - a2.x;
        let mut y = a1.y - a2.y;
        let gcd_val = gcd(x, y);
        x = x / gcd_val;
        y = y / gcd_val;
        let delta = Delta { x, y };
        let vec = map.entry(delta).or_insert(vec![]);
        vec.push(a2.clone());
    }
    map
}

fn find_monitoring(asteroids: &Vec<Asteroid>) -> Asteroid {
    let mut station = Asteroid { x: -1, y: -1 };
    let mut most_los = 0;
    for a1 in asteroids {
        let map = delta_map(a1, asteroids);
        let current_count = map.len();
        if current_count > most_los {
            most_los = current_count;
            station = a1.clone();
        }
    }
    println!("Best asteroid can see {} other asteroids", most_los);
    station
}

fn main() {
    let asteroids = parse_input("data/input.txt");
    let station = find_monitoring(&asteroids);
    println!("Asteroid: {:?}", station);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let vec = parse_input("data/input-t1.txt");
        assert_eq!(vec[0], Asteroid { x: 6, y: 0 });
        assert_eq!(vec[vec.len() - 1], Asteroid { x: 9, y: 9 })
    }

    #[test]
    fn test_distance() {
        let a1 = Asteroid { x: 1, y: 1 };
        let a2 = Asteroid { x: 3, y: 1 };
        let a3 = Asteroid { x: 1, y: 2 };
        assert_eq!(2.0, distance(&a1, &a2));
        assert_eq!(1.0, distance(&a1, &a3));
    }

    #[test]
    fn test_delta_map() {
        let asteroids = parse_input("data/input-t2.txt");
        let a1 = Asteroid { x: 3, y: 4 };
        let map = delta_map(&a1, &asteroids);
        assert_eq!(map.len(), 8);
    }

    #[test]
    fn test_asteroid_count() {
        let asteroids = parse_input("data/input-t2.txt");
        let a = find_monitoring(&asteroids);
        assert_eq!(a, Asteroid { x: 3, y: 4 });
    }
}
