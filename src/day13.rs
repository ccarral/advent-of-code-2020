use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day13, part1)]
fn parse_input(input: &str) -> (i128, Vec<i128>) {
    let mut lines = input.lines();
    let current_time = lines.next().unwrap().parse().unwrap();
    let values = lines.next().unwrap().split(",");

    let mut parsed_values = vec![];

    for v in values {
        if v == "x" {
            continue;
        } else {
            parsed_values.push(v.parse().unwrap());
        }
    }

    (current_time, parsed_values)
}

#[aoc(day13, part1)]
fn part_1(input: &(i128, Vec<i128>)) -> i128 {
    let (current_time, mut values) = input.clone();
    let mut dist_vec = vec![];
    let mut i = current_time;
    let dist = |a: i128, b: i128| (a - b).abs();
    while !values.is_empty() {
        let mut rm_idxs = vec![];
        for (idx, id) in values.iter().enumerate() {
            if i % id == 0 {
                let distance = dist(i, current_time);
                dist_vec.push((*id, i, distance));
                rm_idxs.push(idx);
            }
        }
        for idx in rm_idxs {
            values.remove(idx);
        }
        i += 1;
    }

    dist_vec.sort_by(|a, b| a.2.cmp(&b.2));

    let (id, time, dist) = dist_vec[0];

    return id * dist;
}

#[aoc_generator(day13, part2)]
fn parse_input_p2(input: &str) -> Vec<i128> {
    let mut parsed_values = vec![];

    let mut lines = input.lines();
    lines.next();
    let values = lines.next().unwrap().split(",");
    for v in values {
        if v == "x" {
            parsed_values.push(-1);
        } else {
            parsed_values.push(v.parse().unwrap());
        }
    }

    return parsed_values;
}

#[aoc(day13, part2)]
fn part_2(input: &[i128]) -> i128 {
    let mut i = 1;
    let mut found = false;
    let values = input.to_vec();

    loop {
        let mut values_iter = values.iter();
        let mut k = i;
        // println!("{}", i);
        loop {
            if let Some(x) = values_iter.next() {
                // print!("({},{})  ", x, k);
                if *x == -1 {
                    k += 1;
                    continue;
                } else if !(k % x == 0) {
                    // println!("{}", !x);
                    break;
                }
            } else {
                found = true;
                break;
            }

            k += 1;
        }
        if found {
            break;
        }
        i += 1;
        // println!("");
        // println!("");
    }

    return i;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "939\n7,13,x,x,59,x,31,19";

    #[test]
    fn test_parse_input() {
        let (time, values) = parse_input(EXAMPLE_INPUT);
        assert_eq!(time, 939);
        assert_eq!(values, vec![7, 13, 59, 31, 19]);
    }

    #[test]
    fn test_p1() {
        let input = parse_input(EXAMPLE_INPUT);
        let p1 = part_1(&input);
        assert_eq!(295, p1);
    }

    #[test]
    fn test_input_2() {
        let values = parse_input_p2(EXAMPLE_INPUT);
        assert_eq!(vec![7, 13, -1, -1, 59, -1, 31, 19], values);
    }

    #[test]
    fn test_p2() {
        let values = parse_input_p2(EXAMPLE_INPUT);
        assert_eq!(part_2(&values), 1068781);
        let values2 = parse_input_p2("XX\n67,x,7,59,61");
        assert_eq!(part_2(&values2), 779210);
    }
}
