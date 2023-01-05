use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::num::ParseIntError;
use std::str::FromStr;
use type_cli::CLI;

#[derive(CLI)]
struct Input(String);

#[derive(Debug)]
struct Assignment {
    start: u32,
    end: u32,
}

#[derive(Debug)]
struct ParseAssignmentError;

impl FromStr for Assignment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ix = s.find("-").unwrap();
        let start = &s[..ix];
        let end = &s[ix + 1..];
        Ok(Assignment {
            start: start.parse()?,
            end: end.parse()?,
        })
    }
}

#[derive(Debug)]
struct SectionAssignment {
    p1: Assignment,
    p2: Assignment,
}

#[derive(Debug)]
struct ParseSectionAssignmentError;

impl FromStr for SectionAssignment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ix = s.find(",").unwrap();
        let p1 = &s[..ix];
        let p2 = &s[ix + 1..];
        Ok(SectionAssignment {
            p1: Assignment::from_str(p1)?,
            p2: Assignment::from_str(p2)?,
        })
    }
}

fn get_lines(path: &String) -> Lines<BufReader<File>> {
    io::BufReader::new(File::open(path).expect("Failed to open file")).lines()
}

// Do the assignments overlap whatsoever?
fn contains_overlap(s: &SectionAssignment) -> bool {
    if s.p1.start == s.p2.start {
        return true;
    } else if s.p1.end == s.p2.end {
        return true;
    } else if s.p1.start <= s.p2.start {
        if s.p2.start <= s.p1.end {
            return true;
        }
    } else {
        if s.p1.start <= s.p2.end {
            return true;
        }
    }
    false
}

fn main() {
    let Input(path) = Input::process();
    let mut contained = 0;
    for line in get_lines(&path) {
        if let Ok(x) = line {
            let sa = SectionAssignment::from_str(&x).unwrap();
            if contains_overlap(&sa) {
                contained += 1;
            }
        }
    }
    println!("Assignments with any overlap with the other: {}", contained)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sass_from_string() {
        let s = "69-420,1-10";
        let sa = SectionAssignment::from_str(s).unwrap();
        println!("{:?}", sa);
    }

    #[test]
    fn test_overlap() {
        // Fully contained
        let s1 = SectionAssignment {
            p1: Assignment { start: 45, end: 47 },
            p2: Assignment { start: 24, end: 99 },
        };
        // Fully contained, end overlaps
        let s2 = SectionAssignment {
            p1: Assignment { start: 6, end: 6 },
            p2: Assignment { start: 4, end: 6 },
        };
        // Partial overlap
        let s3 = SectionAssignment {
            p1: Assignment { start: 34, end: 47 },
            p2: Assignment { start: 45, end: 99 },
        };
        // No overlap
        let s4 = SectionAssignment {
            p1: Assignment { start: 1, end: 10 },
            p2: Assignment { start: 45, end: 99 },
        };
        assert_eq!(true, contains_overlap(&s1));
        assert_eq!(true, contains_overlap(&s2));
        assert_eq!(true, contains_overlap(&s3));
        assert_eq!(false, contains_overlap(&s4));
    }
}
