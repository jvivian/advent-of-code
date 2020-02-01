// The wires twist and turn, but the two wires occasionally cross paths. To fix the circuit,
// you need to find the intersection point closest to the central port. Because the wires
// are on a grid, use the Manhattan distance for this measurement. While the wires do technically
// cross right at the central port where they both start, this point does not count, nor does a
// wire count as crossing with itself.
// For example, if the first wire's path is R8,U5,L5,D3, then starting from the central port (o),
// it goes right 8, up 5, left 5, and finally down 3.

// P1: What is the Manhattan distance from the central port to the closest intersection?

// P2: To do this, calculate the number of steps each wire takes to reach each intersection;
// choose the intersection where the sum of both wires' steps is lowest
use std::collections::{HashMap, HashSet};
use std::io;

// 2d Coordinate struct to use as key in HashMap
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

// Manhattan distance is the absolute value summed
impl Coord {
    fn manhattan_distance(self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

// Enum for direction (up, down, left, right) and distance
#[derive(Debug)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

// Impl 'from' conversion for strings to Direction
impl From<&str> for Direction {
    fn from(item: &str) -> Direction {
        use Direction::*;

        let code = item
            .as_bytes()
            .iter()
            .next()
            .expect("Failed to grab direction from input");

        let val = item[1..].parse::<i32>().expect("Failed to parse integer");

        match code {
            b'U' => Up(val),
            b'D' => Down(val),
            b'R' => Right(val),
            b'L' => Left(val),
            _ => unreachable!(),
        }
    }
}

fn main() {
    // read in coords
    let stdin = io::stdin();
    let mut wire1 = String::new();
    let mut wire2 = String::new();
    stdin.read_line(&mut wire1).expect("Failed to read wire1");
    stdin.read_line(&mut wire2).expect("Failed to read wire2");

    // Convert to a vector of strings
    let wire1: Vec<&str> = wire1.trim().split(',').collect();
    let wire2: Vec<&str> = wire2.trim().split(',').collect();

    // Get set of all wire coordinates in path
    let map1 = parse_wire(wire1);
    let map2 = parse_wire(wire2);

    // Get intersection
    let set1: HashSet<Coord> = map1.keys().cloned().collect();
    let set2: HashSet<Coord> = map2.keys().cloned().collect();

    // Get minimum distance and steps
    let min_dist = set1
        .intersection(&set2)
        .map(|x| x.manhattan_distance())
        .min()
        .unwrap();

    // Get minimum number of steps
    let min_steps = set1
        .intersection(&set2)
        .map(|x| map1[x] + map2[x])
        .min()
        .unwrap();

    println!(
        "Minimum Manhattan dist: {}\tMinimum steps: {}",
        min_dist, min_steps
    );
}

// Parse each wire by adding its Coords to a HashSet
// Range iteration works ONE WAY (smaller to larger)
fn parse_wire(wire: Vec<&str>) -> HashMap<Coord, u32> {
    // HashMap to store steps, Coord to represent current position, and step counter
    let mut map = HashMap::new();
    let mut current = Coord { x: 0, y: 0 };
    let mut steps: u32 = 0;

    // Parse input and update wire position
    for entry in wire {
        match entry.into() {
            Direction::Up(val) => {
                for y in current.y + 1..current.y + val + 1 {
                    let c = Coord { x: current.x, y };
                    wire_step(c, &mut steps, &mut map);
                }
                current.y = current.y + val;
            }
            Direction::Down(val) => {
                for y in current.y - val..current.y {
                    let c = Coord { x: current.x, y };
                    wire_step(c, &mut steps, &mut map);
                }
                current.y = current.y - val;
            }
            Direction::Right(val) => {
                for x in current.x + 1..current.x + val + 1 {
                    let c = Coord { x, y: current.y };
                    wire_step(c, &mut steps, &mut map);
                }
                current.x = current.x + val;
            }
            Direction::Left(val) => {
                for x in current.x - val..current.x {
                    let c = Coord { x, y: current.y };
                    wire_step(c, &mut steps, &mut map);
                }
                current.x = current.x - val;
            }
        };
    }
    map.remove(&Coord { x: 0, y: 0 });
    map
}

// Update a HashMap counter of the wire
fn wire_step(c: Coord, steps: &mut u32, map: &mut HashMap<Coord, u32>) -> () {
    *steps += 1;
    if !map.contains_key(&c) {
        map.insert(c, *steps);
    }
}
