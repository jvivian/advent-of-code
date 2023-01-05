use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::str;
use type_cli::CLI;

#[derive(CLI)]
struct Input(String);

fn get_lines(path: &String) -> Lines<BufReader<File>> {
    BufReader::new(File::open(path).expect("Failed to open file")).lines()
}

// Checks if a length `n` string is a marker
fn is_marker(s: &str, n: usize) -> bool {
    if s.chars().collect::<Vec<char>>().len() != n {
        return false;
    }
    if s.chars().collect::<HashSet<char>>().len() == n {
        return true;
    }
    false
}

#[derive(Debug)]
struct NoMarkerPosError;

// Return the end index for a size `n` marker in a given string
fn marker_pos(s: &str, n: usize) -> Result<usize, NoMarkerPosError> {
    for i in 0..s.len() {
        if is_marker(&s[i..i + n], n) {
            return Ok(i + n);
        }
    }
    Err(NoMarkerPosError)
}

fn main() {
    let Input(path) = Input::process();
    let lines = get_lines(&path);
    if let Some(Ok(s)) = lines.into_iter().next() {
        let pos = marker_pos(&s, 14).expect("Failed to find marker");
        println!("{}", pos);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_marker() {
        let s1 = "mjqj";
        let s2 = "jqjp";
        let s3 = "qjpq";
        let s4 = "jpqm";
        assert_eq!(is_marker(s1, 4), false);
        assert_eq!(is_marker(s2, 4), false);
        assert_eq!(is_marker(s3, 4), false);
        assert_eq!(is_marker(s4, 4), true);
    }

    #[test]
    fn test_marker_pos() {
        let s = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        println!("{}", marker_pos(s, 4).unwrap());
    }
}
