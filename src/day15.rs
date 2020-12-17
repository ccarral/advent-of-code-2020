use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[aoc(day15, part1)]
fn part_1(input: &str) -> usize {
    return get_nth(input, 2020);
}

#[aoc(day15, part2)]
fn part_2(input: &str) -> usize {
    return get_nth(input, 30000000);
}

fn get_nth(input: &str, n: usize) -> usize {
    let mut seeds: Vec<usize> = input.split(",").map(|x| x.parse().unwrap()).collect();

    let mut position_hash = HashMap::<usize, usize>::new();

    for (idx, s) in seeds.iter().enumerate() {
        position_hash.insert(*s, idx);
    }

    let mut get_nth_inner = |n: usize, seeds: &Vec<usize>| {
        let last = position_hash.insert(seeds[n - 1], n - 1);

        if last.is_none() {
            return 0;
        } else {
            // Not first occurrence of value
            let last_pos = last.unwrap();
            return n - 1 - last_pos;
        }
    };
    // Starts right at the end of the seeds vector and appends to it
    for i in seeds.len()..=n {
        let new = get_nth_inner(i, &seeds);
        seeds.push(new);
    }

    seeds[n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1() {
        let val = part_1("0,3,6");
        assert_eq!(val, 436);
    }
}
