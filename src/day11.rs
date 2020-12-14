use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut seats: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let chars = line.chars().collect();
        seats.push(chars);
    }

    return seats;
}

#[aoc(day11, part1)]
fn part_1(seats: &Vec<Vec<char>>) -> usize {
    let mut new_seats = seats.clone();

    loop {
        let switch_results = switch_seats(&new_seats);
        let switched = switch_results.1;
        new_seats = switch_results.0;
        if !switched {
            break;
        }
    }

    let mut occuppied_count = 0;
    let (m, n) = (new_seats.len(), new_seats[0].len());
    for i in 0..m {
        for j in 0..n {
            if new_seats[i][j] == '#' {
                occuppied_count += 1;
            }
        }
    }

    return occuppied_count;
}

#[aoc(day11, part2)]
fn part_2(seats: &Vec<Vec<char>>) -> usize {
    let mut new_seats = seats.clone();
    loop {
        let switch_results = switch_seats_p2(&new_seats);
        let switched = switch_results.1;
        new_seats = switch_results.0;
        if !switched {
            break;
        }
    }

    let mut occuppied_count = 0;
    let (m, n) = (new_seats.len(), new_seats[0].len());
    for i in 0..m {
        for j in 0..n {
            if new_seats[i][j] == '#' {
                occuppied_count += 1;
            }
        }
    }

    return occuppied_count;
}

fn switch_seat(seats: &Vec<Vec<char>>, i: usize, j: usize) -> (bool, char) {
    if seats[i][j] == 'L' {
        let mut all_empty = true;
        for x in i - 1..=i + 1 {
            for y in j - 1..=j + 1 {
                all_empty = all_empty && (seats[x][y] == 'L' || seats[x][y] == '.');
            }
        }
        let switch = if all_empty { true } else { false };
        let switch_char = if switch { '#' } else { 'L' };

        (switch, switch_char)
    } else if seats[i][j] == '#' {
        let mut occuppied_count = 0;
        for x in i - 1..=i + 1 {
            for y in j - 1..=j + 1 {
                if x == i && y == j {
                    continue;
                }
                occuppied_count += if seats[x][y] == '#' { 1 } else { 0 };
            }
        }

        let switch = occuppied_count >= 4;
        let switch_char = if switch { 'L' } else { '#' };
        (switch, switch_char)
    } else {
        (false, '.')
    }
}

