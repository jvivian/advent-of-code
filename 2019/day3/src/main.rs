// What is the Manhattan distance from the central port to the closest intersection?
use std::collections::HashSet;
use std::io;

// 2d Coordinate struct to use as key in HashMap
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

// Manhattan distance is just the absolute value summed
impl Coord {
    fn manhattan_distance(self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

// Enum for direction (up, down, left, right)
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
    let set1 = parse_wire(wire1);
    let set2 = parse_wire(wire2);

    // Get intersection coordinates between wire1 and wire2
    let inter: HashSet<_> = set1.intersection(&set2).collect();

    // Finally, calculate the minimum manhattan distance
    let min_dist = set1
        .intersection(&set2)
        .map(|x| x.manhattan_distance())
        .min()
        .unwrap();

    println!(
        "The Manhattan dist of the closest intersection is: {:?}",
        min_dist
    );
}

// Parse each wire by adding its Coords to a HashSet
// Range iteration works ONE WAY (smaller to larger)
// Ergo, Down and Left ranges are reversed (although .rev() is unnecessary here)
fn parse_wire(wire: Vec<&str>) -> HashSet<Coord> {
    let mut set = HashSet::new();
    let mut current = Coord { x: 0, y: 0 };
    for entry in wire {
        // Update HashSet with coordinates
        match entry.into() {
            Direction::Up(val) => {
                for y in current.y..current.y + val + 1 {
                    set.insert(Coord { x: current.x, y });
                }
                current.y = current.y + val;
            }
            Direction::Down(val) => {
                for y in current.y - val..current.y {
                    set.insert(Coord { x: current.x, y });
                }
                current.y = current.y - val;
            }
            Direction::Right(val) => {
                for x in current.x..current.x + val + 1 {
                    set.insert(Coord { x, y: current.y });
                }
                current.x = current.x + val;
            }
            Direction::Left(val) => {
                for x in current.x - val..current.x {
                    set.insert(Coord { x, y: current.y });
                }
                current.x = current.x - val;
            }
        };
    }
    // Central port is ignored, so remove 0,0
    set.remove(&Coord { x: 0, y: 0 });
    set
}
