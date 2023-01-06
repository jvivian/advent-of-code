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
struct SectionAssignment {
    person1: Assignment,
    person2: Assignment,
}

#[derive(Debug)]
struct ParseSectionAssignmentError;

impl FromStr for SectionAssignment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ix = s.find(",").unwrap();
        let p1 = &s[..ix];
        let p2 = &s[ix + 1..];
        let pix1 = p1.find("-").unwrap();
        let s1 = &p1[..pix1];
        let e1 = &p1[pix1 + 1..];
        let pix2 = p2.find("-").unwrap();
        let s2 = &p2[..pix2];
        let e2 = &p2[pix2 + 1..];
        Ok(SectionAssignment {
            person1: Assignment {
                start: s1.parse()?,
                end: e1.parse()?,
            },
            person2: Assignment {
                start: s2.parse()?,
                end: e2.parse()?,
            },
        })
    }
}

fn get_lines(path: &String) -> Lines<BufReader<File>> {
    io::BufReader::new(File::open(path).expect("Failed to open file")).lines()
}

fn contains_containment(s: &SectionAssignment) -> bool {
    if s.person1.start == s.person2.start {
        return true;
    } else if s.person1.end == s.person2.end {
        return true;
    } else if s.person1.start <= s.person2.start {
        if s.person1.end >= s.person2.end {
            return true;
        }
    } else {
        if s.person1.end <= s.person2.end {
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
            if contains_containment(&sa) {
                contained += 1;
            }
        }
    }
    println!("Assignments that fully contain the other: {}", contained)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        // Fully contained
        let s1 = SectionAssignment {
            person1: Assignment { start: 45, end: 47 },
            person2: Assignment { start: 24, end: 99 },
        };
        // Fully contained, end overlaps
        let s2 = SectionAssignment {
            person1: Assignment { start: 6, end: 6 },
            person2: Assignment { start: 4, end: 6 },
        };
        // Fully contained, end overlaps reversed
        let s5 = SectionAssignment {
            person1: Assignment { start: 4, end: 6 },
            person2: Assignment { start: 5, end: 6 },
        };
        // Partial overlap
        let s3 = SectionAssignment {
            person1: Assignment { start: 34, end: 47 },
            person2: Assignment { start: 45, end: 99 },
        };
        // No overlap
        let s4 = SectionAssignment {
            person1: Assignment { start: 1, end: 10 },
            person2: Assignment { start: 45, end: 99 },
        };
        assert_eq!(true, contains_containment(&s1));
        assert_eq!(true, contains_containment(&s2));
        assert_eq!(true, contains_containment(&s5));
        assert_eq!(false, contains_containment(&s3));
        assert_eq!(false, contains_containment(&s4));
    }

    #[test]
    fn test_sass_from_string() {
        let s = "69-420,1-10";
        let sa = SectionAssignment::from_str(s).unwrap();
        println!("{:?}", sa);
    }
}
