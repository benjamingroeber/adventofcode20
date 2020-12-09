use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

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

#[derive(Error, Debug)]
pub enum HelperError {
    #[error("could not open file")]
    IoError(#[from] std::io::Error),
    #[error("parsing error")]
    ParsingError,
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
