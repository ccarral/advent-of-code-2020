use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use simple_error::bail;
use std::error::Error;

struct PasswordRow {
    lb: usize,
    ub: usize,
    character: String,
    password: String,
}

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Result<Vec<PasswordRow>, Box<dyn Error>> {
    input.lines().map(|l| parse_line(l)).collect()
}

#[allow(dead_code)]
fn parse_line(line: &str) -> Result<PasswordRow, Box<dyn Error>> {
    lazy_static! {
        static ref MATCHER: Regex =
            Regex::new(r"(?P<lb>\d+)-(?P<ub>\d+)\s(?P<character>\w+):\s(?P<password>\w+)").unwrap();
    }
    match MATCHER.captures(line) {
        Some(caps) => {
            let lb = caps.name("lb").unwrap();
            let ub = caps.name("ub").unwrap();
            let character = caps.name("character").unwrap();
            let password = caps.name("password").unwrap();
            let row = PasswordRow {
                lb: lb.parse()?,
                ub: ub.parse()?,
                character: character.to_string(),
                password: password.to_string(),
            };

            Ok(row)
        }
        None => bail!("No matches!"),
    }
}

#[aoc(day2, part1)]
fn part1(input: &[PasswordRow]) -> usize {
    input.iter().filter(|x| password_valid(x)).count()
}

#[aoc(day2, part2)]
fn part2(input: &[PasswordRow]) -> usize {
    input.iter().filter(|x| password_valid2(x)).count()
}

fn password_valid(psswd: &PasswordRow) -> bool {
    let char_count = psswd.password.matches(&psswd.character).count();
    char_count >= psswd.lb && char_count <= psswd.ub
}

fn password_valid2(psswd: &PasswordRow) -> bool {
    let psswd_chars: Vec<String> = psswd.password.chars().map(|c| c.to_string()).collect();
    xor(
        psswd_chars[psswd.lb - 1] == psswd.character,
        psswd_chars[psswd.ub - 1] == psswd.character,
    )
}

fn xor(a: bool, b: bool) -> bool {
    (a && !b) | (!a & b)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &str = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
    #[test]
    fn test_parse_input() {
        let output: Vec<PasswordRow> = parse_input_day2(EXAMPLE_INPUT).unwrap();
        assert_eq!(output.len(), 3);

        assert_eq!(output[0].ub, 3);
        assert_eq!(output[2].ub, 9);

        assert_eq!(output[0].character, "a");
        assert_eq!(output[2].character, "c");
    }

    #[test]
    fn test_parse_line() {
        let line = "1-3 a: abcde";
        let parsed_result = parse_line(line);
        assert!(parsed_result.is_ok());
        let password = parsed_result.unwrap();
        assert_eq!(password.lb, 1);
        assert_eq!(password.ub, 3);
        assert_eq!(password.character, "a");
        assert_eq!(password.password, "abcde");
    }

    #[test]
    fn check_valid() {
        let line = "1-3 a: abcde";
        let line2 = "1-3 b: cdefg";
        let passwd = parse_line(line).unwrap();
        let passwd2 = parse_line(line2).unwrap();
        assert_eq!(password_valid(&passwd), true);
        assert_eq!(password_valid(&passwd2), false);
    }
    #[test]
    fn check_valid2() {
        let valid = parse_line("1-3 a: abcde").unwrap();
        let invalid = parse_line("1-3 b: cdefg").unwrap();
        let invalid2 = parse_line("2-9 c: ccccccccc").unwrap();
        assert_eq!(password_valid2(&valid), true);
        assert_eq!(password_valid2(&invalid), false);
        assert_eq!(password_valid2(&invalid2), false);
    }
}
