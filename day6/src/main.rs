use helpers::read_file;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    let input = read_file("./assets/days/day6.txt")?;
    let mut sum_any = 0;
    let mut sum_all = 0;

    // Each group's answers are separated by a blank line, and within each group,
    // each person's answers are on a single line.
    for group in input.split("\n\n") {
        // The form asks a series of 26 yes-or-no questions marked 'a' through 'z'.
        let possible_answers = 'a'..='z';
        // Part 1
        // All you need to do is identify the questions for which anyone in your group answers "yes".
        let any_yes = possible_answers
            .clone()
            .filter(|answer| group.lines().any(|form| form.contains(*answer)))
            // For each group, count the number of questions to which anyone answered "yes".
            // What is the sum of those counts?
            .count();
        sum_any += any_yes;
        // Part 2
        // you need to identify the questions to which everyone answered "yes"!
        let all_yes = possible_answers
            .filter(|answer| group.lines().all(|form| form.contains(*answer)))
            // For each group, count the number of questions to which everyone answered "yes".
            // What is the sum of those counts?
            .count();
        sum_all += all_yes;
    }

    println!("Sum of yes answers on at least one form per group: {}", sum_any);
    println!("Sum of yes answers on each form per group: {}", sum_all);
    Ok(())
}
