use itertools::Itertools;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

// PARSING
pub fn parse_lines_file<T: AsRef<Path>, F>(filename: T) -> Result<Vec<F>, HelperError>
where
    F: FromStr,
{
    let mut file = File::open(filename)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    content
        .lines()
        .map(|l| FromStr::from_str(&l).map_err(|_| HelperError::ParsingError))
        .collect()
}

pub fn read_file<T: AsRef<Path>>(filename: T) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(filename)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn read_line_usize_from_file<T: AsRef<Path>>(
    filename: T,
) -> Result<Vec<usize>, Box<dyn Error>> {
    let numbers: Result<Vec<_>, _> = read_file(filename)?.lines().map(|l| l.parse()).collect();
    Ok(numbers?)
}

pub fn split_once<'a>(s: &'a str, pat: &str) -> (&'a str, &'a str){
    if let Some(split_at) = s.find(pat) {
        let (first, rest) = s.split_at(split_at);
        (first, &rest[pat.len()..])
    } else {
        (s, "")
    }
}

// GRID
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grid<T> {
    columns: usize,
    items: Vec<T>,
}

impl<T> Grid<T> {
    pub fn with_items(items: Vec<T>, columns: usize) -> Result<Self, GridError> {
        let len = items.len();
        if len % columns == 0 {
            Ok(Grid { columns, items })
        } else {
            Err(GridError::UnevenRowsError(len, columns))
        }
    }

    fn get_idx(&self, col: usize, row: usize) -> usize {
        row * self.columns + col
    }

    pub fn get(&self, col: usize, row: usize) -> Option<&T> {
        // println!("{}", self.columns);
        if col < self.columns {
            self.items.get(self.get_idx(col, row))
        } else {
            None
        }
    }

    pub fn num_columns(&self) -> usize {
        self.columns
    }

    pub fn num_rows(&self) -> usize {
        self.items.len() / self.columns
    }

    pub fn set(&mut self, col: usize, row: usize, v: T) {
        let idx = self.get_idx(col, row);
        self.items[idx] = v
    }

    fn get_unchecked(&self, col: usize, row: usize) -> &T {
        if col < self.columns {
            &self.items[self.get_idx(col, row)]
        } else {
            panic!("Index out of bounds!")
        }
    }

    pub fn items(&self) -> &[T] {
        &self.items
    }

    pub fn items_iter(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        let rows = self.items.len() / self.columns;
        (0..rows)
            .cartesian_product(0..self.columns)
            .map(move |(row, col)| ((col, row), self.get_unchecked(col, row)))
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, item) in self.items.iter().enumerate() {
            if i % self.columns == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", item)?;
        }
        Ok(())
    }
}

// ERROR HANDLING

#[derive(Error, Debug)]
pub enum HelperError {
    #[error("could not open file")]
    IoError(#[from] std::io::Error),
    #[error("parsing error")]
    ParsingError,
}

#[derive(Clone, Copy, Debug, Error)]
pub enum GridError {
    #[error("{0} items can not be divided among {1} columns")]
    UnevenRowsError(usize, usize),
}

#[cfg(test)]
mod tests {
    use crate::*;

    static TXT_CONTENT: &str = "Lorem ipsum
Foo
Bar";
    #[test]
    fn test_read_file() {
        let content = read_file("../assets/helpers/text.txt").unwrap();
        assert_eq!(TXT_CONTENT, content)
    }

    #[test]
    fn test_usize_from_file() {
        let lines = read_line_usize_from_file("../assets/helpers/usize.txt").unwrap();
        assert_eq!(&vec![1, 23, 47, 1_000_000], &lines)
    }
}
