use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

#[aoc(day14, part1)]
fn part_1(input: &str) -> u64 {
    let mask_matcher = Regex::new(r"mask\s*=\s*(?P<mask>.+)").unwrap();
    let mem_matcher = Regex::new(r"mem\[(?P<addr>\d+)\] = (?P<value>\d+)").unwrap();

    let mut memory = HashMap::<u64, u64>::new();

    let mut mask = "";
    for line in input.lines() {
        match mask_matcher.captures(line) {
            Some(caps) => {
                mask = caps.name("mask").unwrap();
            }
            None => (),
        }

        match mem_matcher.captures(line) {
            Some(caps) => {
                let address = caps.name("addr").unwrap().parse().unwrap();
                let value = caps.name("value").unwrap().parse().unwrap();
                memory.insert(address, apply_mask(value, mask));
            }
            None => (),
        }
    }

    memory.iter().fold(0u64, |acc, (_k, v)| acc + v)
}

#[aoc(day14, part2)]
fn part_2(input: &str) -> u64 {
    let mask_matcher = Regex::new(r"mask\s*=\s*(?P<mask>.+)").unwrap();
    let mem_matcher = Regex::new(r"mem\[(?P<addr>\d+)\] = (?P<value>\d+)").unwrap();

    let mut memory = HashMap::<u64, u64>::new();

    let mut mask = "";
    for line in input.lines() {
        match mask_matcher.captures(line) {
            Some(caps) => {
                mask = caps.name("mask").unwrap();
            }
            None => (),
        }

        match mem_matcher.captures(line) {
            Some(caps) => {
                let address: u64 = caps.name("addr").unwrap().parse().unwrap();
                let value = caps.name("value").unwrap().parse().unwrap();

                let address_str = format!("{:036b}", address);
                let masked_address = mask_addr(&address_str, mask);
                let expanded_addresses = expand_address(&masked_address);
                for addr in expanded_addresses {
                    memory.insert(addr, value);
                }
            }
            None => (),
        }
    }

    memory.iter().fold(0u64, |acc, (_k, v)| acc + v)
}

fn mask_addr(addr: &str, mask: &str) -> String {
    let mut new_addr: Vec<char> = addr.chars().collect();
    let mask_chars: Vec<char> = mask.chars().collect();

    for i in 0..mask_chars.len() {
        if mask_chars[i] == '1' {
            new_addr[i] = '1';
        } else if mask_chars[i] == 'X' {
            new_addr[i] = 'X';
        }
    }

    new_addr.iter().collect()
}

fn rec_bin_str(len: usize, array: &mut Vec<char>, idx: usize, pool: &mut Vec<Vec<char>>) {
    if idx == len {
        pool.push(array.clone());
        return;
    }

    array[idx] = '0';
    rec_bin_str(len, array, idx + 1, pool);

    array[idx] = '1';
    rec_bin_str(len, array, idx + 1, pool);
}

fn get_binary_strings(n: usize) -> Vec<Vec<char>> {
    let mut pool = Vec::<Vec<char>>::new();
    let mut array = vec!['0'; n];
    rec_bin_str(n, &mut array, 0, &mut pool);

    return pool;
}

fn expand_address(addr_in: &str) -> Vec<u64> {
    let chars = addr_in.chars();
    let mut hole_indexes = vec![];

    let addr_template = || {
        let new: Vec<char> = addr_in.chars().collect();
        new
    };

    let mut new_addresses = vec![];

    chars.enumerate().for_each(|(idx, c)| {
        if c == 'X' {
            hole_indexes.push(idx);
        }
    });

    let k = hole_indexes.len();

    let permuts = get_binary_strings(k);

    for p in permuts {
        let mut new = addr_template();
        for (bit, hole_idx) in p.iter().zip(hole_indexes.iter()) {
            new[*hole_idx] = *bit;
        }

        let addr_str: String = new.iter().collect();
        let new_addr_u64 = u64::from_str_radix(&addr_str, 2).unwrap();

        new_addresses.push(new_addr_u64);
    }

    return new_addresses;
}

fn apply_mask(value: u64, mask: &str) -> u64 {
    let bits: Vec<char> = mask.chars().collect();
    let len = 36;
    let mut modified: u64 = value;

    for i in (0..len).rev() {
        let b = bits[i];
        let shift = len - i - 1;
        if b == '1' {
            modified |= 1u64 << shift;
        } else if b == '0' {
            modified &= !(1u64 << shift);
        }
    }

    return modified;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str =
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";

    #[test]
    fn test_mask() {
        let lit = 11;
        let mask1 = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let mask2 = "0XXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let mut masked = apply_mask(lit, mask1);
        assert_eq!(masked, 73);
        masked = apply_mask(101, mask1);
        assert_eq!(masked, 101);

        masked = apply_mask(34359738370, mask2);
        assert_eq!(masked, 64);
    }

    #[test]
    fn test_p1() {
        let mask_matcher = Regex::new(r"mask\s*=\s*(?P<mask>.+)").unwrap();
        let mem_matcher = Regex::new(r"mem\[(?P<addr>\d+)\] = (?P<value>\d+)").unwrap();
        assert!(mask_matcher.is_match("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"));
        assert!(mem_matcher.is_match("mem[8] = 11"));
        let count = part_1(EXAMPLE_INPUT);
        assert_eq!(count, 165);
    }

    #[test]
    fn test_p2() {
        let input = "mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1";
        let count = part_2(input);
        assert_eq!(count, 208);
    }

    #[test]
    fn test_expand_addr() {
        let addr = "10XX01";
        // Should yield
        // 101101 -> 45
        // 101001 -> 41
        // 100101 -> 37
        // 100001 -> 33

        let expanded = expand_address(addr);
        println!("{:?}", expanded);
        assert!(true);
    }

    #[test]
    fn format_bin() {
        let x = 34359738368u64;
        let s = dbg!(format!("{:036b}", x));
        println!("{}", s);
        assert_eq!(s.len(), 36);
    }
}
