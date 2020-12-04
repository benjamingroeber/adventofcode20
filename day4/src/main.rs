use crate::PassportError::MissingTokenPart;
use crate::Token::{BirthYear, CountryID, ExpirationYear, IssueYear};
use helpers::read_file;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use thiserror::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_file("assets/days/day4.txt")?;
    let passports = parse_passports(&input)?;

    // Part 1
    // Count the number of valid passports - those that have all required fields.
    // Treat cid as optional. In your batch file, how many passports are valid?
    println!(
        "Valid passports ignoring Country ID {}",
        passports
            .iter()
            .filter(|p| p.has_all_fields_ignore_cid())
            .count()
    );

    // Part 2
    // Your job is to count the passports where all required fields are both present and valid
    // according to the given rules. Here are some example values:
    println!(
        "Valid passports with day2 rules {}",
        passports
            .iter()
            .filter(|p| p.is_satisfying_rules_ignore_cid())
            .count()
    );

    Ok(())
}

// Each passport is represented as a sequence of key:value pairs separated by spaces or newlines. Passports are separated by blank lines.
fn parse_passports(input: &str) -> Result<Vec<Passport>, PassportError> {
    let mut passports = Vec::new();
    let mut passport = Passport::default();
    for x in input.lines().flat_map(|word| word.split(' ')) {
        if !x.is_empty() {
            passport.set(x.try_into()?);
        } else {
            passports.push(passport);
            passport = Passport::default();
        }
    }

    // push last passport if not empty
    if passport != Passport::default() {
        passports.push(passport);
    }

    Ok(passports)
}

type Year = u16;

