use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse_input_day_1(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|n| n.parse()).collect()
}

#[aoc(day1, part1)]
fn part1(_input: &[i32]) -> i32 {
    for x in _input.iter() {
        for y in _input.iter() {
            if x + y == 2020 {
                return x * y;
            }
        }
    }
    unreachable!();
}

#[aoc(day1, part2)]
fn part2(_input: &[i32]) -> i32 {
    for x in _input.iter() {
        for y in _input.iter() {
            for z in _input.iter() {
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "1721\n979\n366\n299\n675\n1456";
        let input_vec = parse_input_day_1(input).unwrap();
        assert_eq!(input_vec[0], 1721);
        assert_eq!(input_vec[1], 979);
    }

    #[test]
    fn test_p1_e1() {
        let input = "1721\n979\n366\n299\n675\n1456";
        let input_vec = parse_input_day_1(input).unwrap();
        assert_eq!(part1(&input_vec), 514579);
    }

    #[test]
    fn test_p2_e1() {
        let input = "1721\n979\n366\n299\n675\n1456";
        let input_vec = parse_input_day_1(input).unwrap();
        assert_eq!(part2(&input_vec), 241861950);
    }
}
