use crate::CupsError::InvalidCupNumbering;
use itertools::{Itertools, MinMaxResult};
use std::error::Error;
use std::fs::read_to_string;
use std::str::FromStr;
use thiserror::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let _input = read_to_string("./assets/days/day23.txt")?;
    println!("Hello, world!");
    Ok(())
}

struct Cup{
    label: usize,
    next: usize
}

#[derive(Clone, Debug)]
struct Cups {
    min: usize,
    max: usize,
    cur_pos: usize,
    cups: Vec<Cup>,
}

impl Cups {
    fn with_cups(cups: Vec<Cup>) -> Result<Self, CupsError> {
        let (min, max) = match cups.iter().minmax() {
            MinMaxResult::MinMax(min, max) => (*min, *max),
            _ => return Err(InvalidCupNumbering),
        };
        // Each number must exist only once
        for number in min..=max {
            if cups.iter().filter(|c| **c == number).count() != 1 {
                return Err(InvalidCupNumbering);
            }
        }
        // check for all between min and max for wrapping around
        Ok(Cups {
            min,
            max,
            cur_pos: 0,
            cups,
        })
    }
    fn play_round(&mut self) {
        // The crab picks up the three cups that are immediately clockwise of the current cup. They
        // are removed from the circle; cup spacing is adjusted as necessary to maintain the circle.
        let (c1, c2, c3) = (
            self.get_cup_with_offset(1),
            self.get_cup_with_offset(2),
            self.get_cup_with_offset(3),
        );

        println!("Taking {} {} {}", c1, c2, c3);
        // The crab selects a destination cup: the cup with a label equal to the current cup's label
        // minus one.
        let current_cup = self.get_cur_label();
        let mut destination_label = self.get_previous_label(current_cup);

        // If this would select one of the cups that was just picked up, the crab will
        // keep subtracting one until it finds a cup that wasn't just picked up.
        while destination_label == c1 || destination_label == c2 || destination_label == c3 {
            destination_label = self.get_previous_label(destination_label);
        }
        println!("Destination: {}", destination_label);

        // The crab places the cups it just picked up so that they are immediately clockwise of the
        // destination cup.
        let dest_idx = self.next_pos(self.get_cup_idx(destination_label));
        // They keep the same order as when they were picked up.
        let orig_idx = self.get_cup_idx(c1);


        println!("IDX {} <->{}", orig_idx, dest_idx);
        self.swap_three(orig_idx, dest_idx );

        // The crab selects a new current cup: the cup which is immediately clockwise of the current
        // cup.
        self.cur_pos = self.next_pos(self.cur_pos);
    }

    fn next_pos(&self, cur_pos: usize) -> usize {
        (cur_pos + 1) % self.cups.len()
    }

    fn swap_three(&mut self, origin: usize, destination: usize) {
        self.swap_n(origin, destination, 3)
    }

    fn swap_n(&mut self, origin: usize, destination: usize, n: usize) {
        for offset in 0..n {
            let idx_orig = (origin + offset)%self.cups.len();
            let idx_dest = (destination + offset)%self.cups.len();
            println!("Swapping {} with {}", self.cups[idx_orig], self.cups[idx_dest]);
            self.cups.swap(idx_orig, idx_dest);
        }
    }

    fn get_cup_idx(&self, cup: Cup) -> usize {
        self
            .cups
            .iter()
            .enumerate()
            .find(|(_, c)| **c == cup)
            // every cup of the same game must exist here
            .unwrap()
            .0
    }

    // If at any point in this process the value goes below the lowest value on any cup's label,
    // it wraps around to the highest value on any cup's label instead.
    fn get_previous_label(&self, cup: Cup) -> usize {
        if cup == self.min {
            self.max
        } else {
            cup - 1
        }
    }

    fn get_cur_label(&self) -> Cup {
        self.cups[self.cur_pos]
    }

    fn get_cup_with_offset(&self, offset: usize) -> Cup {
        self.cups[(self.cur_pos + offset) % self.cups.len()]
    }
}

impl FromStr for Cups {
    type Err = CupsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cups: Option<Vec<Cup>> = s
            .chars()
            .map(|c| c.to_digit(10).map(|d| d as usize))
            .collect();

        Cups::with_cups(cups.ok_or(CupsError::ParseCup)?)
    }
}

#[derive(Clone, Debug, Error)]
enum CupsError {
    #[error("could not parse cup ids")]
    ParseCup,
    #[error("cups must be at least 4 contiguous unique numbers")]
    InvalidCupNumbering,
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "389125467";
    #[test]
    fn test_example_part1() {

        let mut cups = Cups::from_str(TEST_INPUT).unwrap();
        println!("{:?}", cups);
        cups.play_round();
        println!("{:?}", cups);
    }
}