#[derive(Copy, Clone, Debug)]
enum Token<'a> {
    BirthYear(Year),
    IssueYear(Year),
    ExpirationYear(Year),
    Height(Height<'a>),
    HairColor(HairColor<'a>),
    EyeColor(EyeColor<'a>),
    PassportID(PassportID<'a>),
    CountryID(&'a str),
}

impl<'a> TryFrom<&'a str> for Token<'a> {
    type Error = PassportError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut parts = value.splitn(2, ':');
        let key = parts
            .next()
            .ok_or_else(|| MissingTokenPart(value.to_string()))?;
        let value = parts
            .next()
            .ok_or_else(|| MissingTokenPart(value.to_string()))?;

        let token = match key {
            "byr" => Ok(BirthYear(value.parse().unwrap())),
            "iyr" => Ok(IssueYear(value.parse().unwrap())),
            "eyr" => Ok(ExpirationYear(value.parse().unwrap())),
            "hgt" => Ok(Token::Height(value.into())),
            "hcl" => Ok(Token::HairColor(value.into())),
            "ecl" => Ok(Token::EyeColor(value.into())),
            "pid" => Ok(Token::PassportID(value.into())),
            "cid" => Ok(CountryID(value)),
            _ => Err(PassportError::UnknownTokenKey(
                key.to_string(),
                value.to_string(),
            )),
        };

        Ok(token?)
    }
}

#[derive(Error, Debug)]
pub enum PassportError {
    #[error("missing part of token in `{0}`")]
    MissingTokenPart(String),
    #[error("unknown token key {0} in `{1}`")]
    UnknownTokenKey(String, String),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Height<'a> {
    Cm(u16),
    In(u16),
    Unknown(&'a str),
}

//     hgt (Height) - a number followed by either cm or in
impl<'a> From<&'a str> for Height<'a> {
    fn from(value: &'a str) -> Self {
        let suffix = &value[value.len() - 2..];

        value[..value.len() - 2]
            .parse::<u16>()
            .map(|prefix| match suffix {
                "in" => Height::In(prefix),
                "cm" => Height::Cm(prefix),
                _ => Height::Unknown(value),
            })
            .unwrap_or(Height::Unknown(value))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum HairColor<'a> {
    Valid(&'a str),
    Invalid(&'a str),
}

impl<'a> HairColor<'a> {
    fn is_valid(&self) -> bool {
        match self {
            HairColor::Valid(_) => true,
            HairColor::Invalid(_) => false,
        }
    }

    fn is_valid_color(color: &str) -> bool {
        color.starts_with('#')
            && color.len() == 7
            && color[1..].chars().all(|c| c.is_ascii_hexdigit())
    }
}

// a # followed by exactly six characters 0-9 or a-f.
impl<'a> From<&'a str> for HairColor<'a> {
    fn from(value: &'a str) -> Self {
        if HairColor::is_valid_color(value) {
            HairColor::Valid(value)
        } else {
            HairColor::Invalid(value)
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum EyeColor<'a> {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
    Unknown(&'a str),
}

impl<'a> EyeColor<'a> {
    fn is_valid(&self) -> bool {
        !matches!(self, EyeColor::Unknown(_))
    }
}

// exactly one of: amb blu brn gry grn hzl oth
impl<'a> From<&'a str> for EyeColor<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "amb" => EyeColor::Amber,
            "blu" => EyeColor::Blue,
            "brn" => EyeColor::Brown,
            "gry" => EyeColor::Gray,
            "grn" => EyeColor::Green,
            "hzl" => EyeColor::Hazel,
            "oth" => EyeColor::Other,
            _ => EyeColor::Unknown(value),
        }
    }
}

// pid (Passport ID)
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum PassportID<'a> {
    Valid(&'a str),
    Invalid(&'a str),
}

// a nine-digit number, including leading zeroes.
impl<'a> From<&'a str> for PassportID<'a> {
    fn from(value: &'a str) -> Self {
        if value.chars().count() == 9 && value.chars().all(|c| c.is_numeric()) {
            PassportID::Valid(value)
        } else {
            PassportID::Invalid(value)
        }
    }
}

impl<'a> PassportID<'a> {
    fn is_valid(&self) -> bool {
        matches!(self, PassportID::Valid(_))
    }
}

// The expected fields are as follows:
//     byr (Birth Year)
//     iyr (Issue Year)
//     eyr (Expiration Year)
//     hgt (Height)
//     hcl (Hair Color)
//     ecl (Eye Color)
//     pid (Passport ID)
//     cid (Country ID)
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Passport<'a> {
    // Birth Year
    byr: Option<Year>,
    // Issue Year
    iyr: Option<Year>,
    // Expiration Year
    eyr: Option<Year>,
    // Height
    hgt: Option<Height<'a>>,
    // Hair Color
    hcl: Option<HairColor<'a>>,
    // Eye Color
    ecl: Option<EyeColor<'a>>,
    // Passport ID
    pid: Option<PassportID<'a>>,
    // Country ID
    cid: Option<&'a str>,
}

impl<'a> Passport<'a> {
    fn set(&mut self, token: Token<'a>) {
        match token {
            BirthYear(byr) => self.byr = Some(byr),
            IssueYear(iyr) => self.iyr = Some(iyr),
            ExpirationYear(eyr) => self.eyr = Some(eyr),
            Token::Height(hgt) => self.hgt = Some(hgt),
            Token::HairColor(hcl) => self.hcl = Some(hcl),
            Token::EyeColor(ecl) => self.ecl = Some(ecl),
            Token::PassportID(pid) => self.pid = Some(pid),
            CountryID(cid) => self.cid = Some(cid),
        }
    }

    // The third passport is interesting; the only missing field is cid, so it looks like data from
    // North Pole Credentials, not a passport at all! Surely, nobody would mind if you made the
    // system temporarily ignore missing cid fields.
    fn has_all_fields_ignore_cid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    // You can continue to ignore the cid field, but each other field has strict rules about what
    // values are valid for automatic validation
    fn is_satisfying_rules_ignore_cid(&self) -> bool {
        //     byr (Birth Year) - four digits; at least 1920 and at most 2002.
        is_year_between(self.byr, 1920, 2002)
        //     iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        && is_year_between(self.iyr, 2010, 2020)
        //     eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        && is_year_between(self.eyr,2020, 2030 )
        //     hgt (Height)
        //         If cm, the number must be at least 150 and at most 193.
        //         If in, the number must be at least 59 and at most 76.
        && self.hgt.map_or(false, |height|match height {
            Height::Cm(cm) => (150..=193).contains(&cm),
            Height::In(inch) => (59..=76).contains(&inch),
            Height::Unknown(_) => false
        })
        //     hcl (Hair Color)
        && self.hcl.map_or(false, |hcl|hcl.is_valid())
        //     ecl (Eye Color)
        && self.ecl.map_or(false, |ecl|ecl.is_valid())
        //     pid (Passport ID) - a nine-digit number, including leading zeroes.
        && self.pid.map_or(false, |pid|pid.is_valid())
        //     cid (Country ID) - ignored, missing or not.
    }
}

fn is_year_between(year: Option<Year>, min: Year, max: Year) -> bool {
    year.map_or(false, |y| (min..=max).contains(&y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_example() {
        let input = read_file("../assets/days/day4_p1_example.txt").unwrap();
        let passports = parse_passports(&input).unwrap();

        assert_eq!(passports.len(), 4);
        // The first passport is valid - all eight fields are present.
        assert!(passports[0].has_all_fields_ignore_cid());
        //  The second passport is invalid - it is missing hgt (the Height field).
        assert!(!passports[1].has_all_fields_ignore_cid());
        // The third passport is interesting; the only missing field is cid, so it looks like data
        // from North Pole Credentials, not a passport at all! Surely, nobody would mind if you made
        // the system temporarily ignore missing cid fields. Treat this "passport" as valid.
        assert!(passports[2].has_all_fields_ignore_cid());
        // The fourth passport is missing two fields, cid and byr. Missing cid is fine, but missing
        // any other field is not, so this passport is invalid.
        assert!(!passports[3].has_all_fields_ignore_cid())
    }

    #[test]
    fn test_day2_example_valid() {
        let input = read_file("../assets/days/day4_p2_example_valid.txt").unwrap();
        let passports = parse_passports(&input).unwrap();

        for p in passports {
            assert!(p.is_satisfying_rules_ignore_cid())
        }
    }

    #[test]
    fn test_day2_example_invalid() {
        let input = read_file("../assets/days/day4_p2_example_invalid.txt").unwrap();
        let passports = parse_passports(&input).unwrap();

        for p in passports {
            assert!(!p.is_satisfying_rules_ignore_cid())
        }
    }

    #[test]
    fn test_parse_height() {
        let height: Height = "60in".into();
        assert!(match height {
            Height::In(v) => v == 60,
            _ => false,
        });

        let height: Height = "190cm".into();
        assert!(match height {
            Height::Cm(v) => v == 190,
            _ => false,
        });

        let height: Height = "190in".into();
        assert!(match height {
            Height::In(v) => v == 190,
            _ => false,
        });

        let height: Height = "190".into();
        assert!(match height {
            Height::Unknown(v) => v == "190",
            _ => false,
        });
    }

    #[test]
    fn test_parse_hair_color() {
        // hcl valid:   #123abc
        let hcl: HairColor = "#123abc".into();
        assert!(hcl.is_valid());
        // hcl invalid: #123abz
        let hcl: HairColor = "#123abz".into();
        assert!(!hcl.is_valid());
        // hcl invalid: 123abc
        let hcl: HairColor = "123abc".into();
        assert!(!hcl.is_valid());
    }

    #[test]
    fn test_parse_eye_color() {
        // ecl valid:   brn
        let ecl: EyeColor = "brn".into();
        assert!(ecl.is_valid());
        // ecl invalid: wat
        let ecl: EyeColor = "wat".into();
        assert!(!ecl.is_valid())
    }

    #[test]
    fn test_parse_passport_id() {
        // pid valid:   000000001
        let pid: PassportID = "000000001".into();
        assert!(pid.is_valid());
        // pid invalid: 0123456789
        let pid: PassportID = "0123456789".into();
        assert!(!pid.is_valid())
    }
}
