use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse_input_day_1(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(|n| n.parse()).collect()
}

#[aoc(day1, part1)]
fn part_sol1(_input: &[u32]) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert!(1 == 1)
    }
}
