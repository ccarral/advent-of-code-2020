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

fn has_hole(n1: usize, n3: usize) -> bool {
    return n3 - n1 == 2 || n3 - n1 == 3;
}

#[aoc(day10, part2)]
fn part_2(input: &[usize]) -> usize {
    let mut count = 0;
    for i in 1..input.len() - 1 {
        if has_hole(input[i - 1], input[i + 1]) {
            count += count_inner(input, i);
        }
    }
    return count;
}

fn count_inner(slice: &[usize], pivot_idx: usize) -> usize {
    let mut new_vec = slice.to_vec();
    let input = std::io::stdin();
    // println!("pivot: {}", new_vec[pivot_idx]);
    new_vec.remove(pivot_idx);
    // println!("{:?}", new_vec);
    // input.read_line(&mut String::new());
    let mut found_hole = false;
    let mut count = 0;
    for i in 1..new_vec.len() - 1 {
        // println!("({})", i);
        input.read_line(&mut String::new());
        if has_hole(new_vec[i - 1], new_vec[i + 1]) {
            count += 1;
            // println!("Descending");
            count += count_inner(&new_vec, i);
            found_hole = true;
        }
    }
    if !found_hole {
        return 0;
    } else {
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

    // #[test]
    fn test_example_p2() {
        let vals = parse_input(EXAMPLE_INPUT);
        let count = part_2(&vals);
        assert_eq!(count, 8);
    }

    #[test]
    fn test_hole() {
        assert!(has_hole(4, 6));
        assert!(has_hole(4, 7));
        assert!(!has_hole(4, 10));
    }
}