fn switch_seats(seats: &Vec<Vec<char>>) -> (Vec<Vec<char>>, bool) {
    let m = seats.len();
    let n = seats[0].len();
    let mut new_seating = seats.clone();
    let mut switched = false;
    for i in 0..m {
        for j in 0..n {
            if i > 0 && i < m - 1 && j > 0 && j < n - 1 {
                // Normal position
                let (_switched, _new_char) = switch_seat(seats, i, j);
                new_seating[i][j] = _new_char;
                switched = switched || _switched;
            } else {
                // Edge cases

                //|1|2|3|
                //|4|5|6|
                //|7|8|9|

                // For each seat[i][j], it will be treated as pos 5

                if seats[i][j] == '.' {
                    continue;
                }

                let mut free_seats = vec![];

                if i == 0 {
                    // Top row
                    let pos_8_free = seats[i + 1][j] == 'L' || seats[i + 1][j] == '.';
                    free_seats.push(pos_8_free);

                    if j == 0 {
                        // Top left
                        let pos_9_free = seats[i + 1][j + 1] == 'L' || seats[i + 1][j + 1] == '.';
                        let pos_6_free = seats[i][j + 1] == 'L' || seats[i][j + 1] == '.';
                        free_seats.push(pos_6_free);
                        free_seats.push(pos_9_free);
                    } else if j == n - 1 {
                        // Top right
                        let pos_4_free = seats[i][j - 1] == 'L' || seats[i][j - 1] == '.';
                        let pos_7_free = seats[i + 1][j - 1] == 'L' || seats[i + 1][j - 1] == '.';
                        free_seats.push(pos_4_free);
                        free_seats.push(pos_7_free);
                    } else {
                        // Rest of top row
                        let pos_4_free = seats[i][j - 1] == 'L' || seats[i][j - 1] == '.';
                        let pos_6_free = seats[i][j + 1] == 'L' || seats[i][j + 1] == '.';
                        let pos_7_free = seats[i + 1][j - 1] == 'L' || seats[i + 1][j - 1] == '.';
                        let pos_9_free = seats[i + 1][j + 1] == 'L' || seats[i + 1][j + 1] == '.';
                        free_seats.push(pos_4_free);
                        free_seats.push(pos_6_free);
                        free_seats.push(pos_7_free);
                        free_seats.push(pos_9_free);
                    }
                } else if i == m - 1 {
                    // Bottom row
                    let pos_2_free = seats[i - 1][j] == 'L' || seats[i - 1][j] == '.';
                    free_seats.push(pos_2_free);
                    if j == 0 {
                        // Bottom left
                        let pos_3_free = seats[i - 1][j + 1] == 'L' || seats[i - 1][j + 1] == '.';
                        let pos_6_free = seats[i][j + 1] == 'L' || seats[i][j + 1] == '.';
                        free_seats.push(pos_3_free);
                        free_seats.push(pos_6_free);
                    } else if j == n - 1 {
                        //Bottom right
                        let pos_1_free = seats[i - 1][j - 1] == 'L' || seats[i - 1][j - 1] == '.';
                        let pos_4_free = seats[i][j - 1] == 'L' || seats[i][j - 1] == '.';
                        free_seats.push(pos_1_free);
                        free_seats.push(pos_4_free);
                    } else {
                        // Rest of bottom row
                        let pos_1_free = seats[i - 1][j - 1] == 'L' || seats[i - 1][j - 1] == '.';
                        let pos_3_free = seats[i - 1][j + 1] == 'L' || seats[i - 1][j + 1] == '.';
                        let pos_4_free = seats[i][j - 1] == 'L' || seats[i][j - 1] == '.';
                        let pos_6_free = seats[i][j + 1] == 'L' || seats[i][j + 1] == '.';
                        free_seats.push(pos_1_free);
                        free_seats.push(pos_3_free);
                        free_seats.push(pos_4_free);
                        free_seats.push(pos_6_free);
                    }
                }

                if j == 0 && (i != 0 && i != m - 1) {
                    // Leftmost column (not corners)
                    let pos_2_free = seats[i - 1][j] == 'L' || seats[i - 1][j] == '.';
                    let pos_3_free = seats[i - 1][j + 1] == 'L' || seats[i - 1][j + 1] == '.';
                    let pos_6_free = seats[i][j + 1] == 'L' || seats[i][j + 1] == '.';
                    let pos_8_free = seats[i + 1][j] == 'L' || seats[i + 1][j] == '.';
                    let pos_9_free = seats[i + 1][j + 1] == 'L' || seats[i + 1][j + 1] == '.';
                    free_seats.push(pos_2_free);
                    free_seats.push(pos_3_free);
                    free_seats.push(pos_6_free);
                    free_seats.push(pos_8_free);
                    free_seats.push(pos_9_free);
                } else if j == n - 1 && (i != 0 && i != m - 1) {
                    // Rightmost column (not corners)
                    let pos_1_free = seats[i - 1][j - 1] == 'L' || seats[i - 1][j - 1] == '.';
                    let pos_2_free = seats[i - 1][j] == 'L' || seats[i - 1][j] == '.';
                    let pos_4_free = seats[i][j - 1] == 'L' || seats[i][j - 1] == '.';
                    let pos_7_free = seats[i + 1][j - 1] == 'L' || seats[i + 1][j - 1] == '.';
                    let pos_8_free = seats[i + 1][j] == 'L' || seats[i + 1][j] == '.';
                    free_seats.push(pos_1_free);
                    free_seats.push(pos_2_free);
                    free_seats.push(pos_4_free);
                    free_seats.push(pos_7_free);
                    free_seats.push(pos_8_free);
                }

                if seats[i][j] == 'L' {
                    // Check all seats that surround it are empty (all values in free_seats are
                    // true)
                    let mut all_empty = true;
                    for val in free_seats {
                        all_empty = all_empty && val;
                    }
                    let switch = all_empty;
                    let switch_char = if switch { '#' } else { 'L' };
                    switched = switched || switch;
                    new_seating[i][j] = switch_char;
                } else if seats[i][j] == '#' {
                    // Check if there are 4 or more seats occuppied (values in free_seats that are
                    // false)
                    let mut occuppied_count = 0;
                    for free in free_seats {
                        if !free {
                            occuppied_count += 1;
                        }
                    }
                    let switch = occuppied_count >= 4;
                    switched = switched || switch;
                    let switch_char = if switch { 'L' } else { '#' };
                    new_seating[i][j] = switch_char;
                }
            }
        }
    }

    return (new_seating, switched);
}

pub fn switch_seats_p2(seats: &Vec<Vec<char>>) -> (Vec<Vec<char>>, bool) {
    let mut new_seats = seats.clone();
    let m = seats.len();
    let n = seats[0].len();
    let mut switched = false;

    for i in 0..m {
        for j in 0..n {
            let occuppied = count_occuppied_p2(seats, (i, j), (m, n));
            if seats[i][j] == 'L' {
                // Check that there are no occuppied seats
                if occuppied == 0 {
                    switched = true;
                    new_seats[i][j] = '#';
                }
            } else if seats[i][j] == '#' {
                // Check if count >= 5

                if occuppied >= 5 {
                    switched = true;
                    new_seats[i][j] = 'L';
                }
            }
        }
    }

    (new_seats, switched)
}

