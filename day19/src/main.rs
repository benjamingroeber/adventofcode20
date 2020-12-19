use helpers::{read_file, split_once};
use std::error::Error;
use std::collections::HashMap;
use thiserror::Error;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_file("./assets/days/day19.txt")?;
    let (rules, messages) = split_once(&input, "\n\n");
    let v = Validator::from_str(rules)?;

    let count = messages.lines().filter(|message|v.is_match(message)).count();
    println!("Number of Matching Messages: {}", count);

    // Part 2:
    // As you look over the list of messages, you realize your matching rules aren't quite right.
    // To fix them, completely replace rules 8: 42 and 11: 42 31 with the following:
    // 8: 42 | 42 8
    // 11: 42 31 | 42 11 31
    let input_p2 = read_file("./assets/days/day19_part2.txt")?;
    let (rules_p2, _) = split_once(&input_p2, "\n\n");
    let v = Validator::from_str(rules_p2)?;

    let count = messages.lines().filter(|message|v.is_match(message)).count();
    println!("Number of Matching Messages with loops: {}", count);
    Ok(())
}

#[derive(Clone, Debug)]
struct Or {
    rules: Vec<Pattern>
}

#[derive(Clone, Debug)]
struct Validator {
    rules: Rules
}

impl FromStr for Validator {
    type Err = ValidatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rules = parse_rules(s)?;
        // Sanity check, guarantees rule 0 to be there
        rules.get(&0).ok_or(ValidatorError::MissingRule0)?;
        Ok(Self{rules})
    }
}

impl Validator {
    pub fn is_match(&self, s: &str) -> bool {
        let rule0 = self.rules.get(&0).unwrap();
        self.match_pattern(rule0, &vec![s]).iter().any(|rest|*rest == "")
    }

    fn match_pattern<'a>(&self, pattern: &Pattern, candidates: &Vec<&'a str>) -> Vec<&'a str> {
        let mut new_candidates = Vec::new();
        for s in candidates {
            if *s == "" {
                new_candidates.push("");
                continue
            }
            match pattern {
                Pattern::Symbol(c) => {
                    s.chars().next().map(|next| {
                        if *c == next {
                            new_candidates.push(&s[c.len_utf8()..]);
                        }
                    });
                },
                Pattern::Rule(id) => {
                    // TODO is map_or false logically correct?
                    let pat = self.rules.get(id).unwrap();
                    let mut cs = self.match_pattern(pat, &candidates);
                    new_candidates.append(&mut cs);
                }
                Pattern::And(rs) => {
                    let mut rest = candidates.clone();
                    for pat in rs {
                        rest = self.match_pattern(pat, &rest);
                    }
                    new_candidates.append(&mut rest);
                }
                Pattern::Or(rs) => {
                    for pat in rs {
                        let mut mine = self.match_pattern(pat, candidates);
                        new_candidates.append(&mut mine)
                    }
                }
            };
        }
        new_candidates
    }
}

#[derive(Clone, Debug)]
enum Pattern {
    Symbol(char),
    Rule(Id),
    And(Vec<Pattern>),
    Or(Vec<Pattern>)
}

type Id = u32;
type Rules = HashMap<Id, Pattern>;

fn parse_rules(s: &str) -> Result<Rules, ValidatorError> {
    s.lines().map(|line| {
        let (id, rules) = split_once(line, ": ");
        let rule = if rules.contains('|') {
            let or = rules.split('|').map(|s|parse_pattern(s)).collect::<Result<Vec<_>,_>>()?;
            Pattern::Or(or)
        } else {
            parse_pattern(rules)?
        };
        id.parse().map(|id| (id, rule)).map_err(ValidatorError::from)
    }).collect()
}

fn parse_pattern(input: &str) -> Result<Pattern, ValidatorError> {
    let result : Result<Vec<_>,_> = input.trim().split(' ').map(|s| {
        if s.starts_with("\"") {
            if s.ends_with("\"") && s.chars().count() == 3 {
                let char = s.chars().skip(1).next().unwrap();
                Ok (Pattern::Symbol(char))
            } else {
                Err(ValidatorError::ParseSymbol(s.to_string()))
            }
        } else if s.chars().all(|c|c.is_numeric()) {
            s.parse().map(|id| Pattern::Rule(id)).map_err(|e|e.into())
        } else {
            Err(ValidatorError::UnknownPattern(s.to_string()))
        }
    }).collect();
    Ok(result.map(|v| if v.len() == 1 {
        v.first().cloned().unwrap()
    } else {
        Pattern::And( v )
    })?)
}

#[derive(Clone, Debug, Error)]
enum ValidatorError {
    #[error("unexpected symbol format in {0}")]
    ParseSymbol(String),
    #[error("could not parse pattern id")]
    ParseId(#[from] ParseIntError),
    #[error("unknown pattern in {0}")]
    UnknownPattern(String),
    #[error("no rule 0")]
    MissingRule0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = read_file("../assets/days/day19_example.txt").unwrap();
        let (rules, _) = split_once(&input, "\n\n");
        let v = Validator::from_str(rules).unwrap();

        //In the above example, ababbb and abbbab match, but bababa, aaabbb, and aaaabbb do not
        assert!(v.is_match("ababbb"));
        assert!(v.is_match("abbbab"));
        assert!(!v.is_match("bababa"));
        assert!(!v.is_match("aaabbb"));
        assert!(!v.is_match("aaaabbb"));
    }
}
/*
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"*/