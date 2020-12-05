use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;

#[allow(non_camel_case_types)]
type year = String;

#[allow(non_camel_case_types)]
type hex = String;

struct Passport {
    birth_year: Option<year>,
    issue_year: Option<year>,
    expiration_year: Option<year>,
    height: Option<String>,
    hair_color: Option<hex>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

#[derive(Debug, PartialEq)]
enum Height {
    CM(u64),
    IN(u64),
    NONE,
}

fn parse_height(h: &str) -> Height {
    lazy_static! {
        static ref CM_MATCHER: Regex = Regex::new(r"(?P<cm>\d+)cm").unwrap();
        static ref IN_MATCHER: Regex = Regex::new(r"(?P<in>\d+)in").unwrap();
    }

    let mut ret = Height::NONE;

    match CM_MATCHER.captures(h) {
        Some(v) => {
            ret = Height::CM(v.name("cm").unwrap().parse().unwrap());
        }
        None => (),
    }

    match IN_MATCHER.captures(h) {
        Some(v) => {
            ret = Height::IN(v.name("in").unwrap().parse().unwrap());
        }
        None => (),
    }

    return ret;
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Result<Vec<Passport>, Box<dyn Error>> {
    separate_input(input)?
        .iter()
        .map(|row| parse_passport(row))
        .collect()
}

#[aoc(day4, part1)]
fn part_1(passport_list: &[Passport]) -> usize {
    passport_list.iter().filter(|p| valid_passport(p)).count()
}

#[aoc(day4, part2)]
fn part_2(passport_list: &[Passport]) -> usize {
    passport_list
        .iter()
        .filter(|p| valid_passport_p2(p))
        .count()
}

fn valid_passport(passport: &Passport) -> bool {
    if passport.birth_year.is_none()
        || passport.issue_year.is_none()
        || passport.expiration_year.is_none()
        || passport.height.is_none()
        || passport.hair_color.is_none()
        || passport.eye_color.is_none()
        || passport.passport_id.is_none()
    {
        return false;
    }

    return true;
}

fn valid_passport_p2(passport: &Passport) -> bool {
    lazy_static! {
        static ref HC_MATCHER: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        static ref PID_MATCHER: Regex = Regex::new(r"^\d{9}$").unwrap();
    }

    if !valid_passport(passport) {
        return false;
    }
    if let Some(y) = &passport.birth_year {
        let parsed_year: usize = y.parse().unwrap();
        if y.to_string().chars().count() != 4 || !(parsed_year >= 1920 && parsed_year <= 2002) {
            return false;
        }
    }
    if let Some(y) = &passport.issue_year {
        let parsed_year: usize = y.parse().unwrap();
        if y.to_string().chars().count() != 4 || !(parsed_year >= 2010 && parsed_year <= 2020) {
            return false;
        }
    }
    if let Some(y) = &passport.expiration_year {
        let parsed_year: usize = y.parse().unwrap();
        if y.to_string().chars().count() != 4 || !(parsed_year >= 2020 && parsed_year <= 2030) {
            return false;
        }
    }

    if let Some(height) = &passport.height {
        match parse_height(height) {
            Height::IN(v) => {
                if !(v >= 59 && v <= 76) {
                    return false;
                }
            }
            Height::CM(v) => {
                if !(v >= 150 && v <= 193) {
                    return false;
                }
            }
            Height::NONE => {
                return false;
            }
        }
    }

    if let Some(hair_color) = &passport.hair_color {
        if !HC_MATCHER.is_match(hair_color) {
            return false;
        }
    }

    if let Some(passport_id) = &passport.passport_id {
        if !PID_MATCHER.is_match(passport_id) {
            return false;
        }
    }

    if let Some(eye_color) = &passport.eye_color {
        match eye_color.as_ref() {
            "amb" => (),
            "blu" => (),
            "brn" => (),
            "gry" => (),
            "grn" => (),
            "hzl" => (),
            "oth" => (),
            _ => {
                return false;
            }
        }
    }

    return true;
}

fn parse_passport(input: &str) -> Result<Passport, Box<dyn Error>> {
    lazy_static! {
        static ref MATCHER_BIRTH_YR: Regex = Regex::new(r"byr:(?P<birth_year>\d+)").unwrap();
        static ref MATCHER_ISSUE_YR: Regex = Regex::new(r"iyr:(?P<issue_year>\d+)").unwrap();
        static ref MATCHER_EXP_YR: Regex = Regex::new(r"eyr:(?P<expiration_year>\d+)").unwrap();
        static ref MATCHER_HGHT: Regex = Regex::new(r"hgt:(?P<height>\d+[cm|in]*)").unwrap();
        static ref MATCHER_HCL: Regex = Regex::new(r"hcl:(?P<hair_color>\S+)").unwrap();
        static ref MATCHER_ECL: Regex = Regex::new(r"ecl:(?P<eye_color>\S+)").unwrap();
        static ref MATCHER_PID: Regex = Regex::new(r"pid:(?P<passport_id>\S+)").unwrap();
        static ref MATCHER_CID: Regex = Regex::new(r"cid:(?P<country_id>\S+)").unwrap();
    }

    let birth_year = match MATCHER_BIRTH_YR.captures(input) {
        Some(caps) => Some(caps.name("birth_year").unwrap().to_string()),
        None => None,
    };
    let issue_year = match MATCHER_ISSUE_YR.captures(input) {
        Some(caps) => Some(caps.name("issue_year").unwrap().to_string()),
        None => None,
    };
    let expiration_year = match MATCHER_EXP_YR.captures(input) {
        Some(caps) => Some(caps.name("expiration_year").unwrap().to_string()),
        None => None,
    };
    let height = match MATCHER_HGHT.captures(input) {
        Some(caps) => Some(caps.name("height").unwrap().to_string()),
        None => None,
    };
    let hair_color = match MATCHER_HCL.captures(input) {
        Some(caps) => Some(caps.name("hair_color").unwrap().to_string()),
        None => None,
    };
    let eye_color = match MATCHER_ECL.captures(input) {
        Some(caps) => Some(caps.name("eye_color").unwrap().to_string()),
        None => None,
    };
    let passport_id = match MATCHER_PID.captures(input) {
        Some(caps) => Some(caps.name("passport_id").unwrap().to_string()),
        None => None,
    };
    let country_id = match MATCHER_CID.captures(input) {
        Some(caps) => Some(caps.name("country_id").unwrap().parse()?),
        None => None,
    };

    let passport = Passport {
        birth_year,
        issue_year,
        expiration_year,
        height,
        hair_color,
        eye_color,
        passport_id,
        country_id,
    };

    Ok(passport)
}

fn separate_input(input: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut separated: Vec<String> = Default::default();
    let mut lines = input.lines();
    let mut acc_lines: Vec<&str> = Default::default();
    let mut appended = false;
    loop {
        match lines.next() {
            Some(l) => {
                if l.trim().is_empty() {
                    //Empty line
                    if !appended {
                        separated.push(acc_lines.join(" "));
                        appended = true;
                        acc_lines.clear();
                    }
                } else {
                    acc_lines.push(l);
                    appended = false;
                }
            }
            None => {
                separated.push(acc_lines.join(" "));
                break;
            }
        }
    }
    return Ok(separated);
}

#[cfg(test)]
mod tests {
    use super::*;

    const  EXAMPLE_INPUT : &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in";

    #[test]
    fn test_parse_input() {
        let output = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(output.len(), 4);
        assert_eq!(output[3].birth_year, None);
    }

    #[test]
    fn test_separate() {
        let separated = separate_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(separated.len(), 4);

        let other_input = "AAAAA\nAAAA\nAAAA\nAAA\nAAAA\n\n\nBBBB\nBBB\n\n\nCCC";
        assert_eq!(separate_input(other_input).unwrap().len(), 3);

        let yet_another = "AAAA\nAAAA\n\nBBBBBB\n\nCCCCC\nCCCC\nCCCCCC";
        assert_eq!(separate_input(yet_another).unwrap().len(), 3);
    }
    #[test]
    fn test_lines() {
        let input = "\nA\nB\n\nC\n\n";
        let lines: Vec<&str> = input.lines().collect();
        assert_eq!(lines.len(), 6);
        assert_eq!(lines[0].len(), 0);
    }

    #[test]
    fn test_parse_passport() {
        let inputs = separate_input(EXAMPLE_INPUT).unwrap();
        let passport = parse_passport(&inputs[0]).unwrap();
        let passport2 = parse_passport(&inputs[1]).unwrap();
        let passport3 = parse_passport(&inputs[2]).unwrap();
        let passport4 = parse_passport(&inputs[3]).unwrap();
        assert_eq!(passport.birth_year, Some("1937".to_string()));
        assert_eq!(passport2.birth_year, Some("1929".to_string()));
        assert_eq!(passport3.birth_year, Some("1931".to_string()));
        assert_eq!(passport4.birth_year, None);

        assert_eq!(passport.issue_year, Some("2017".to_string()));
        assert_eq!(passport2.issue_year, Some("2013".to_string()));
        assert_eq!(passport3.issue_year, Some("2013".to_string()));
        assert_eq!(passport4.issue_year, Some("2011".to_string()));

        assert_eq!(passport.expiration_year, Some("2020".to_string()));
        assert_eq!(passport2.expiration_year, Some("2023".to_string()));
        assert_eq!(passport3.expiration_year, Some("2024".to_string()));
        assert_eq!(passport4.expiration_year, Some("2025".to_string()));

        assert_eq!(passport.height, Some("183cm".to_string()));
        assert_eq!(passport2.height, None);
        assert_eq!(passport3.height, Some("179cm".to_string()));
        assert_eq!(passport4.height, Some("59in".to_string()));

        assert_eq!(passport.hair_color, Some("#fffffd".to_string()));
        assert_eq!(passport2.hair_color, Some("#cfa07d".to_string()));
        assert_eq!(passport3.hair_color, Some("#ae17e1".to_string()));
        assert_eq!(passport4.hair_color, Some("#cfa07d".to_string()));

        assert_eq!(passport.eye_color, Some("gry".to_string()));
        assert_eq!(passport2.eye_color, Some("amb".to_string()));
        assert_eq!(passport3.eye_color, Some("brn".to_string()));
        assert_eq!(passport4.eye_color, Some("brn".to_string()));

        assert_eq!(passport.country_id, Some("147".to_string()));
        assert_eq!(passport2.country_id, Some("350".to_string()));
        assert_eq!(passport3.country_id, None);
        assert_eq!(passport4.country_id, None);
    }

    #[test]
    fn test_valid_passport_p1() {
        let passports = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(valid_passport(&passports[0]), true);
        assert_eq!(valid_passport(&passports[1]), false);
        assert_eq!(valid_passport(&passports[2]), true);
        assert_eq!(valid_passport(&passports[3]), false);
        assert_eq!(part_1(&passports), 2);
    }

    #[test]
    fn test_invalid_p2() {
        const INVALID: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        let passports = parse_input(INVALID).unwrap();
        assert_eq!(valid_passport_p2(&passports[0]), false);
        assert_eq!(valid_passport_p2(&passports[1]), false);
        assert_eq!(valid_passport_p2(&passports[2]), false);
        assert_eq!(valid_passport_p2(&passports[3]), false);
    }

    #[test]
    fn test_valid_p2() {
        const VALID: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        let passports = parse_input(VALID).unwrap();
        assert_eq!(valid_passport_p2(&passports[0]), true);
        assert_eq!(valid_passport_p2(&passports[1]), true);
        assert_eq!(valid_passport_p2(&passports[2]), true);
        assert_eq!(valid_passport_p2(&passports[3]), true);
    }

    #[test]
    fn test_parse_height() {
        let h = parse_height("186cm");
        assert_eq!(h, Height::CM(186));
        let h2 = parse_height("74in");
        assert_eq!(h2, Height::IN(74));
        let h3 = parse_height("74");
        assert_eq!(h3, Height::NONE);
    }
}
