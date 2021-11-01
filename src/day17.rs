use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
struct Space {
    points: Vec<Vec<Vec<char>>>,
    len: usize,
}

// impl fmt::Debug for Space {
// fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// let mut points: Vec<&(isize, isize, isize)> = self.points.keys().collect();

// points.sort_by(|a, b| a.0.cmp(&b.0));
// Ok(())
// }
// }

impl Space {
    pub fn new(l: usize) -> Self {
        let new_space = vec![vec![vec!['.'; l]; l]; l];
        // for x in 0..l {
        // for y in 0..l {
        // for z in 0..l {
        // new_space[x][y][z] = '.';
        // }
        // }
        // }
        Space {
            points: new_space,
            len: l,
        }
    }

    pub fn set_point(&mut self, coordinates: (usize, usize, usize), state: char) {
        let (x, y, z) = coordinates;
        self.points[x][y][z] = state;
    }

    pub fn get_point(&self, coordinates: (usize, usize, usize)) -> char {
        let (x, y, z) = coordinates;
        self.points[x][y][z]
    }

    pub fn count_active_neighbours(&mut self, point: (usize, usize, usize)) -> usize {
        let (x, y, z) = point;
        let mut neighbours = vec![];
        let diffs = [-1, 0, 1];

        let mut active_count = 0;

        let is_active = |c| c == '#';

        for dx in diffs.iter() {
            for dy in diffs.iter() {
                for dz in diffs.iter() {
                    if (dx, dy, dz) != (&0, &0, &0) {
                        neighbours.push((x as isize + dx, y as isize + dy, z as isize + dz));
                    }
                }
            }
        }

        assert_eq!(neighbours.len(), 26);

        for n in neighbours {
            let x = n.0 as usize;
            let y = n.1 as usize;
            let z = n.2 as usize;
            let state = self.get_point((x, y, z));

            if is_active(state) {
                active_count += 1;
            }
        }

        return active_count;
    }

    pub fn get_point_list(&self) -> Vec<(usize, usize, usize)> {
        let mut points = vec![];

        for z in 0..self.len {
            for y in 0..self.len {
                for x in 0..self.len {
                    points.push((x, y, z));
                }
            }
        }
        return points;
    }

    pub fn is_active(&self, point: (usize, usize, usize)) -> bool {
        self.get_point(point) == '#'
    }

    pub fn count_active(&self) -> usize {
        let mut active_count = 0;
        let points = self.get_point_list();
        for p in points {
            // println!("point: {}", v);
            if self.is_active(p) {
                active_count += 1;
            }
        }

        return active_count;
    }

    pub fn cycle(&mut self) {
        let mut activate_queue = vec![];
        let mut deactivate_queue = vec![];
        let point_list = self.get_point_list();

        for p in point_list {
            let is_active = self.is_active(p);
            let active_neighbours = self.count_active_neighbours(p);

            if is_active {
                // println!("active: {:?}", p);
                if active_neighbours == 2 || active_neighbours == 3 {
                    ()
                } else {
                    deactivate_queue.push(p);
                }
            //
            } else {
                if active_neighbours == 3 {
                    activate_queue.push(p);
                }
            }
        }

        for a in activate_queue {
            // println!("activating: {:?}", a);
            self.set_point(a, '#');
        }
        for d in deactivate_queue {
            self.set_point(d, '.');
        }
    }
}

#[aoc_generator(day17)]
fn parse_input(input: &str) -> Space {
    let mut space = Space::new(100);
    input.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            space.set_point((x, y, 0), c);
        })
    });

    return space;
}

#[aoc(day17, part1)]
fn part_1_17(input: &Space) -> usize {
    let mut space = input.clone();

    for i in 0..6 {
        if i == 0 {
            for y in 0..3 {
                for x in 0..3 {
                    print!("{} ", space.is_active((x, y, 0)));
                }
                println!("");
            }
        }

        space.cycle();
    }

    space.count_active()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_input() {
        let input = ".#.\n..#\n###";
        let mut space = parse_input(input);
        // dbg!(space.points);
        let count = part_1_17(&space);
        assert_eq!(space.count_active_neighbours((1, 1, 0)), 6);
    }

    #[test]
    fn test_cycle() {
        let input = ".#.\n..#\n###";
        let mut space = parse_input(input);
        assert_eq!(space.count_active(), 5);
        space.cycle();
        // assert_eq!(space.count_active_neighbours(()), 11);
    }
}
