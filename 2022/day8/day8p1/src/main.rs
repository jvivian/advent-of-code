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

#[derive(Debug)]
struct Forest {
    north: DataFrame,
    south: DataFrame,
    west: DataFrame,
    east: DataFrame,
}

impl Forest {
    fn new(df: &DataFrame) -> Forest {
        Forest {
            north: df.clone(),
            south: df.clone().reverse(),
            west: df.clone().transpose().unwrap(),
            east: df.clone().transpose().unwrap().reverse(),
        }
    }

    fn mask_trees(mut self) -> Self {
        mask_visible(&mut self.north);
        mask_visible(&mut self.south);
        mask_visible(&mut self.west);
        mask_visible(&mut self.east);
        self.south = self.south.reverse();
        self.west = self.west.transpose().unwrap();
        self.east = self.east.reverse().transpose().unwrap();
        self
    }

    fn visibility_matrix(self) -> Result<DataFrame, PolarsError> {
        Ok(DataFrame::new(
            vec![self.north, self.south, self.west, self.east]
                .iter()
                .zip(NAMES)
                .map(|(d, name)| flatten(d, name))
                .collect(),
        )?)
    }
}

// Get owned column names from DataFrame
fn get_cols(df: &DataFrame) -> Vec<String> {
    df.get_column_names()
        .iter()
        .map(|x| x.to_string())
        .collect()
}

// Series map function to determine if a tree is visible
fn is_visible(s: &Series) -> Series {
    let mut big = -1;
    s.iter()
        .map(|v| {
            let x = v.to_string().parse::<i32>().unwrap();
            if x > big {
                big = x;
                return true;
            }
            false
        })
        .collect()
}

// Converts dataframe into boolean mask representing visibility
fn mask_visible(df: &mut DataFrame) {
    for name in &get_cols(&df) {
        df.apply(name, is_visible).unwrap();
    }
}

// Flatten dataframe into series
fn flatten(df: &DataFrame, name: &str) -> Series {
    Series::new(
        name,
        df.iter()
            .flat_map(|s| {
                s.clone()
                    .bool()
                    .unwrap()
                    .into_iter()
                    .collect::<Vec<Option<bool>>>()
            })
            .map(|x| x.unwrap_or(false))
            .collect::<Vec<_>>(),
    )
}

fn count_visible(df: &DataFrame) -> Result<usize, PolarsError> {
    Ok(Forest::new(df)
        .mask_trees()
        .visibility_matrix()?
        .lazy()
        .select([fold_exprs(lit(0), |acc, x| Ok(acc + x), [all()])])
        .filter(all().gt(lit(0)))
        .collect()?
        .shape()
        .0)
}

fn main() {
    let Input(grid_path) = Input::process();
    let mut csv_path = PathBuf::from(&grid_path);
    csv_path.set_extension("csv");
    grid_to_csv(&grid_path, csv_path.to_str().unwrap());
    let file = File::open(&csv_path).unwrap();
    let df = CsvReader::new(file).has_header(false).finish().unwrap();
    let n = count_visible(&df).unwrap();
    println!(
        "The total number of visible trees of grid shape {:?} was {}",
        df.shape(),
        n
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
    fn test_count_visible() {
        let df = get_df();
        let n = count_visible(&df).unwrap();
        assert_eq!(n, 21);
    }

    #[test]
    fn test_flatten() {
        let mut df = get_df();
        mask_visible(&mut df);
        let flat = flatten(&df, "foo");
        assert_eq!(flat.get(0).unwrap().to_string(), "true");
    }
    #[test]
    fn test_is_vis_north() {
        let mut df = get_df();
        mask_visible(&mut df);
        println!("{}", &df);
        assert_eq!("false", df["column_1"].get(1).unwrap().to_string());
        assert_eq!("true", df["column_1"].get(2).unwrap().to_string());
    }

    #[test]
    fn test_is_vis_south() {
        let mut df = get_df().reverse();
        mask_visible(&mut df);
        df = df.reverse();
        println!("{}", &df);
        assert_eq!("false", df["column_1"].get(0).unwrap().to_string());
        assert_eq!("true", df["column_1"].get(2).unwrap().to_string());
    }

    #[test]
    fn test_is_vis_west() {
        let mut df = get_df().transpose().unwrap();
        mask_visible(&mut df);
        let res = df.transpose().unwrap();
        println!("{}", &res);
        assert_eq!("false", res["column_1"].get(0).unwrap().to_string());
        assert_eq!("true", res["column_1"].get(1).unwrap().to_string());
    }

    #[test]
    fn test_is_vis_east() {
        let mut df = get_df().transpose().unwrap().reverse();
        mask_visible(&mut df);
        let res = df.reverse().transpose().unwrap();
        println!("{}", &res);
        assert_eq!("false", res["column_0"].get(0).unwrap().to_string());
        assert_eq!("true", res["column_0"].get(2).unwrap().to_string());
    }
    #[test]
    fn test_is_visible() {
        let mut df = get_df();
        for name in &get_cols(&df) {
            df.apply(name, is_visible).unwrap();
        }
        println!("{:?}", &df);
        assert_eq!(df.shape(), (5, 5));
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
