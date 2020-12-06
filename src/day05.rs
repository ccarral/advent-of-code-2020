use aoc_runner_derive::{aoc, aoc_generator};

type Seat = (u64, u64, u64);

#[aoc_generator(day5)]
fn parse_input_day5(input: &str) -> Vec<Seat> {
    input.lines().map(|l| parse_seat(l)).collect()
}

#[aoc(day5, part1)]
fn day5_part1(input: &[Seat]) -> u64 {
    input.iter().max_by(|x, y| x.2.cmp(&y.2)).unwrap().2
}

#[aoc(day5, part2)]
fn day5_part2(input: &[Seat]) -> u64 {
    let mut sorted = input.to_vec();
    sorted.sort_by(|x, y| x.cmp(y));
    let len = sorted.len();

    for i in 0..(len - 1) {
        let current_seat = sorted[i];
        let next_seat = sorted[i + 1];

        if next_seat.2 > current_seat.2 + 1 {
            return current_seat.2 + 1;
        }
    }
    unreachable!();
}

fn parse_seat(seat: &str) -> Seat {
    let row = &seat[..7];
    let row_transformed: String = row
        .chars()
        .map(|c| match c {
            'F' => '0',
            'B' => '1',
            _ => unreachable!(),
        })
        .collect();
    let column = &seat[7..];
    let column_transformed: String = column
        .chars()
        .map(|c| match c {
            'L' => '0',
            'R' => '1',
            _ => unreachable!(),
        })
        .collect();

    let row_u64 = parse_bin(&row_transformed);
    let col_u64 = parse_bin(&column_transformed);

    (row_u64, col_u64, row_u64 * 8 + col_u64)
}

fn parse_bin(b: &str) -> u64 {
    u64::from_str_radix(b, 2).unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE_INPUT: &str = "BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";
    #[test]
    fn test_parse_input() {
        let seat_input: Vec<&str> = EXAMPLE_INPUT.lines().collect();
        assert_eq!(parse_seat(seat_input[0]), (70, 7, 567));
        assert_eq!(parse_seat(seat_input[1]), (14, 7, 119));
        assert_eq!(parse_seat(seat_input[2]), (102, 4, 820));
    }

    #[test]
    fn test_p1() {
        let seats = parse_input_day5(EXAMPLE_INPUT);
        let max_id = day5_part1(&seats);
        assert_eq!(max_id, 820);
    }

    #[test]
    fn test_p2() {
        let seats = parse_input_day5(EXAMPLE_INPUT);
        let id = day5_part2(&seats);
    }
}
