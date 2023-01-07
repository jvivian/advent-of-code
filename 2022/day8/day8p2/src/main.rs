/// Advent of Code - Day 8
/// Author: John Vivian
/// Language: Rust
use aoclib::get_lines;
use polars::lazy::dsl::fold_exprs;
use polars::prelude::*;
use std::fs::File;
use std::io::Write;
use std::num::ParseIntError;
use std::path::PathBuf;
use thiserror::Error;
use type_cli::CLI;

#[derive(CLI)]
struct Input(String);
// Cardinal names
const NAMES: &'static [&str] = &["North", "South", "West", "East"];

// Converts grid of numbers to csv of numbers
fn grid_to_csv(in_path: &str, out_path: &str) {
    let output = PathBuf::from(out_path);
    if output.exists() {
        std::fs::remove_file(&output).expect("Error deleting file");
    }
    let mut f = File::create(&output).unwrap();
    for line in get_lines(&in_path) {
        if let Ok(x) = line {
            let s = x
                .chars()
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",");
            writeln!(f, "{}", s).expect("Failed to write file");
        }
    }
}

#[derive(Error, Debug)]
enum GridError {
    #[error("Invalid grid value, unable to convert to i32")]
    InvalidGridValueError(#[from] ParseIntError),
}

// Container for holding views of the forest
#[derive(Debug)]
struct Forest {
    north: DataFrame,
    south: DataFrame,
    west: DataFrame,
    east: DataFrame,
    visdf: Option<DataFrame>,
}

impl Forest {
    fn new(df: &DataFrame) -> Forest {
        Forest {
            north: df.clone(),
            south: df.clone().reverse(),
            west: df.clone().transpose().unwrap(),
            east: df.clone().transpose().unwrap().reverse(),
            visdf: None,
        }
    }

    // Converts grid into count of tree visibility
    fn count_trees(mut self) -> Self {
        for df in [
            &mut self.north,
            &mut self.south,
            &mut self.west,
            &mut self.east,
        ] {
            for name in &get_cols(df) {
                df.apply(name, num_visible).unwrap();
            }
        }
        self.south = self.south.reverse();
        self.west = self.west.transpose().unwrap();
        self.east = self.east.reverse().transpose().unwrap();
        self
    }

    // Converts boolean grid into [TREE x DIRECTION] matrix
    fn visibility_matrix(mut self) -> Result<Self, PolarsError> {
        self.visdf = Some(DataFrame::new(
            vec![&self.north, &self.south, &self.west, &self.east]
                .iter()
                .zip(NAMES)
                .map(|(d, name)| flatten(d, name))
                .collect(),
        )?);
        Ok(self)
    }

    // Calcualte scenic scores
    fn scenic_scores(self) -> Result<DataFrame, PolarsError> {
        self.visdf
            .expect("Missing visibility matrix")
            .lazy()
            .with_column(fold_exprs(lit(1), |acc, x| Ok(acc * x), [all()]).alias("Scenic Score"))
            .sort(
                "Scenic Score",
                SortOptions {
                    descending: false,
                    nulls_last: false,
                },
            )
            .collect()
    }
}

// Get owned column names from DataFrame
fn get_cols(df: &DataFrame) -> Vec<String> {
    df.get_column_names()
        .iter()
        .map(|x| x.to_string())
        .collect()
}

// Count number of trees visible for each tree
fn num_visible(s: &Series) -> Series {
    let v = s
        .iter()
        .map(|x| x.try_extract::<i32>().expect("Could not parse i32"))
        .collect::<Vec<i32>>();
    s.iter()
        .enumerate()
        .map(|(i, x)| {
            let mut n = 0;
            for j in (0..i).rev() {
                let val = x.try_extract::<i32>().expect("Couldn't parse i32");
                if val > v[j] {
                    n += 1;
                } else {
                    return n + 1;
                }
            }
            n
        })
        .collect()
}

// Flatten dataframe into series
fn flatten(df: &DataFrame, name: &str) -> Series {
    Series::new(
        name,
        df.iter()
            .flat_map(|s| {
                s.clone()
                    .i32()
                    .unwrap()
                    .into_iter()
                    .collect::<Vec<Option<i32>>>()
            })
            .map(|x| x.unwrap_or(-1))
            .collect::<Vec<_>>(),
    )
}

fn main() {
    let Input(grid_path) = Input::process();
    let mut csv_path = PathBuf::from(&grid_path);
    csv_path.set_extension("csv");
    grid_to_csv(&grid_path, csv_path.to_str().unwrap());
    let file = File::open(&csv_path).unwrap();
    let df = CsvReader::new(file).has_header(false).finish().unwrap();
    let f = Forest::new(&df).count_trees().visibility_matrix().unwrap();
    let scores = f.scenic_scores().unwrap();
    println!(
        "The highest scenic scores of grid size {:?} were {}",
        df.shape(),
        scores.tail(Some(7))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    // fn test_path() -> PathBuf {
    //     PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.txt")
    // }

    fn get_df() -> DataFrame {
        let csv_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.csv");
        let file = File::open(&csv_path).unwrap();
        CsvReader::new(file).has_header(false).finish().unwrap()
    }

    #[test]
    fn test_scenic_scores() {
        let df = get_df();
        let f = Forest::new(&df).count_trees().visibility_matrix().unwrap();
        let scores = f.scenic_scores().unwrap();
        println!("{}", scores);
    }

    #[test]
    fn test_count_trees() {
        let df = get_df();
        let f = Forest::new(&df).count_trees().visibility_matrix().unwrap();
        println!("{:?}", f);
    }
    #[test]
    fn test_num_visible() {
        let mut df = get_df();
        for name in &get_cols(&df) {
            df.apply(name, num_visible).unwrap();
        }
        println!("{}", df);
    }

    #[test]
    fn test_read_grid_csv() {
        let df = get_df();
        println!("{}", df);
        println!("{}", df["column_1"]);
        let foo = df["column_1"].get(0).unwrap().to_string();
        assert_eq!("3", foo);
    }
}
