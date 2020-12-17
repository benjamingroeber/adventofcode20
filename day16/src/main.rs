use helpers::read_file;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::ops::RangeInclusive;
use std::str::FromStr;
use thiserror::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_file("./assets/days/day16.txt")?;
    let ticket_translation = parse_ticket_translation(&input)?;

    // you can identify invalid nearby tickets by considering only whether tickets contain values
    // that are not valid for any field
    // Adding together all of the invalid values produces your ticket scanning error rate
    println!("Scan Error Rate:{}", ticket_translation.scan_error_rate());

    // Once you work out which field is which, look for the six fields on your ticket that start
    // with the word departure. What do you get if you multiply those six values together?
    let named_values = ticket_translation.valid_ticket_named_values()?;
    let product: Unit = named_values
        .iter()
        .filter(|(n, _)| n.starts_with("departure"))
        .map(|c| c.1)
        .product();
    println!("Product of 'departure.*' field: {:?}", product);

    Ok(())
}

type Unit = usize;

pub struct TicketTranslation<'a> {
    constraints: Constraints<'a>,
    ticket: Ticket,
    other_tickets: Vec<Ticket>,
    ticket_values: Unit,
}

impl<'a> TicketTranslation<'a> {
    pub fn scan_error_rate(&self) -> Unit {
        self.other_tickets
            .iter()
            .map(|ticket| self.constraints.scan_error_rate(ticket))
            .sum()
    }

    pub fn valid_tickets(&self) -> impl Iterator<Item = &Ticket> + '_ {
        self.other_tickets
            .iter()
            .filter(move |t| self.constraints.is_valid(t))
    }

    fn get_valid_field_values(&self, index: usize) -> impl Iterator<Item = Unit> + '_ {
        self.valid_tickets().map(move |ticket| ticket.values[index])
    }

    pub fn valid_ticket_named_values(&self) -> Result<HashMap<&str, Unit>, TicketError> {
        let mappings = self.get_mappings()?;
        Ok(self.ticket.named_values(&mappings))
    }

    pub fn candidate_mappings(&self) -> Result<Vec<HashSet<&str>>, TicketError> {
        let mut candidates = vec![HashSet::new(); self.ticket_values];

        // find candidates
        for (field, candidate) in candidates.iter_mut().enumerate().take(self.ticket_values) {
            // println!("UHHHHH {}", field);
            for (name, constraint) in &self.constraints.constraints {
                let fv: Vec<_> = self.get_valid_field_values(field).collect();

                if fv.iter().all(|f| constraint.matches_any_range(*f)) {
                    candidate.insert(*name);
                }
            }
        }
        Ok(candidates)
    }

    pub fn get_mappings(&self) -> Result<HashMap<&str, usize>, TicketError> {
        let mut candidates = self.candidate_mappings()?;

        // find assignments with one possible field, and remove the field from all other possibilities
        let mut result = HashMap::new();
        loop {
            let unique_mapping = candidates
                .iter()
                .enumerate()
                .find(|(_, c)| c.len() == 1)
                .map(|c| c.0);
            if let Some(index) = unique_mapping {
                let name = *candidates[index].iter().next().unwrap();
                result.insert(name, index);
                for c in candidates.iter_mut() {
                    c.remove(name);
                }
            } else {
                // no further mappings possible
                // sanity check for mapping
                if let Some(c) = candidates
                    .iter()
                    .enumerate()
                    .find(|c| !c.1.is_empty())
                    .map(|c| c.0)
                {
                    return Err(TicketError::AmbiguousFieldMapping(c));
                }
                break;
            }
        }

        Ok(result)
    }
}

pub fn parse_ticket_translation(input: &str) -> Result<TicketTranslation, TicketError> {
    let mut sections = input.split("\n\n");
    if sections.clone().count() != 3 {
        return Err(TicketError::UnexpectedFormat);
    }
    let constraints = sections
        .next()
        .map(|s| parse_constraints(s))
        .ok_or(TicketError::UnexpectedFormat)??;

    // your ticket:
    // 7,1,14
    let ticket = sections
        .next()
        .map(|s| s.lines().skip(1).map(Ticket::from_str).next())
        .flatten()
        .ok_or(TicketError::UnexpectedFormat)??;

    let other_tickets = sections
        .next()
        .map(|s| parse_nearby_tickets(s))
        .ok_or(TicketError::UnexpectedFormat)??;

    // sanity check
    let length = other_tickets
        .get(0)
        .map(|tickets| tickets.values.len())
        .unwrap_or(0);

    if !other_tickets.iter().all(|t| t.values.len() == length) {
        return Err(TicketError::IrregularTicketValues);
    }

    Ok(TicketTranslation {
        constraints,
        ticket,
        other_tickets,
        ticket_values: length,
    })
}

#[derive(Debug, Clone)]
struct Constraints<'a> {
    constraints: HashMap<&'a str, Constraint>,
}

impl<'a> Constraints<'a> {
    // Start by determining which tickets are completely invalid; these are tickets that contain
    // values which aren't valid for any field.
    // It doesn't matter which position corresponds to which field; you can identify invalid nearby
    // tickets by considering only whether tickets contain values that are not valid for any field.
    // In this example, the values on the first nearby ticket are all valid for at least one field.
    // This is not true of the other three nearby tickets: the values 4, 55, and 12 are are not
    // valid for any field. Adding together all of the invalid values produces your ticket scanning
    // error rate: 4 + 55 + 12 = 71.
    fn scan_error_rate(&self, ticket: &Ticket) -> Unit {
        ticket
            .values
            .iter()
            .filter(|value| !self.matches_any(**value))
            .sum()
    }

