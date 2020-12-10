use helpers::parse_lines_file;
use itertools::Itertools;
use std::collections::HashMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let adapters: Vec<Jolts> = parse_lines_file("./assets/days/day10.txt")?;

    let adapters = SortedAdapters::new(adapters);
    let rating = adapters.chain_output_rating();
    let (sum1, sum3) = adapters.count_1_3_chain_differences();
    println!(
        "Jolt Rating: {:?}, 1s: {}, 3s: {}, product = {}",
        rating,
        sum1,
        sum3,
        sum1 * sum3
    );

    let combinations = adapters.count_possible_valid_combinations();
    println!("Possible Adapter Combinations: {}", combinations);
    Ok(())
}

type Jolts = u32;
const MAX_DIFF: Jolts = 3;

#[derive(Clone, Debug)]
struct SortedAdapters {
    adapters: Vec<Jolts>,
}

impl SortedAdapters {
    fn new(mut adapters: Vec<Jolts>) -> Self {
        // Treat the charging outlet near your seat as having an effective joltage rating of 0.
        adapters.push(0);
        adapters.sort_unstable();
        // With these adapters, your device's built-in joltage adapter would be rated for
        // 19 + 3 = 22 jolts, 3 higher than the highest-rated adapter.
        if let Some(highest) = adapters.last().copied() {
            adapters.push(highest + MAX_DIFF)
        }
        let a = SortedAdapters { adapters };
        // TODO Some error handling
        if !a.is_chain_possible() {
            panic! {"something went wrong there is a gap bigger than 3 jolts"}
        }
        a
    }

    fn chain_output_rating(&self) -> Option<Jolts> {
        self.adapters.last().copied()
    }

    fn chain_jolt_differences(&self) -> impl Iterator<Item = Jolts> + '_ {
        self.adapters.iter().tuple_windows().map(|(a, b)| b - a)
    }

    fn is_chain_possible(&self) -> bool {
        self.chain_jolt_differences()
            .all(|diff| diff > 0 && diff <= MAX_DIFF)
    }

    fn count_1_3_chain_differences(&self) -> (usize, usize) {
        self.chain_jolt_differences()
            .fold((0, 0), |(mut acc_1, mut acc_3), v| {
                match v {
                    1 => acc_1 += 1,
                    3 => acc_3 += 1,
                    _ => {}
                };
                (acc_1, acc_3)
            })
    }

    fn count_possible_valid_combinations(&self) -> u64 {
        let mut computed_combinations: HashMap<Jolts, u64> = HashMap::new();

        for &jolts in self.adapters.iter() {
            // edge case, single arrangement possible for the outlet
            if jolts == 0 {
                computed_combinations.insert(0, 1);
                continue;
            }

            // we've got no duplicates, and a sorted Vec, such that going forward we will always have
            // the values of previous adapters computed.
            // Based on this we can compute the possible combinations for the current adapter, by
            // summing combinations for the previous 1-3 adapters.
            let combinations = (1..=3)
                .map(|offset| {
                    // as long as jolts >= offset, we're always getting a positive number
                    if jolts >= offset {
                        let prev = jolts - offset;
                        *computed_combinations.get(&prev).unwrap_or(&0)
                    } else {
                        0
                    }
                })
                .sum();

            computed_combinations.insert(jolts, combinations);
        }
        self.adapters
            .last()
            .map(|last| computed_combinations[last])
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example_short() {
        let adapters = SortedAdapters::new(
            parse_lines_file("../assets/days/day10_example_short.txt").unwrap(),
        );

        //With these adapters, your device's built-in joltage adapter would be rated for
        // 19 + 3 = 22 jolts, 3 higher than the highest-rated adapter.
        assert_eq!(adapters.chain_output_rating(), Some(22));

        // In this example, when using every adapter, there are 7 differences of 1 jolt and 5
        // differences of 3 jolts.
        assert_eq!((7, 5), adapters.count_1_3_chain_differences())
    }

    #[test]
    fn test_part2_examples() {
        let short_example = SortedAdapters::new(
            parse_lines_file("../assets/days/day10_example_short.txt").unwrap(),
        );
        let long_example =
            SortedAdapters::new(parse_lines_file("../assets/days/day10_example_long.txt").unwrap());

        // Given the adapters from the first example, the total number of arrangements that connect
        // the charging outlet to your device is 8.
        let combinations_short = short_example.count_possible_valid_combinations();

        // In total, this set of adapters can connect the charging outlet to your device in 19208
        // distinct arrangements.
        let combinations_long = long_example.count_possible_valid_combinations();

        assert_eq!(8, combinations_short);
        assert_eq!(19208, combinations_long)
    }
}
