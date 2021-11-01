use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<usize> {
    let mut vec: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();
    vec.sort();
    let highest = vec.last().unwrap();
    vec.push(highest + 3);
    vec.insert(0, 0);
    return vec;
}

#[aoc(day10, part1)]
fn part_1(input: &[usize]) -> usize {
    let mut count_1 = 0;
    let mut count_3 = 0;
    for i in 0..input.len() - 1 {
        match input[i + 1] - input[i] {
            1 => count_1 += 1,
            3 => count_3 += 1,
            _ => unreachable!(),
        }
    }

    return count_3 * count_1;
}

#[aoc(day10, part2)]
fn part_2(input: &[usize]) -> usize {
    let current_vec = input;
    let len = current_vec.len();
    let mut pivots = vec![];

    for i in 0..len - 2 {
        if current_vec[i + 1] - current_vec[i] < 3 && current_vec[i + 2] - current_vec[i] <= 3 {
            pivots.push(i);
        }
    }

    let mut count = 0;
    for idx in pivots {
        count += count_inner(&current_vec, idx);
    }

    return count + 2;
}
fn count_inner(slice: &[usize], pivot_idx: usize) -> usize {
    let mut current_vec = slice.to_vec();
    current_vec.remove(pivot_idx);
    let len = current_vec.len();
    let mut pivots = vec![];
    let mut pivot_found = false;

    for i in 0..len - 2 {
        if current_vec[i + 1] - current_vec[i] < 3 && current_vec[i + 2] - current_vec[i] <= 3 {
            pivot_found = true;
            pivots.push(i);
        }
    }

    if !pivot_found {
        // dbg!(current_vec);
        return 1;
    } else {
        let mut count = 0;
        for idx in pivots {
            count += count_inner(&current_vec, idx);
        }

        return count;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";

    #[test]
    fn test_example() {
        let vals = parse_input(EXAMPLE_INPUT);
        let part_1 = part_1(&vals);
        assert_eq!(part_1, 35);
    }

    #[test]
    fn test_example_p2() {
        let vals = parse_input(EXAMPLE_INPUT);
        let count = part_2(&vals);
        assert_eq!(count, 8);
    }
}