    fn is_valid(&self, ticket: &Ticket) -> bool {
        ticket.values.iter().all(|value| self.matches_any(*value))
    }

    fn matches_any(&self, value: Unit) -> bool {
        self.constraints
            .iter()
            .any(|(_, c)| c.matches_any_range(value))
    }
}

#[derive(Clone, Debug)]
struct Constraint {
    ranges: Vec<RangeInclusive<Unit>>,
}

impl Constraint {
    fn matches_any_range(&self, value: Unit) -> bool {
        self.ranges.iter().any(|range| range.contains(&value))
    }
}

// class: 1-3 or 5-7
// row: 6-11 or 33-44
// seat: 13-40 or 45-50
fn parse_constraints(s: &str) -> Result<Constraints, TicketError> {
    let constraints: Result<HashMap<_, _>, _> = s
        .lines()
        .map(|line| {
            if let Some(split_at) = line.find(NAME_SEPARATOR) {
                let name = &line[..split_at];
                let rules = &line[split_at + NAME_SEPARATOR.len()..];
                Constraint::from_str(rules).map(|constraint| (name, constraint))
            } else {
                Err(TicketError::UnknownConstraint(line.to_string()))
            }
        })
        .collect();
    Ok(Constraints {
        constraints: constraints?,
    })
}

static FIELD_SEPARATOR: char = ',';
#[derive(Clone, Debug)]
pub struct Ticket {
    values: Vec<Unit>,
}

impl FromStr for Ticket {
    type Err = TicketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Result<Vec<_>, _> = s
            .split(FIELD_SEPARATOR)
            .map(|u| Unit::from_str(u).map_err(|_| TicketError::NumberParser(u.to_string())))
            .collect();
        Ok(Ticket { values: values? })
    }
}

impl Ticket {
    fn named_values<'a>(&'a self, mapping: &HashMap<&'a str, usize>) -> HashMap<&'a str, usize> {
        mapping
            .iter()
            .map(|(name, idx)| (*name, self.values[*idx]))
            .collect()
    }
}

// nearby tickets:
// 7,3,47
// 40,4,50
// 55,2,20
// 38,6,12
fn parse_nearby_tickets(s: &str) -> Result<Vec<Ticket>, TicketError> {
    s.lines()
        .skip(1)
        .map(|line| Ticket::from_str(line))
        .collect()
}

static NAME_SEPARATOR: &str = ": ";
static RANGE_SEPARATOR: &str = " or ";
static MIN_MAX_SEPARATOR: &str = "-";
impl FromStr for Constraint {
    type Err = TicketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges = s
            .split(RANGE_SEPARATOR)
            .map(|range| parse_range(range))
            .collect::<Result<Vec<_>, _>>();
        Ok(Constraint { ranges: ranges? })
    }
}

// format is [0-9]+-[0-9]+ e.g. 0-100
fn parse_range(s: &str) -> Result<RangeInclusive<Unit>, TicketError> {
    if let Some((min, max)) = s
        .find(MIN_MAX_SEPARATOR)
        .map(|idx| s.split_at(idx))
        // skip - in max number
        .map(|(min, max)| (min, &max[1..]))
    {
        match min.parse::<Unit>() {
            Ok(min) => match max.parse::<Unit>() {
                Ok(max) => Ok(min..=max),
                Err(_) => Err(TicketError::NumberParser(max.to_string())),
            },
            Err(_) => Err(TicketError::NumberParser(min.to_string())),
        }
    } else {
        Err(TicketError::UnknownConstraint(s.to_string()))
    }
}

#[derive(Clone, Debug, Error)]
pub enum TicketError {
    #[error("unknown constraint {0}")]
    UnknownConstraint(String),
    #[error("parsing of number {0} failed")]
    NumberParser(String),
    #[error("unexpected input, rules, own ticket, other tickets are expected")]
    UnexpectedFormat,
    #[error("all tickets must have the same number of values")]
    IrregularTicketValues,
    #[error("field assignments are ambiguous, more than one possibility for column {0}")]
    AmbiguousFieldMapping(usize),
}

#[test]
fn test_example_part1() {
    let input = read_file("../assets/days/day16_example.txt").unwrap();
    let ticket_translation = parse_ticket_translation(&input).unwrap();

    // the values 4, 55, and 12 are are not valid for any field.
    // Adding together all of the invalid values produces your ticket scanning error rate:
    // 4 + 55 + 12 = 71.
    assert_eq!(ticket_translation.scan_error_rate(), 71)
}

#[test]
fn test_constraint_match_range() {
    let constraint = Constraint {
        ranges: vec![0..=10, 15..=20, 30..=40],
    };
    // must succeed
    for i in &[0, 5, 10, 15, 17, 20, 30, 35, 40] {
        assert!(constraint.matches_any_range(*i))
    }

    // must fail
    for i in &[11, 14, 21, 25, 29, 41] {
        assert!(!constraint.matches_any_range(*i))
    }
}

#[test]
fn test_example_part2() {
    let input = read_file("../assets/days/day16_example_part2.txt").unwrap();
    let ticket_translation = parse_ticket_translation(&input).unwrap();

    let mappings = ticket_translation.valid_ticket_named_values().unwrap();
    // Based on the nearby tickets in the above example, the first position must be row, the second
    // position must be class, and the third position must be seat; you can conclude that in your
    // ticket, class is 12, row is 11, and seat is 13.
    assert_eq!(mappings.get("row"), Some(&11));
    assert_eq!(mappings.get("class"), Some(&12));
    assert_eq!(mappings.get("seat"), Some(&13));
}