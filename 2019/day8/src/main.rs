// Part 1 The image you received is 25 pixels wide and 6 pixels tall.

// To make sure the image wasn't corrupted during transmission, the Elves would like
// you to find the layer that contains the fewest 0 digits. On that layer, what is the
// number of 1 digits multiplied by the number of 2 digits?

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

fn parse_input(path: &str) -> Vec<i32> {
    let f = File::open(path).expect("Failed to open file");
    let mut f = BufReader::new(f);
    let mut input = String::new();
    f.read_line(&mut input).expect("Failed to read line");
    input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).expect("Failed to parse digit") as i32)
        .collect()
}

// Converts to a vector of layers, where a layer represents a 2d coordinate and value
fn convert_to_layers(pixels: Vec<i32>) -> Vec<HashMap<Coord, i32>> {
    let mut layers = vec![];
    let mut x = 0;
    let mut y = 0;
    let mut map = HashMap::new();
    for (i, p) in pixels.iter().enumerate() {
        map.insert(Coord { x, y }, p.clone());
        y += 1;
        if y % 25 == 0 {
            x += 1;
            y = 1;
        }
        if (i + 1) % 150 == 0 {
            layers.push(map.clone());
            map = HashMap::new();
            x = 0;
            y = 0;
        }
    }
    layers
}

// Return index of layer which has the fewest zeros
fn get_fewest_zeros(layers: &Vec<HashMap<Coord, i32>>) -> usize {
    let mut count = 150;
    let mut index = 0;
    for (i, layer) in layers.iter().enumerate() {
        let n = layer
            .values()
            .filter(|&x| *x == 0)
            .collect::<Vec<_>>()
            .len();
        if n < count {
            count = n;
            index = i;
        }
    }
    println!("Fewest Zeros: {} at index {}", count, &index);
    index
}

fn count_n(layer: &HashMap<Coord, i32>, n: i32) -> usize {
    layer
        .values()
        .filter(|&x| *x == n)
        .collect::<Vec<_>>()
        .len()
}

fn main() {
    // Get pixel stream
    let pixels = parse_input("data/input.txt");
    let layers = convert_to_layers(pixels);
    let i = get_fewest_zeros(&layers);
    let total = count_n(&layers[i], 1) * count_n(&layers[i], 2);
    println!(
        "Number of 1s multiplied by number of 2s in layer {} is {}",
        i, total
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer() {
        let pixels = parse_input("data/input.txt");
        let layers = convert_to_layers(pixels);
        assert_eq!(100, layers.len());
        for layer in layers {
            assert_eq!(150, layer.len());
        }
    }
}
