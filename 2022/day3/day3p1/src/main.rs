use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use type_cli::CLI;

#[derive(CLI)]
struct Input(String);

fn get_lines(path: &String) -> Lines<BufReader<File>> {
    io::BufReader::new(File::open(path).expect("Failed to open file")).lines()
}

fn get_item_list() -> Vec<char> {
    let lowers = ('a'..='z').collect::<Vec<char>>();
    let capitals = ('A'..='Z').collect::<Vec<char>>();
    lowers.into_iter().chain(capitals.into_iter()).collect()
}

fn score_item(c: &char, items: &Vec<char>) -> usize {
    items.iter().position(|&r| r == *c).unwrap() + 1
}

fn find_overlap_item(s: &str) -> char {
    let div = s.len() / 2;
    let p1 = &s[..div];
    let p2 = &s[div..];
    let set: HashSet<char> = p1.chars().collect();
    let inter: Vec<char> = p2.chars().filter(|c| set.contains(c)).collect();
    inter[0]
}

fn score_rucksack(rucksack: &str, items: &Vec<char>) -> usize {
    score_item(&find_overlap_item(rucksack), items)
}

fn main() {
    let items = get_item_list();
    let Input(path) = Input::process();
    let lines = get_lines(&path);
    let score: usize = lines.map(|x| score_rucksack(&x.unwrap(), &items)).sum();
    println!("Sum of priorities of Rucksack: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_item() {
        let items = get_item_list();
        assert_eq!(16, score_item(&'p', &items));
    }

    #[test]
    fn test_get_item_list() {
        let items = get_item_list();
        assert_eq!('a', items[0]);
        assert_eq!('A', items[26]);
        assert_eq!(items.len(), 52);
    }

    #[test]
    fn test_find_overlap() {
        let x = "catspisS";
        assert_eq!('s', find_overlap_item(x));
    }

    #[test]
    fn test_score_rucksack() {
        let rucksack = "catSpisS";
        let items = get_item_list();
        let score = score_rucksack(rucksack, &items);
        println!("{}", score)
    }
}
