use helpers::read_file;
use std::collections::HashMap;
use std::error::Error;
use thiserror::Error;

static SHINY_GOLD: &str = "shiny gold";

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_file("./assets/days/day7.txt")?;

    let rules = parse_rules(&input)?;
    // Part 1: How many bag colors can eventually contain at least one shiny gold bag?
    println!(
        "{} bags eventually contain any gold bags.",
        rules.count_can_reach_color(SHINY_GOLD)
    );

    // Part 2: How many individual bags are required inside your single shiny gold bag?
    println!(
        "{} bags inside a single gold bag.",
        rules.count_contained_bags(SHINY_GOLD)?
    );

    Ok(())
}

#[derive(Clone, Debug)]
struct Rules<'a> {
    rules: HashMap<&'a str, Vec<(usize, &'a str)>>,
}

impl<'a> Rules<'a> {
    // Returns the count of top level bags types, which eventually contain the bag of the requested color
    fn count_can_reach_color(&self, color: &'a str) -> usize {
        self.rules
            .iter()
            .filter(|r| self.can_reach(r.0, color))
            .count()
    }
    fn can_reach(&self, start: &str, target: &str) -> bool {
        // unknown start items count as not reachable
        self.rules.get(start).map_or(false, |children| {
            children
                .iter()
                // based on lazy evaluation, either target is a reachable, or continue searching
                // in the children of the target
                .any(|(_, color)| *color == target || self.can_reach(color, target))
        })
    }

    // Count amount of bags INSIDE the given color
    fn count_contained_bags(&self, color: &str) -> Result<usize, BagError> {
        // we can't count the gold bag itself at the first level
        self.count_bags(color).map(|count| count - 1)
    }

    // Count amount of nested bags
    // Note that the first level counts as 1 bag, see count_contained_bags
    fn count_bags(&self, color: &str) -> Result<usize, BagError> {
        let mut sum = 1;
        for (count, color) in self
            .rules
            .get(color)
            .ok_or_else(|| BagError::UnknownBagColor(color.to_string()))?
        {
            sum += count * self.count_bags(color)?
        }
        Ok(sum)
    }
}

fn parse_rules(input: &str) -> Result<Rules, BagError> {
    let mut rules: HashMap<&str, Vec<(usize, &str)>> = HashMap::new();
    for definition in input.lines().map(|line| line.trim_end_matches('.')) {
        let split_at = definition
            .find(SEPARATOR)
            .ok_or_else(|| BagError::SplitParts(definition.to_string()))?;
        let (color, suffix) = definition.split_at(split_at);
        let content = suffix.split_at(SEPARATOR_LEN).1;
        let bags = parse_content(content)?;
        // if there is already an entry there went something wrong
        if rules.insert(color, bags).is_some() {
            return Err(BagError::DuplicateRule(color.to_string()));
        }
    }

    Ok(Rules { rules })
}

static SEPARATOR: &str = " bags contain ";
static SEPARATOR_LEN: usize = 14;

static SINGULAR_SUFFIX: &str = " bag";
static PLURAL_SUFFIX: &str = " bags";

fn parse_content(content: &str) -> Result<Vec<(usize, &str)>, BagError> {
    let mut bags = Vec::new();
    if content == ("no other bags") {
        return Ok(bags);
    };

    for bag in content.split(", ") {
        let bag = remove_bag_suffix(bag)?;
        // split count and color
        let (count, color) = bag
            .find(' ')
            .and_then(|mid| {
                let (count, color) = bag.split_at(mid);
                // remove leading space
                let color = color.trim_start();
                Some((count, color))
            })
            .ok_or_else(|| BagError::SplitContent(bag.to_string()))?;
        let count: usize = count
            .parse()
            .map_err(|_| BagError::ParseCount(count.to_string()))?;
        bags.push((count, color));
    }
    Ok(bags)
}

fn remove_bag_suffix(value: &str) -> Result<&str, BagError> {
    if value.ends_with(PLURAL_SUFFIX) {
        Ok(value.trim_end_matches(PLURAL_SUFFIX))
    } else if value.ends_with(SINGULAR_SUFFIX) {
        Ok(value.trim_end_matches(SINGULAR_SUFFIX))
    } else {
        Err(BagError::UnknownBagSuffix(value.to_string()))
    }
}

#[derive(Error, Clone, Debug)]
enum BagError {
    #[error("could not split {0} at ' bags contain '")]
    SplitParts(String),
    #[error("could not bag contents {0} at ', '")]
    SplitContent(String),
    #[error("could get bag count from {0}")]
    ParseCount(String),
    #[error("duplicate rule for {0}")]
    DuplicateRule(String),
    #[error("unknown suffix for {0}, only ' bags' and ' bag' are allowed")]
    UnknownBagSuffix(String),
    #[error("unknown bag color {0}")]
    UnknownBagColor(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_suffix() {
        let expected = "light brown";

        let singular = remove_bag_suffix("light brown bag");
        let plural = remove_bag_suffix("light brown bags");
        let missing_space = remove_bag_suffix("light brownbags");
        let wrong_suffix = remove_bag_suffix("light brown gags");

        assert_eq!(expected, singular.unwrap());
        assert_eq!(expected, plural.unwrap());
        assert!(missing_space.is_err());
        assert!(wrong_suffix.is_err());
    }

    // In the above rules, the following options would be available to you:
    //
    //     A bright white bag, which can hold your shiny gold bag directly.
    //     A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
    //     A dark orange bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
    //     A light red bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
    // So, in this example, the number of bag colors that can eventually contain at least one shiny gold bag is 4.
    #[test]
    fn test_example_day1() {
        let input = read_file("../assets/days/day7_example.txt").unwrap();
        let rules = parse_rules(&input).unwrap();

        let can_hold_shiny_gold = rules.count_can_reach_color(SHINY_GOLD);
        assert_eq!(4, can_hold_shiny_gold);
    }

    // Consider again your shiny gold bag and the rules from the above example:
    //
    //     faded blue bags contain 0 other bags.
    //     dotted black bags contain 0 other bags.
    //     vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted black bags.
    //     dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black bags.
    //
    // So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags within it) plus 2 vibrant plum bags (and the 11 bags within each of those): 1 + 1*7 + 2 + 2*11 = 32 bags!
    #[test]
    fn test_example_day2() {
        let input = read_file("../assets/days/day7_example.txt").unwrap();
        let rules = parse_rules(&input).unwrap();

        let contained_bags = rules.count_contained_bags(SHINY_GOLD);

        assert_eq!(32, contained_bags.unwrap());
    }
}