use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use thiserror::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("./assets/days/day23.txt")?;
    let labels = parse_labels(&input)?;

    // Part 1
    {
        let mut cups = Cups::with_labels(&labels);
        cups.play(100);
        println!("Order: {}", cups.get_order_after_1());
    }

    // Part 2
    {
        let mut cups = Cups::with_labels_and_fill(&labels, 1_000_000);
        cups.play(10_000_000);

        // Under the two cups that will end up immediately clockwise of cup 1. You can have them if
        // you predict what the labels on those cups will be when the crab is finished.
        // What do you get if you  multiply their labels together?
        let (p1, p2) = cups.get_two_cups_after_one();
        println!("P1: {} P2: {} Product: {}", p1, p2, p1 * p2)
    }
    Ok(())
}

type Label = usize;
#[derive(Clone, Debug)]
struct Cup {
    label: usize,
    next: usize,
}

#[derive(Clone, Debug)]
struct Cups {
    cur_pos: usize,
    cups: Vec<Cup>,
}

impl Cups {
    fn with_labels(labels: &[Label]) -> Self {
        // expect all numbers from 1 to cups.len
        // fill 0 for easier indexing
        let mut cups = vec![Cup { label: 0, next: 0 }; labels.len() + 1];
        let mut numbers = labels.iter().peekable();
        let cur_pos = labels[0];
        while let Some(label) = numbers.next() {
            let next = if let Some(next) = &numbers.peek() {
                ***next
            } else {
                cur_pos
            };
            cups[*label] = Cup {
                label: *label,
                next,
            }
        }

        // check for all between min and max for wrapping around
        Cups { cur_pos, cups }
    }

    fn with_labels_and_fill(labels: &[Label], n: usize) -> Self {
        // initialize all values
        // here we're relying on the fact that labels contain all numbers from 1 to labels.len();
        let mut cups = vec![Cup { label: 0, next: 0 }; n + 1];

        let cur_pos = labels[0];
        let mut max = 0_usize;
        let mut numbers = labels.iter().peekable();
        while let Some(label) = numbers.next() {
            // track the maximum number to continue filling up from max +1
            if *label > max {
                max = *label
            }
            let next = if let Some(next) = &numbers.peek() {
                ***next
            } else {
                // the last entry should point to the first number filling up
                max + 1
            };
            cups[*label] = Cup {
                label: *label,
                next,
            }
        }
        // fill up until the desired number of cups is reached
        for i in max + 1..=n {
            cups[i] = Cup {
                label: i,
                next: i + 1,
            }
        }
        // adjust the last cup to point to the beginning
        cups[n].next = cur_pos;

        // check for all between min and max for wrapping around
        Cups { cur_pos, cups }
    }

    fn play(&mut self, n: usize) {
        for _ in 0..n {
            self.play_round()
        }
    }

    fn play_round(&mut self) {
        // The crab picks up the three cups that are immediately clockwise of the current cup. They
        // are removed from the circle; cup spacing is adjusted as necessary to maintain the circle.
        let c1 = self.get_next(self.cur_pos);
        let c2 = self.get_next(c1);
        let c3 = self.get_next(c2);
        self.cups[self.cur_pos].next = self.cups[c3].next;
        // The crab selects a destination cup: the cup with a label equal to the current cup's label
        // minus one.
        let mut destination_cup = self.get_previous_label(self.cur_pos);
        // If this would select one of the cups that was just picked up, the crab will
        // keep subtracting one until it finds a cup that wasn't just picked up.
        while destination_cup == c1 || destination_cup == c2 || destination_cup == c3 {
            destination_cup = self.get_previous_label(destination_cup);
        }

        // The crab places the cups it just picked up so that they are immediately clockwise of the
        // destination cup.
        let link_to = self.cups[destination_cup].next;
        self.cups[destination_cup].next = c1;
        self.cups[c3].next = link_to;

        // The crab selects a new current cup: the cup which is immediately clockwise of the current
        // cup.
        self.cur_pos = self.cups[self.cur_pos].next;
    }

    fn get_previous_label(&self, cur_pos: usize) -> usize {
        if cur_pos == 1 {
            self.cups.len() - 1
        } else {
            cur_pos - 1
        }
    }

    fn get_next(&self, cur_pos: usize) -> usize {
        self.cups[cur_pos].next
    }

    fn get_order_after_1(&self) -> usize {
        // After the crab is done, what order will the cups be in? Starting after the cup labeled 1,
        // collect the other cups' labels clockwise into a single string with no extra characters;
        // each number except 1 should appear exactly once.
        let mut next = self.get_next(1);
        let mut order = 0;
        while next != 1 {
            order = order * 10 + next;
            next = self.get_next(next)
        }
        order
    }

    // The crab is going to hide your stars - one each - under the two cups that will end up
    // immediately clockwise of cup 1. You can have them if you predict what the labels on those
    // cups will be when the crab is finished.
    // Determine which two cups will end up immediately clockwise of cup 1.
    fn get_two_cups_after_one(&self) -> (usize, usize) {
        let p1 = self.get_next(1);
        let p2 = self.get_next(p1);
        (p1, p2)
    }
}

impl Display for Cups {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut pos = self.cur_pos;
        while self.cups[pos].next != self.cur_pos {
            write!(f, "{} ", pos)?;
            pos = self.cups[pos].next;
        }
        write!(f, "{}", pos)?;
        Ok(())
    }
}

fn parse_labels(s: &str) -> Result<Vec<Label>, CupsError> {
    let labels: Option<Vec<Label>> = s
        .chars()
        .map(|c| c.to_digit(10).map(|d| d as usize))
        .collect();
    Ok(labels.ok_or(CupsError::ParseCup)?)
}

#[derive(Clone, Debug, Error)]
enum CupsError {
    #[error("could not parse cup ids")]
    ParseCup,
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "389125467";
    #[test]
    fn test_example_part1() {
        let labels = parse_labels(TEST_INPUT).unwrap();
        let mut cups = Cups::with_labels(&labels);
        // For example, suppose your cup labeling were 389125467.
        // If the crab were to do merely 10 moves, the following changes would occur:
        cups.play(10);

        let state = format!("{}", cups);
        // -- final --
        // cups:  5 (8) 3  7  4  1  9  2  6
        let expected = "8 3 7 4 1 9 2 6 5";
        assert_eq!(state, expected);

        // In the above example, after 10 moves,
        // the cups clockwise from 1 are labeled 9, 2, 6, 5, and so on, producing 92658374.
        // If the crab were to complete all 100 moves, the order after cup 1 would be 67384529.
        let order_10 = cups.get_order_after_1();
        assert_eq!(order_10, 92658374);
        cups.play(90);

        let order_100 = cups.get_order_after_1();
        assert_eq!(order_100, 67384529);
    }

    #[test]
    fn text_example_part2() {
        let labels = parse_labels(TEST_INPUT).unwrap();
        let mut cups = Cups::with_labels_and_fill(&labels, 1_000_000);

        cups.play(10_000_000);
        // In the above example (389125467), this would be 934001 and then 159792;
        let (p1, p2) = cups.get_two_cups_after_one();

        assert_eq!(p1, 934001);
        assert_eq!(p2, 159792);
        // multiplying these together produces 149245887792.
        assert_eq!(149245887792, p1 * p2);
    }
}
