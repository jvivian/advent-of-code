use aoclib::{canonical_parent, get_lines};
use log::{debug, info};
use pretty_env_logger;
use std::{
    collections::HashMap,
    fmt::Display,
    num::ParseIntError,
    path::{Component, Path, PathBuf},
    str::FromStr,
};
use thiserror::Error;
use type_cli::CLI;

#[derive(CLI)]
struct Input(String);

#[derive(Debug, Default, PartialEq, Clone)]
struct File {
    name: String,
    size: u32,
}

#[derive(Error, Debug)]
enum ParseFileError {
    #[error("Improper format for `ls` result {0}")]
    InvalidItem(String),
    #[error("Failed to parse integer")]
    InvalidInteger(#[from] ParseIntError),
}

impl FromStr for File {
    type Err = ParseFileError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let svec = s.split_whitespace().collect::<Vec<&str>>();
        if svec.len() != 2 {
            return Err(ParseFileError::InvalidItem(s.to_owned()));
        }
        Ok(File {
            name: svec[1].into(),
            size: svec[0].parse()?,
        })
    }
}

#[derive(PartialEq, Debug)]
enum Command {
    Cd(String),
    Ls,
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut svec = s.split_whitespace();
        svec.next();
        match svec.next() {
            Some("cd") => Ok(Command::Cd(
                svec.next().expect("`cd` missing directory").into(),
            )),
            Some("ls") => Ok(Command::Ls),
            _ => Err(ParseCommandError),
        }
    }
}
#[derive(Error, Debug)]
struct ParseCommandError;

impl Display for ParseCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to Parse Command")
    }
}

#[derive(Debug, Error)]
enum FileSystemError {
    #[error("Error parsing command")]
    CmdError(#[from] ParseCommandError),
    #[error("Error parsing File")]
    FileError(#[from] ParseFileError),
    #[error("Failed to canonicalize path")]
    CanonicalizeError(#[from] std::io::Error),
}

// FileSystem contains a key/value mapping of the Path to a File
#[derive(Debug)]
struct FileSystem {
    fmap: HashMap<PathBuf, File>,
    cwd: PathBuf,
}

impl Default for FileSystem {
    fn default() -> Self {
        FileSystem {
            fmap: HashMap::new(),
            cwd: Path::new(&Component::RootDir).into(),
        }
    }
}

impl FileSystem {
    fn traverse(mut self, path: &str) -> Result<Self, FileSystemError> {
        for line in get_lines(&path).skip(1) {
            if let Ok(x) = line {
                match Command::from_str(&x) {
                    Ok(Command::Cd(d)) => match d.as_str() {
                        ".." => self.cwd = canonical_parent(&self.cwd),
                        _ => self.cwd = self.cwd.join(&d),
                    },
                    Ok(Command::Ls) => {}
                    Err(ParseCommandError) => {
                        if let Ok(File { name, size }) = File::from_str(&x) {
                            self.fmap.insert(self.cwd.join(&name), File { name, size });
                        }
                    }
                }
            }
        }
        Ok(self)
    }
}

// A file's size counts towards all directories it resides within
fn dir_sizes(fs: &FileSystem) -> HashMap<String, u32> {
    let mut sizes = HashMap::new();
    for (p, f) in fs.fmap.iter() {
        for path in p.ancestors().skip(1) {
            if let Some(key) = path.to_str() {
                let s = sizes.entry(key.to_string()).or_insert(0);
                *s += f.size;
            }
        }
    }
    sizes
}

fn space_remaining(fs: &FileSystem, total: u32) -> u32 {
    let sizes = dir_sizes(fs);
    let taken = root_size(sizes).expect("No valid root found!");
    total - taken
}

// Finds root given unix `/` or windows `\\`
fn root_size(sizes: HashMap<String, u32>) -> Option<u32> {
    match sizes.get("/") {
        Some(k) => Some(*k),
        None => match sizes.get("\\") {
            Some(k) => Some(*k),
            None => None,
        },
    }
}

fn main() {
    pretty_env_logger::init();
    debug!("Traversing file system");
    let Input(path) = Input::process();
    let fs = FileSystem::default()
        .traverse(&path)
        .expect("Failed to traverse file system");
    debug!("Calculating directory sizes...");
    let mut sizes = dir_sizes(&fs);
    debug!("Finding smallest directory to fit criteria...");
    let total = 70000000;
    let required = 30000000;
    let size_needed = required - space_remaining(&fs, total);
    sizes.retain(|_, v| *v >= size_needed);
    let mut smallest = u32::MAX;
    for (k, v) in &sizes {
        if v < &smallest {
            println!("New smallest size!\n\t{}\t{}", k, v);
            smallest = v.to_owned();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_path() -> String {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("test.txt")
            .as_path()
            .display()
            .to_string()
    }

    #[test]
    fn test_root_size() {
        let path = test_path();
        let fs = FileSystem::default().traverse(&path).unwrap();
        let sizes = dir_sizes(&fs);
        let rval = root_size(sizes).unwrap();
        assert_eq!(rval, 48381165);
    }

    #[test]
    fn test_dir_size() {
        let path = test_path();
        let fs = FileSystem::default().traverse(&path).unwrap();
        let mut sizes = dir_sizes(&fs);
        assert_eq!(sizes.get("\\a").unwrap(), &94853);
        assert_eq!(sizes.get("\\d").unwrap(), &24933642);
        assert_eq!(sizes.get("\\a\\e").unwrap(), &584);
        println!("{:?}", sizes);
        sizes.retain(|_, v| *v <= 100000);
        println!("{:?}", sizes);
        let total: u32 = sizes.values().sum();
        println!("{:?}", total);
    }

    #[test]
    fn test_traverse_fs() {
        let path = test_path();
        let fs = FileSystem::default().traverse(&path).unwrap();
        let path = Path::new(&Component::RootDir).join("d").join("d.ext");
        let res = fs.fmap.get(&path).unwrap().to_owned();
        let f = File {
            name: "d.ext".into(),
            size: 5626152,
        };
        assert_eq!(f, res);
        println!("{:?}", fs);
        println!("{:?}", path);
    }

    #[test]
    fn test_command() {
        let s = "$ cd d";
        let c = Command::from_str(s).unwrap();
        assert_eq!(c, Command::Cd(String::from("d")));
        let s = "$ ls";
        let c = Command::from_str(s).unwrap();
        assert_eq!(c, Command::Ls);
    }
}
