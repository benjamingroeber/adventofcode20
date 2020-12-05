use crate::TicketError::NoSeatFound;
use helpers::read_file;
use itertools::Itertools;
use std::convert::TryFrom;
use std::error::Error;
use thiserror::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_file("./assets/days/day5.txt")?;
    let mut tickets = input.lines().map(BoardingPass::try_from).collect::<Result<Vec<_>,_,>>()?;
    tickets.sort_by_key(|t| t.seat_id);

    // Part 1:  What is the highest seat ID on a boarding pass?
    let highest = tickets.last().ok_or(NoSeatFound)?.seat_id;
    println!("Highes encountered seat id: {:?}", highest);

    // Part 2: Your seat wasn't at the very front or back, though;
    // the seats with IDs +1 and -1 from yours will be in your list.
    // What is the ID of your seat?
    let (before, after) = tickets
        .iter()
        .tuple_windows::<(_, _)>()
        .find(|(a, b)| a.seat_id + 1 != b.seat_id)
        .ok_or(NoSeatFound)?;
    println!(
        "Empty Seat Id {} between {:?} and {:?}!",
        before.seat_id + 1,
        before,
        after
    );

    Ok(())
}

static _PLANE_ROWS: usize = 128;
static PLANE_COLUMNS: usize = 8;

#[derive(Clone, Debug)]
struct BoardingPass<'a> {
    row_input: &'a str,
    col_input: &'a str,
    row_id: usize,
    col_id: usize,
    seat_id: usize,
}

impl<'a> BoardingPass<'a> {
    // Every seat also has a unique seat ID: multiply the row by 8, then add the column.
    fn compute_seat_id(row_id: usize, col_id: usize) -> usize {
        row_id * PLANE_COLUMNS + col_id
    }
}

#[derive(Error, Debug)]
pub enum TicketError {
    #[error("unexpected token `{0}` in SeatNumber `{1}`")]
    UnexpectedToken(String, String),
    #[error("SeatNumber {0} should be exactly 10 characters long")]
    UnexpectedLength(String),
    #[error("Row Identifiers {0} should only contain '{1}' or '{2}'")]
    UnexpectedBinaryToken(String, char,char),
    #[error("Boarding error, Seat Row {0}, column {1} does not exist!")]
    BoardingError(usize, usize),
    #[error("no Seat found")]
    NoSeatFound,
}

fn binary_partition(input: &str, high: char, low: char) -> Result<usize, TicketError> {
    let mut value = 0;
    for (i, c) in input.chars().rev().enumerate() {
        if c == high {
            value += 2_usize.pow(i as u32)
        } else if c == low {} else {
           return Err(TicketError::UnexpectedBinaryToken(input.to_string(), high, low));
        }
    }
    Ok(value)
}

// this airline uses binary space partitioning to seat people.
// A seat might be specified like FBFBBFFRLR, where
// F means "front", B means "back", L means "left", and R means "right".
impl<'a> TryFrom<&'a str> for BoardingPass<'a> {
    type Error = TicketError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        // 7 row chars + 3 col chars
        if s.chars().count() != 10 {
            return Err(TicketError::UnexpectedLength(s.to_owned()));
        }

        // The first 7 characters will either be F or B; these specify exactly one of the 128
        // rows on the plane (numbered 0 through 127)
        // The last three characters will be either L or R; these specify exactly one of the 8
        // columns of seats on the plane (numbered 0 through 7).
        let (row, col) = s.split_at(7);
        let row_id = binary_partition(row, 'B', 'F')?;
        let col_id = binary_partition(col, 'R', 'L')?;

        // Every seat also has a unique seat ID: multiply the row by 8, then add the column.
        let seat_id = BoardingPass::compute_seat_id(row_id, col_id);
        Ok(BoardingPass {
            row_input: row,
            col_input: col,
            row_id,
            col_id,
            seat_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // FBFBBFFRLR
    #[test]
    fn test_example() {
        let boarding_pass = BoardingPass::try_from("FBFBBFFRLR").unwrap();

        // Start by considering the whole range, rows 0 through 127.
        // F means to take the lower half, keeping rows 0 through 63.
        // B means to take the upper half, keeping rows 32 through 63.
        // F means to take the lower half, keeping rows 32 through 47.
        // B means to take the upper half, keeping rows 40 through 47.
        // B keeps rows 44 through 47.
        // F keeps rows 44 through 45.
        // The final F keeps the lower of the two, row 44.
        assert_eq!(boarding_pass.row_id, 44);

        // Start by considering the whole range, columns 0 through 7.
        // R means to take the upper half, keeping columns 4 through 7.
        // L means to take the lower half, keeping columns 4 through 5.
        // The final R keeps the upper of the two, column 5.
        assert_eq!(boarding_pass.col_id, 5);

        // In this example, the seat has ID 44 * 8 + 5 = 357.
        assert_eq!(boarding_pass.seat_id, 357)
    }

    #[test]
    fn test_example_raw() {
        // same as above
        assert_eq!( 44, binary_partition("FBFBBFF",'B', 'F').unwrap());
        assert_eq!( 5, binary_partition("RLR",'R', 'L').unwrap());
    }

    #[test]
    fn test_compute_seat_id() {
        // In this example, the seat has ID 44 * 8 + 5 = 357
        assert_eq!(BoardingPass::compute_seat_id(44,5),357)
    }
}
