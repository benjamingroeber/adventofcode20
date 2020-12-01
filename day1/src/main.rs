use itertools::Itertools;
use helpers::read_line_usize_from_file;

fn main() {
    let numbers = read_line_usize_from_file("assets/days/day1.txt").unwrap();
    // Part 1
    // Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.
    day1(&numbers, 2, 2020);
    // Part 2
    // Find _three_ numbers in your expense report that meet the same criteria
    day1(&numbers, 3, 2020);
}

fn day1(input: &[usize], n: usize, target: usize) {
    // Get first n values resulting in sum equals target
    if let Some(values) = n_combination_target_sum(input, target, n).next() {
        // Multiply resulting values with each other
        let product: usize = values.iter().copied().product();
        println!("Sum of {:?} = {:?}", values, product);
    } else {
        println!("There are no {} numbers, resulting in {}", n, target);
    }
}

fn n_combination_target_sum(
    input: &[usize],
    target: usize,
    n: usize,
) -> impl Iterator<Item = Vec<&usize>> {
    input
        .iter()
        .combinations(n)
        // filter all combinations, where the sum of the elements is equal to `target`
        .filter(move |values| values.iter().copied().sum::<usize>() == target)
}