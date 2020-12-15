use helpers::read_file;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

type Unit = usize;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_file("./assets/days/day15.txt").expect("error reading inputfile");
    let starting_numbers: Result<Vec<_>, _> = input.split(',').map(Unit::from_str).collect();

    // Part 1
    // Given your starting numbers, what will be the 2020th number spoken?
    let mut memory = Memory::with_starting_numbers(&starting_numbers?);
    let part_1_limit = 2020;
    println!(
        "Number at {} is: {:?}",
        part_1_limit,
        memory.run_until(part_1_limit)
    );

    // Part 2
    // Given your starting numbers, what will be the 30000000th number spoken?
    let part_2_limit = 30_000_000;
    println!(
        "Number at {} is: {:?}",
        part_1_limit,
        memory.run_until(part_2_limit)
    );
    Ok(())
}

// They're playing a memory game and are ever so excited to explain the rules!
// In this game, the players take turns saying numbers.
#[derive(Clone, Debug)]
struct Memory {
    numbers: HashMap<Unit, Unit>,
    next_number: Unit,
    turn: Unit,
}

impl Memory {
    // They begin by taking turns reading from a list of starting numbers (your puzzle input).
    pub fn with_starting_numbers(starting_numbers: &[Unit]) -> Self {
        let numbers: HashMap<_, _> = starting_numbers[..starting_numbers.len() - 1]
            .iter()
            .enumerate()
            .map(|(i, &n)| (n, i + 1))
            .collect();
        let last_number = *starting_numbers.last().unwrap();
        let turn = starting_numbers.len();
        Self {
            numbers,
            next_number: last_number,
            turn,
        }
    }

    // Then, each turn consists of considering the most recently spoken number:
    //   If the number had been spoken before; the current player announces how many turns apart the number is from when it was previously spoken.
    //   Otherwise that was the first time the number has been spoken, the current player says 0.

    pub fn next(&mut self) {
        self.next_number = if let Some(last_seen) = self.numbers.insert(self.next_number, self.turn)
        {
            let ls = last_seen;
            self.turn - ls
        } else {
            0
        };
        self.turn += 1;
    }

    pub fn run_until(&mut self, last: usize) -> Option<Unit> {
        let mut result = None;
        while self.turn < last {
            self.next();
            result = Some(self.next_number);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_day1() {
        // For example, suppose the starting numbers are 0,3,6:
        // Turn 1: The 1st number spoken is a starting number, 0.
        // Turn 2: The 2nd number spoken is a starting number, 3.
        // Turn 3: The 3rd number spoken is a starting number, 6.
        let mut memory = Memory::with_starting_numbers(&[0, 3, 6]);

        // Turn 4: Now, consider the last number spoken, 6. Since that was the first time the number
        //   had been spoken, the 4th number spoken is 0.
        // Turn 5: Next, again consider the last number spoken, 0. Since it had been spoken before,
        //   the next number to speak is the difference between the turn number when it was last
        //   spoken (the previous turn, 4) and the turn number of the time it was most recently
        //   spoken before then (turn 1). Thus, the 5th number spoken is 4 - 1, 3.
        // Turn 6: The last number spoken, 3 had also been spoken before, most recently on turns
        //   5 and 2. So, the 6th number spoken is 5 - 2, 3.
        // Turn 7: Since 3 was just spoken twice in a row, and the last two turns are 1 turn apart,
        //   the 7th number spoken is 1.
        // Turn 8: Since 1 is new, the 8th number spoken is 0.
        // Turn 9: 0 was last spoken on turns 8 and 4, so the 9th number spoken is the difference
        //   between them, 4.
        // Turn 10: 4 is new, so the 10th number spoken is 0.
        let expected_next_numbers = [0, 3, 3, 1, 0, 4, 0];
        for &expected in expected_next_numbers.iter() {
            memory.next();
            assert_eq!(memory.next_number, expected);
        }
        // Their question for you is: what will be the 2020th number spoken?
        // In the example above, the 2020th number spoken will be 436.
        assert_eq!(436, memory.run_until(2020).unwrap());
    }

    #[test]
    fn test_part1_additional_examples() {
        // Here are a few more examples:
        //     Given the starting numbers 1,3,2, the 2020th number spoken is 1.
        //     Given the starting numbers 2,1,3, the 2020th number spoken is 10.
        //     Given the starting numbers 1,2,3, the 2020th number spoken is 27.
        //     Given the starting numbers 2,3,1, the 2020th number spoken is 78.
        //     Given the starting numbers 3,2,1, the 2020th number spoken is 438.
        //     Given the starting numbers 3,1,2, the 2020th number spoken is 1836.
        let examples = [
            ([1, 3, 2], 1),
            ([2, 1, 3], 10),
            ([1, 2, 3], 27),
            ([2, 3, 1], 78),
            ([3, 2, 1], 438),
            ([3, 1, 2], 1836),
        ];

        for (start_values, expected) in &examples {
            let mut memory = Memory::with_starting_numbers(&start_values[..]);
            assert_eq!(*expected, memory.run_until(2020).unwrap());
        }
    }
}
