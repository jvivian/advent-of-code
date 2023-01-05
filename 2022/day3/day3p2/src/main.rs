use itertools::Itertools;
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
    items.iter().position(|r| r == c).unwrap() + 1
}

fn find_rucksack_overlap(rucksacks: Vec<String>) -> char {
    let set: HashSet<char> = rucksacks[0].chars().collect();
    let _inter: HashSet<char> = rucksacks[1].chars().filter(|c| set.contains(c)).collect();
    rucksacks[2]
        .chars()
        .filter(|c| _inter.contains(c))
        .collect::<Vec<char>>()[0]
}

fn score_rucksacks(rucksacks: Vec<String>, items: &Vec<char>) -> usize {
    score_item(&find_rucksack_overlap(rucksacks), items)
}

fn main() {
    let items = get_item_list();
    let Input(path) = Input::process();
    let lines = get_lines(&path);
    let mut score = 0;
    for chunk in &lines.into_iter().chunks(3) {
        let rucksacks = chunk.map(|x| x.unwrap()).collect_vec();
        score += score_rucksacks(rucksacks, &items);
    }
    println!("Sum for each three-Elf group is: {}", score);
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
    fn test_score_rucksacks() {
        let rucksacks = vec!["catSpisS".to_owned(), "Shit".to_owned(), "aSs".to_owned()];
        let items = get_item_list();
        assert_eq!(45, score_rucksacks(rucksacks, &items));
    }
}
