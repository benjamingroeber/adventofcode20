use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;

use thiserror::Error;
use std::num::ParseIntError;
use crate::PolicyError::ParsePolicy;

fn main() -> Result<(), Box<dyn Error>>{
    let input = helpers::read_file("assets/days/day2.txt")?;
    let policies: Result<Vec<_>, _> = input.lines().map(|l| parse_policy(l)).collect();
    let policies = policies?;
    println!("Fulfilled day1 policy count: {}", day1(&policies));
    println!("Fulfilled day2 policy count: {}", day2(&policies));
    Ok(())
}

// Each line gives the password policy and then the password. The password policy indicates the
// lowest and highest number of times a given letter must appear for the password to be valid.
//
// For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.
// How many passwords are valid according to their policies?
fn day1(policies: &[Policy]) -> usize {
    policies
        .iter()
        .filter(|p| p.is_day1_policy_fulfilled())
        .count()
}

// Each policy actually describes two positions in the password, where 1 means the first character,
// 2 means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept
// of "index zero"!) Exactly one of these positions must contain the given letter. Other occurrences
// of the letter are irrelevant for the purposes of policy enforcement.
fn day2(policies: &[Policy]) -> usize {
    policies
        .iter()
        .filter(|p| p.is_day2_policy_fulfilled())
        .count()
}

#[derive(Error, Debug)]
pub enum PolicyError {
    #[error("could not parse password policy from `{0}`")]
    ParsePolicy(String),
    #[error("could not parse positional argument")]
    ParseUint(#[from] ParseIntError),
    #[error("policy index out of bounds")]
    ValidateDay2Indexing,
    #[error("unknown password policy error")]
    Unknown,
}

#[derive(Debug, PartialEq, Eq)]
struct Policy<'a> {
    lower_bound: usize,
    upper_bound: usize,
    letter: char,
    password: &'a str,
}

impl<'a> Policy<'a> {
    fn is_day1_policy_fulfilled(&self) -> bool {
        let count = self.password.chars().filter(|c| *c == self.letter).count();
        count >= self.lower_bound && count <= self.upper_bound
    }

    fn is_day2_policy_fulfilled(&self) -> bool {
        let mut chars = self.password.chars();
        // first position must exist, or false is returned
        chars.nth(self.lower_bound - 1).map_or( false, |char_at_lower| {
            // reuse the existing iterator by calculating the distance from the first position to
            // the second
            let distance = self.upper_bound - self.lower_bound - 1;
            chars.nth(distance).map_or_else(
                // second position must exist, or only the first requirement is considered
                || char_at_lower == self.letter,
                // if both exist, __exactly__ one of them must match the letter
                |char_at_upper| {
                    (char_at_lower == self.letter || char_at_upper == self.letter)
                        && char_at_upper != char_at_lower
                },
            )
        })
    }
}

// Input each line contains `{lower_bound}-{upper_bound} {letter}: {password}`
fn parse_policy(line: &str) -> Result<Policy, Box<dyn Error>> {
    let dash = line.find("-").ok_or(ParsePolicy(line.to_owned()))?;
    let first_space = line.find(" ").ok_or(ParsePolicy(line.to_owned()))?;
    let colon = line.find(":").ok_or(ParsePolicy(line.to_owned()))?;

    let lower_bound = line[0..dash].parse::<usize>()?;
    let upper_bound = line[dash+1..first_space].parse::<usize>()?;
    let letter = line[first_space+1..colon].chars().next().ok_or(ParsePolicy(line.to_owned()))?;
    // need to skip the space after the colon
    let password = &line[colon+2..];

    let policy = Policy {
        lower_bound,
        upper_bound,
        letter,
        password,
    };
    Ok(policy)
}

// Input each line contains `{lower_bound}-{upper_bound} {letter}: {password}`
#[allow(dead_code)]
fn parse_policy_regex(line: &str) -> Result<Policy, PolicyError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"^(?P<lower_bound>[0-9]+)-(?P<upper_bound>[0-9]+)\s(?P<letter>[A-z]):\s(?P<password>.*)"#).unwrap();
    }

    let captures = RE.captures(line).ok_or_else(||PolicyError::ParsePolicy(line.to_owned()))?;
    assert_eq!(captures.len(), 5);

    let lower_bound: usize = captures.name("lower_bound").ok_or_else(||PolicyError::ParsePolicy(line.to_owned()))?.as_str().parse()?;
    let upper_bound: usize = captures.name("upper_bound").ok_or_else(||PolicyError::ParsePolicy(line.to_owned()))?.as_str().parse()?;
    let letter = captures
        .name("letter")
        .ok_or_else(||PolicyError::ParsePolicy(line.to_owned()))?
        .as_str()
        .chars()
        .next()
        .ok_or_else(||PolicyError::ParsePolicy(line.to_owned()))?;
    let password = captures.name("password").ok_or_else(||PolicyError::ParsePolicy(line.to_owned()))?.as_str();

    let policy = Policy {
        lower_bound,
        upper_bound,
        letter,
        password,
    };
    Ok(policy)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_policy() {
        let policy = parse_policy("10-20 a: password").unwrap();
        assert_eq!(policy, Policy{
            lower_bound: 10,
            upper_bound: 20,
            letter: 'a',
            password: "password"
        })
    }

    #[test]
    fn test_parse_policy_regex() {
        let policy = parse_policy_regex("10-20 a: password").unwrap();
        assert_eq!(
            policy,
            Policy {
                lower_bound: 10,
                upper_bound: 20,
                letter: 'a',
                password: "password"
            }
        )
    }

    #[test]
    fn test_day1_example() {
        // 1-3 a: abcde -> valid
        let e1 = Policy {
            lower_bound: 1,
            upper_bound: 3,
            letter: 'a',
            password: "abcde",
        };
        assert!(e1.is_day1_policy_fulfilled());

        // 1-3 b: cdefg -> invalid
        let e2 = Policy {
            lower_bound: 1,
            upper_bound: 3,
            letter: 'b',
            password: "cdefg",
        };
        assert!(!e2.is_day1_policy_fulfilled());

        // 2-9 c: ccccccccc -> valid
        let e3 = Policy {
            lower_bound: 2,
            upper_bound: 9,
            letter: 'c',
            password: "ccccccccc",
        };
        assert!(e3.is_day1_policy_fulfilled());
    }

    #[test]
    fn test_day2_example() {
        // 1-3 a: abcde -> valid
        let e1 = Policy {
            lower_bound: 1,
            upper_bound: 3,
            letter: 'a',
            password: "abcde",
        };
        assert!(e1.is_day2_policy_fulfilled());

        // 1-3 b: cdefg -> invalid
        let e2 = Policy {
            lower_bound: 1,
            upper_bound: 3,
            letter: 'b',
            password: "cdefg",
        };
        assert!(!e2.is_day2_policy_fulfilled());

        // 2-9 c: ccccccccc -> invalid
        let e3 = Policy {
            lower_bound: 2,
            upper_bound: 9,
            letter: 'c',
            password: "ccccccccc",
        };
        assert!(!e3.is_day2_policy_fulfilled());
    }
}
