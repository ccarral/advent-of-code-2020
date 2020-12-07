use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use std::mem;

type AnswerGroup = Vec<String>;

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<AnswerGroup> {
    let mut answer_groups: Vec<AnswerGroup> = Default::default();
    let mut lines = input.lines();
    let mut acc_lines: Vec<String> = Default::default();
    let mut appended = false;
    loop {
        match lines.next() {
            Some(l) => {
                if l.trim().is_empty() {
                    //Empty line
                    if !appended {
                        let taken_vec = mem::take(&mut acc_lines);
                        answer_groups.push(taken_vec);
                        appended = true;
                    }
                } else {
                    acc_lines.push(l.to_string());
                    appended = false;
                }
            }
            None => {
                answer_groups.push(acc_lines);
                break;
            }
        }
    }
    return answer_groups;
}

#[aoc(day6, part1)]
fn part_1(answer_group_list: &[AnswerGroup]) -> usize {
    answer_group_list
        .iter()
        .map(|ag| count_answers(ag))
        .fold(0, |acc, ct| acc + ct)
}

#[aoc(day6, part2)]
fn part_2(answer_group_list: &[AnswerGroup]) -> usize {
    answer_group_list
        .iter()
        .map(|ag| count_answers2(ag))
        .fold(0, |acc, ct| acc + ct)
}

fn count_answers(answer_group: &AnswerGroup) -> usize {
    let mut answer_hash = HashSet::new();

    for answers in answer_group {
        let chars = answers.chars();
        for c in chars {
            answer_hash.insert(c);
        }
    }

    answer_hash.len()
}

fn count_answers2(answer_group: &AnswerGroup) -> usize {
    let mut answer_hash: HashMap<char, usize> = HashMap::new();
    let person_count = answer_group.len();
    let mut count = 0;
    for answer in answer_group {
        let chars = answer.chars();
        for c in chars {
            *answer_hash.entry(c).or_insert(0) += 1;
        }
    }

    for (_k, v) in answer_hash.iter() {
        if *v == person_count {
            count += 1;
        }
    }
    return count;
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";

    #[test]
    fn test_separate_input() {
        let answer_groups = parse_input(EXAMPLE_INPUT);
        println!("{:?}", answer_groups);
        assert_eq!(answer_groups.len(), 5);
    }

    #[test]
    fn test_count_answers() {
        let answer_groups = parse_input(EXAMPLE_INPUT);
        assert_eq!(count_answers(&answer_groups[0]), 3);
        assert_eq!(count_answers(&answer_groups[1]), 3);
        assert_eq!(count_answers(&answer_groups[2]), 3);
        assert_eq!(count_answers(&answer_groups[3]), 1);
        assert_eq!(count_answers(&answer_groups[4]), 1);
    }

    #[test]
    fn test_example_p1() {
        let answer_groups = parse_input(EXAMPLE_INPUT);
        assert_eq!(part_1(&answer_groups), 11);
    }

    #[test]
    fn test_count2() {
        let answer_groups = parse_input(EXAMPLE_INPUT);
        assert_eq!(count_answers2(&answer_groups[0]), 3);
    }

    #[test]
    fn test_example_p2() {
        let answer_groups = parse_input(EXAMPLE_INPUT);
        assert_eq!(part_2(&answer_groups), 6);
    }
}