pub fn count_occuppied_p2(
    seats: &Vec<Vec<char>>,
    pos: (usize, usize),
    dims: (usize, usize),
) -> usize {
    // _______
    //|UL|U|UR|
    //|L |x| R|
    //|DL|D|DR|

    let mut occuppied = vec![];
    let (i, j) = pos;
    let (m, n) = dims;

    let mut found_u = false;
    let mut found_ul = false;
    let mut found_ur = false;
    let mut found_d = false;
    let mut found_dl = false;
    let mut found_dr = false;
    let mut found_l = false;
    let mut found_r = false;

    // Check up
    if i != 0 {
        for x in (0..=i - 1).rev() {
            if seats[x][j] == '.' {
                continue;
            }
            found_u = seats[x][j] == '#';
            break;
        }
    }

    // Check down
    for x in i + 1..m {
        if seats[x][j] == '.' {
            continue;
        }
        found_d = seats[x][j] == '#';
        break;
    }

    // Check left

    if j != 0 {
        for y in (0..=j - 1).rev() {
            if seats[i][y] == '.' {
                continue;
            }
            found_l = seats[i][y] == '#';
            break;
        }
    }

    // Check right
    for y in j + 1..n {
        if seats[i][y] == '.' {
            continue;
        }
        found_r = seats[i][y] == '#';
        break;
    }

    // Check UL
    let mut x = i;
    let mut y = j;
    loop {
        if x == 0 || y == 0 {
            break;
        }
        x = x - 1;
        y = y - 1;
        if seats[x][y] == '.' {
            continue;
        }
        found_ul = seats[x][y] == '#';
        break;
    }

    // Check UR
    let mut x = i;
    let mut y = j;
    loop {
        if x == 0 || y == n - 1 {
            break;
        }
        x = x - 1;
        y = y + 1;
        if seats[x][y] == '.' {
            continue;
        }
        found_ur = seats[x][y] == '#';
        break;
    }

    // Check DL
    let mut x = i;
    let mut y = j;
    loop {
        if x == m - 1 || y == 0 {
            break;
        }
        x = x + 1;
        y = y - 1;
        if seats[x][y] == '.' {
            continue;
        }
        found_dl = seats[x][y] == '#';
        break;
    }

    // Check DR
    let mut x = i;
    let mut y = j;
    loop {
        x = x + 1;
        y = y + 1;
        if x == m || y == n {
            break;
        }
        if seats[x][y] == '.' {
            continue;
        }
        found_dr = seats[x][y] == '#';
        break;
    }

    occuppied.push(found_ul);
    occuppied.push(found_u);
    occuppied.push(found_ur);
    occuppied.push(found_l);
    occuppied.push(found_r);
    occuppied.push(found_dl);
    occuppied.push(found_d);
    occuppied.push(found_dr);

    let mut occuppied_count = 0;

    for occ in occuppied {
        if occ {
            occuppied_count += 1;
        }
    }

    occuppied_count
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    const EXAMPLE_OUTPUT_1: &str = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";

    const EXAMPLE_OUTPUT_2: &str = "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##";

    const EXAMPLE_OUTPUT_2_P2: &str = "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#";

    const EXAMPLE_OUTPUT_3_P2: &str = "#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#";
    #[test]
    fn test_switch_seats() {
        let seats = parse_input(EXAMPLE_INPUT);
        let expected_output = parse_input(EXAMPLE_OUTPUT_1);
        let expected_output2 = parse_input(EXAMPLE_OUTPUT_2);
        let (new_seats, switched) = switch_seats(&seats);
        let (new_seats2, switched2) = switch_seats(&new_seats);
        assert!(switched);
        assert_eq!(expected_output, new_seats);
        // assert_eq!(expected_output2, new_seats2);
        let m = expected_output2.len();
        let n = expected_output2[0].len();
        for i in 0..m {
            for j in 0..n {
                assert_eq!(expected_output2[i][j], new_seats2[i][j]);
            }
        }
    }

    #[test]
    fn test_p2() {
        let seats = parse_input(EXAMPLE_INPUT);
        let expected_output1 = parse_input(EXAMPLE_OUTPUT_1);
        let expected_output2 = parse_input(EXAMPLE_OUTPUT_2_P2);
        let expected_output3 = parse_input(EXAMPLE_OUTPUT_3_P2);
        let (output1, switched1) = switch_seats_p2(&seats);
        let (output2, switched2) = switch_seats_p2(&output1);
        let (output3, switched3) = switch_seats_p2(&output2);
        assert_eq!(output1, expected_output1);
        assert_eq!(output2, expected_output2);
        assert_eq!(output3, expected_output3);
        let m = expected_output3.len();
        let n = expected_output3[0].len();
    }

    #[test]
    fn test_count_occuppied() {
        let seats = parse_input(EXAMPLE_INPUT);
        let expected_output1 = parse_input(EXAMPLE_OUTPUT_1);
        let expected_output2 = parse_input(EXAMPLE_OUTPUT_2_P2);
        let expected_output3 = parse_input(EXAMPLE_OUTPUT_3_P2);
        let m = seats.len();
        let n = seats[0].len();
        assert_eq!(count_occuppied_p2(&expected_output2, (1, 3), (m, n)), 0);
    }
}
