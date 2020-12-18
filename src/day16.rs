use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::{HashMap, HashSet};

struct Input {
    hash: HashMap<String, [usize; 4]>,
    my_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

#[aoc_generator(day16)]
fn parse_input(input: &str) -> Input {
    const MY_TICKET_IDX: usize = 22;
    const NEARBY_TICKETS_START_IDX: usize = 25;

    let mut values_range = HashMap::<String, [usize; 4]>::new();
    let mut nearby_tickets = vec![];

    let range_matcher =
        Regex::new(r"(?P<key>.+): (?P<v1>\d+)-(?P<v2>\d+) or (?P<v3>\d+)-(?P<v4>\d+)").unwrap();

    let lines: Vec<&str> = input.lines().collect();

    for i in 0..MY_TICKET_IDX {
        let current_line = lines[i];
        match range_matcher.captures(current_line) {
            Some(captures) => {
                let key = captures.name("key").unwrap().to_string();
                let v1 = captures.name("v1").unwrap().parse().unwrap();
                let v2 = captures.name("v2").unwrap().parse().unwrap();
                let v3 = captures.name("v3").unwrap().parse().unwrap();
                let v4 = captures.name("v4").unwrap().parse().unwrap();
                values_range.insert(key, [v1, v2, v3, v4]);
            }
            None => (),
        }
    }

    let my_ticket = lines[MY_TICKET_IDX]
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();

    for i in NEARBY_TICKETS_START_IDX..lines.len() {
        let current_row = lines[i].split(",").map(|v| v.parse().unwrap()).collect();
        nearby_tickets.push(current_row);
    }

    Input {
        hash: values_range,
        my_ticket,
        nearby_tickets,
    }
}

#[aoc(day16, part1)]
fn part_1(in_values: &Input) -> usize {
    let mut err_count = 0;

    let in_range = |x: usize, v: &[usize; 4]| (x >= v[0] && x <= v[1]) || (x >= v[2] && x <= v[3]);

    let nearby_tickets = &in_values.nearby_tickets;
    let valid_ranges = in_values.hash.to_owned();

    for ticket in nearby_tickets {
        // Check that every v is at least valid in one range
        for v in ticket {
            let mut valid_value = false;
            for range in valid_ranges.values() {
                if in_range(*v, range) {
                    valid_value = true;
                }
                if valid_value {
                    break;
                }
            }

            if !valid_value {
                err_count += v;
                break;
            }
        }
    }
    return err_count;
}

#[aoc(day16, part2)]
fn part_2(in_values: &Input) -> usize {
    let in_range = |x: usize, v: &[usize; 4]| (x >= v[0] && x <= v[1]) || (x >= v[2] && x <= v[3]);

    let nearby_tickets = &in_values.nearby_tickets;
    let valid_ranges = in_values.hash.to_owned();
    let my_ticket = &in_values.my_ticket;

    let mut valid_tickets_idx = vec![];

    for (idx, ticket) in nearby_tickets.iter().enumerate() {
        let mut valid_ticket = true;
        // Check that every v is at least valid in one range
        for v in ticket {
            let mut valid_value = false;
            for range in valid_ranges.values() {
                if in_range(*v, range) {
                    valid_value = true;
                }
                if valid_value {
                    break;
                }
            }

            if !valid_value {
                valid_ticket = false;
                break;
            }
        }

        if valid_ticket {
            valid_tickets_idx.push(idx);
        }
    }

    let mut valid_tickets = vec![];

    for idx in valid_tickets_idx {
        valid_tickets.push(nearby_tickets[idx].to_owned());
    }

    let columns = valid_tickets[0].len();
    let rows = valid_tickets.len();

    let column_valid_in_range = |column_idx: usize, range: &[usize; 4]| {
        for i in 0..rows {
            if !in_range(valid_tickets[i][column_idx], range) {
                return false;
            }
        }

        return true;
    };

    let mut column_set = HashSet::<usize>::new();
    let mut field_set = HashSet::<&str>::new();

    // Maps each column to a field (range)
    let mut field_column_hash = HashMap::<&str, usize>::new();

    (0..columns).for_each(|j| {
        column_set.insert(j);
    });

    valid_ranges.keys().for_each(|f| {
        field_set.insert(f);
    });

    while !field_set.is_empty() {
        // Count number of columns that are valid for each range
        let mut field_columns_list = HashMap::<&str, Vec<usize>>::new();
        for f in field_set.iter() {
            let current_range = valid_ranges.get(*f).unwrap();

            let mut valid_in_columns = vec![];
            for j in column_set.iter() {
                if column_valid_in_range(*j, current_range) {
                    valid_in_columns.push(*j);
                }
            }
            field_columns_list.insert(f, valid_in_columns);
        }

        //Find field in field_columns_list for whose range is only valid in one column
        let mut rm_key = "";
        let mut rm_column = 0;
        for (field, columns) in field_columns_list.iter() {
            if columns.len() == 1 {
                field_column_hash.insert(field, columns[0]);
                rm_key = field;
                rm_column = columns[0];
            }
        }

        field_set.remove(rm_key);
        column_set.remove(&rm_column);
    }

    let c1 = field_column_hash.get("departure location").unwrap();
    let c2 = field_column_hash.get("departure station").unwrap();
    let c3 = field_column_hash.get("departure platform").unwrap();
    let c4 = field_column_hash.get("departure track").unwrap();
    let c5 = field_column_hash.get("departure date").unwrap();
    let c6 = field_column_hash.get("departure time").unwrap();

    let columns_departure = [c1, c2, c3, c4, c5, c6];

    columns_departure
        .iter()
        .fold(1, |acc, c| acc * my_ticket[**c])
}
