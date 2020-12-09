use helpers::parse_lines_file;
use itertools::{Itertools, MinMaxResult};
use std::error::Error;
use thiserror::Error;
use std::cmp::Ordering;

type Data = u64;

fn main() -> Result<(), Box<dyn Error>> {
    let numbers : Vec<Data> = parse_lines_file("./assets/days/day9.txt")?;

    let cypher = XMASCypher::new(&numbers);
    // Part 1
    // The first step of attacking the weakness in the XMAS data is to find the first number in the
    // list (after the preamble) which is not the sum of two of the 25 numbers before it.
    // What is the first number that does not have this property?
    let first_invalid = cypher.find_first_invalid(25);
    println!("First invalid number: {:?}", first_invalid);

    // Part 2
    // The final step in breaking the XMAS encryption relies on the invalid number you just found:
    if let Some(target_sum) = first_invalid {
        // you must find a contiguous set of at least two numbers in your list which sum to the invalid
        // number from step 1.
        if let Some(subset) = cypher.find_contiguous_subset_with_sum(target_sum) {
            // To find the encryption weakness, add together the smallest and largest number
            // in this contiguous range;
            if let MinMaxResult::MinMax(&min,&max) = subset.iter().minmax() {
                println!("Sum of smallest and largest number in subset {} + {} = {:?}", min, max, min + max);
                return Ok(())
            }
        }
    }

    Err(XMASCypherError::Day2Error.into())
}

struct XMASCypherValidator {
    ring_start_idx: usize,
    ringbuffer: Vec<Data>,
}

struct XMASCypher<'a> {
    data: &'a [Data],
}

impl<'a> XMASCypher<'a> {
    fn new(data: &'a [Data]) -> Self {
        XMASCypher{data}
    }

    // find the first number in the list (after the preamble) which is not the sum of two of the
    // n (n=`preamble_size`) numbers before it
    fn find_first_invalid(&self, preamble_size: usize) -> Option<Data> {
        let mut data = self.data.iter().copied();
        let preamble: Vec<Data> = (&mut data).take(preamble_size).collect();
        let mut cypher = XMASCypherValidator::with_preamble(preamble);

        data.find(|v| cypher.step(*v) == Step::Invalid )
    }

    // find a contiguous set of at least two numbers in your list which sum to the given target
    fn find_contiguous_subset_with_sum(&self, target_sum: Data) -> Option<&[Data]>{
        for i in 0..self.data.len() {
            for j in i+1..self.data.len() {
                let subset = &self.data[i..j];
                let sum: u64 = subset.iter().sum();

                match sum.cmp(&target_sum) {
                    // from here on it will only get larger, no need to continue
                    Ordering::Greater => break,
                    Ordering::Equal => return Some(subset),
                    Ordering::Less => {}
                }
            }
        }

        None
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Step {
    Valid,
    Invalid,
}

impl XMASCypherValidator {
    fn with_preamble(preamble: Vec<Data>) -> Self {
        XMASCypherValidator {
            ring_start_idx: 0,
            ringbuffer: preamble,
        }
    }

    fn step(&mut self, value: Data) -> Step {
        let two_numbers_match_sum = self
            .ringbuffer
            .iter()
            .combinations(2)
            .any(|a| a[0] + a[1] == value);
        if two_numbers_match_sum {
            self.ringbuffer[self.ring_start_idx] = value;
            self.ring_start_idx = (self.ring_start_idx + 1) % self.ringbuffer.len();
            Step::Valid
        } else {
            Step::Invalid
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::MinMaxResult;

    static EXPECTED_INVALID: Data = 127;
    // In this example, after the 5-number preamble, almost every number is the sum of two of the
    // previous 5 numbers; the only number that does not follow this rule is 127.
    #[test]
    fn test_example_day1() {
        let input = read_file("../assets/days/day9_example.txt").unwrap();
        let numbers: Vec<_> = input.lines().map(|l| l.parse().unwrap()).collect();

        let cypher = XMASCypher::new(&numbers);
        let first_invalid = cypher.find_first_invalid(5);

        assert_eq!(Some(EXPECTED_INVALID), first_invalid)
    }

    // In this list, adding up all of the numbers from 15 through 40 produces the invalid number
    // from step 1, 127.
    #[test]
    fn test_example_day2() {
        let input = read_file("../assets/days/day9_example.txt").unwrap();
        let numbers: Vec<_> = input.lines().map(|l| l.parse().unwrap()).collect();

        let cypher = XMASCypher::new(&numbers);

        // To find the encryption weakness, add together the smallest and largest number in this contiguous range;
        let sub = cypher.find_contiguous_subset_with_sum(EXPECTED_INVALID).unwrap();
        let sum: Data = sub.iter().sum();

        // got correct sum of slice
        assert_eq!(EXPECTED_INVALID, sum);

        // in this example, these are 15 and 47, producing 62.
        if let MinMaxResult::MinMax(&min,&max) = sub.iter().minmax() {
            assert_eq!(min,15);
            assert_eq!(max,47);
        } else {
            panic!("Could not get minimum and maximum numbers!");
        };
    }
}

#[derive(Error, Debug, Clone)]
enum XMASCypherError {
    #[error("day2 not run")]
    Day2Error,
}