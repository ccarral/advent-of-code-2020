use aoc_runner_derive::{aoc, aoc_generator};

type Grid = Vec<Vec<char>>;

struct Map {
    grid: Grid,
    n: usize,
    m: usize,
}

impl Map {
    pub fn get_char_at(&self, i: usize, j: usize) -> char {
        return self.grid[i % self.m][j % self.n];
    }
}

#[aoc_generator(day3)]
fn parse_map(input: &str) -> Map {
    let mut grid: Grid = Default::default();
    input.lines().for_each(|line| {
        let values = line.chars().collect();
        grid.push(values);
    });

    let m = grid.len();
    let n = grid[0].len();

    Map { grid, n, m }
}

fn count_trees_slope(map: &Map, slope: (usize, usize)) -> usize {
    let mut count = 0;
    let mut i = 0;
    let mut j = 0;
    for _x in (0..map.m).step_by(slope.1) {
        match map.get_char_at(i, j) {
            '.' => (),
            '#' => count += 1,
            _ => panic!("This is not tupposed to happen!"),
        }
        i = i + slope.1;
        j = j + slope.0;
    }
    return count;
}

#[aoc(day3, part1)]
fn count_trees(map: &Map) -> usize {
    return count_trees_slope(map, (3, 1));
}

#[aoc(day3, part2)]
fn part_2(map: &Map) -> usize {
    let slopes = [(1usize, 1usize), (3, 1), (5, 1), (7, 1), (1, 2)];

    slopes
        .iter()
        .map(|s| count_trees_slope(map, *s))
        .fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";
    #[test]
    fn test_parse_map() {
        let map = parse_map(TEST_INPUT);
        assert_eq!(map.n, 11);
        assert_eq!(map.m, 11);
    }
    #[test]
    fn test_example_p1() {
        let map = parse_map(TEST_INPUT);
        assert_eq!(count_trees(&map), 7);
    }

    #[test]
    fn test_get_char() {
        let map = parse_map(TEST_INPUT);
        assert_eq!(map.get_char_at(0, 13), '#');
        assert_eq!(map.get_char_at(0, 15), '.');
    }

    #[test]
    fn test_count_slope() {
        let map = parse_map(TEST_INPUT);
        assert_eq!(count_trees_slope(&map, (3, 1)), 7);
        assert_eq!(count_trees_slope(&map, (1, 2)), 2);
    }
    #[test]
    fn test_example_p2() {
        let map = parse_map(TEST_INPUT);
        assert_eq!(part_2(&map), 336);
    }
}
