use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::{Component, Path, PathBuf};

// Retrieve a Lines iterator for a given file path
pub fn get_lines(path: &str) -> Lines<BufReader<File>> {
    BufReader::new(File::open(path).expect("Failed to open file")).lines()
}

// Canonicalize built-in std::path requires the path to exist...
pub fn canonical_parent(path: &PathBuf) -> PathBuf {
    path.parent()
        .unwrap_or(Path::new(&Component::RootDir))
        .to_owned()
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use std::path::PathBuf;

//     fn test_path() -> String {
//         PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//             .join("test.txt")
//             .as_path()
//             .display()
//             .to_string()
//     }

//     #[test]
//     fn test_get_lines() {}
// }
