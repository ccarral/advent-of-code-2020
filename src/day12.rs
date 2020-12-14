use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day12)]
fn parse_instructions(input: &str) -> Result<Vec<Instruction>, ParseIntError> {
    let lines = input.lines();
    let mut instructions = vec![];
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let direction = chars[0];
        let val = &line[1..];
        let parsed_val: isize = val.parse()?;

        assert!(parsed_val > 0);

        let instruction = match direction {
            'N' => Instruction::N(parsed_val),
            'S' => Instruction::S(parsed_val),
            'E' => Instruction::E(parsed_val),
            'W' => Instruction::W(parsed_val),
            'L' => Instruction::L(parsed_val as usize),
            'R' => Instruction::R(parsed_val as usize),
            'F' => Instruction::F(parsed_val as usize),
            _ => unreachable!(),
        };

        instructions.push(instruction);
    }

    Ok(instructions)
}

#[aoc(day12, part1)]
fn part_1(instructions: &[Instruction]) -> usize {
    let mut ferry = Ferry::new();
    for inst in instructions {
        ferry.apply_move(inst);
    }

    (ferry.w.abs() + ferry.n.abs()) as usize
}

#[aoc(day12, part2)]
fn part_2(instructions: &[Instruction]) -> usize {
    let mut ferry = Ferry::new();
    let mut waypoint = Waypoint::new();

    for inst in instructions {
        match inst {
            Instruction::E(v) => waypoint.apply_move(&Instruction::E(*v)),
            Instruction::N(v) => waypoint.apply_move(&Instruction::N(*v)),
            Instruction::S(v) => waypoint.apply_move(&Instruction::S(*v)),
            Instruction::W(v) => waypoint.apply_move(&Instruction::W(*v)),
            Instruction::R(v) => waypoint.apply_move(&Instruction::R(*v)),
            Instruction::L(v) => waypoint.apply_move(&Instruction::L(*v)),
            Instruction::F(v) => {
                let (x, y) = waypoint.get_rel_pos();
                let actual_x_move = match x {
                    Instruction::E(w) => Instruction::E(w * (*v as isize)),
                    Instruction::W(w) => Instruction::W(w * (*v as isize)),
                    _ => unreachable!(),
                };

                let actual_y_move = match y {
                    Instruction::N(w) => Instruction::N(w * (*v as isize)),
                    Instruction::S(w) => Instruction::S(w * (*v as isize)),
                    _ => unreachable!(),
                };
                ferry.apply_move(&actual_x_move);
                ferry.apply_move(&actual_y_move);
            }
        }
    }

    (ferry.w.abs() + ferry.n.abs()) as usize
}

struct Waypoint {
    n: isize,
    e: isize,
}

impl Waypoint {
    pub fn new() -> Self {
        Waypoint { n: 1, e: 10 }
    }

    pub fn apply_move(&mut self, instruction: &Instruction) {
        let rotate_theta = |x: isize, y: isize, th: isize| {
            let theta = (th as f64).to_radians();
            let x_f64 = x as f64;
            let y_f64 = y as f64;
            let new_x = x_f64 * theta.cos() - y_f64 * theta.sin();
            let new_y = x_f64 * theta.sin() + y_f64 * theta.cos();
            (new_x.round() as isize, new_y.round() as isize)
        };

        match instruction {
            Instruction::N(v) => self.n += v,
            Instruction::S(v) => self.n -= v,
            Instruction::E(v) => self.e += v,
            Instruction::W(v) => self.e -= v,
            Instruction::R(v) => {
                let (e, n) = rotate_theta(self.e, self.n, -(*v as isize));
                self.e = e;
                self.n = n;
            }
            Instruction::L(v) => {
                let (e, n) = rotate_theta(self.e, self.n, *v as isize);
                self.e = e;
                self.n = n;
            }
            _ => unreachable!(),
        }
    }

    pub fn get_rel_pos(&self) -> (Instruction, Instruction) {
        let x = if self.e > 0 {
            Instruction::E(self.e)
        } else {
            Instruction::W(-self.e)
        };
        let y = if self.n > 0 {
            Instruction::N(self.n)
        } else {
            Instruction::S(-self.n)
        };

        (x, y)
    }
}

struct Ferry {
    n: isize,
    s: isize,
    e: isize,
    w: isize,
    facing: Direction,
}

impl Ferry {
    pub fn new() -> Self {
        Ferry {
            n: 0,
            s: 0,
            e: 0,
            w: 0,
            facing: Direction::E,
        }
    }

    pub fn apply_move(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::N(v) => {
                self.n += v;
                self.s -= v;
            }
            Instruction::S(v) => {
                self.s += v;
                self.n -= v;
            }
            Instruction::E(v) => {
                self.e += v;
                self.w -= v;
            }
            Instruction::W(v) => {
                self.w += v;
                self.e -= v;
            }
            Instruction::L(v) => {
                self.rotate(&Instruction::L(*v));
            }
            Instruction::R(v) => {
                self.rotate(&Instruction::R(*v));
            }
            Instruction::F(v) => {
                self.move_forward(*v);
            }
        }
    }

    fn rotate(&mut self, instruction: &Instruction) {
        let map_r = [Direction::E, Direction::S, Direction::W, Direction::N];
        let map_l = [Direction::E, Direction::N, Direction::W, Direction::S];

        let curr_idx_r = |dir| map_r.iter().position(|&s| s == dir).unwrap();

        let curr_idx_l = |dir| map_l.iter().position(|&s| s == dir).unwrap();

        self.facing = match instruction {
            Instruction::L(v) => {
                let current_idx = curr_idx_l(self.facing);
                let idx = (current_idx + (v / 90)) % 4;
                map_l[idx]
            }
            Instruction::R(v) => {
                let current_idx = curr_idx_r(self.facing);
                let idx = (current_idx + (v / 90)) % 4;
                map_r[idx]
            }
            _ => unreachable!(),
        };
    }

    fn move_forward(&mut self, val: usize) {
        let direction = self.facing;
        match direction {
            Direction::E => {
                self.apply_move(&Instruction::E(val as isize));
            }
            Direction::N => {
                self.apply_move(&Instruction::N(val as isize));
            }
            Direction::S => {
                self.apply_move(&Instruction::S(val as isize));
            }
            Direction::W => {
                self.apply_move(&Instruction::W(val as isize));
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    N,
    S,
    E,
    W,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    N(isize),
    S(isize),
    E(isize),
    W(isize),
    L(usize),
    R(usize),
    F(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "F10\nN3\nF7\nR90\nF11";
    const EXAMPLE_INPUT_2: &str = "R180\nF10\nN3\nF7\nR90\nF11";

    #[test]
    fn test_parse_instructions() {
        let instructions = parse_instructions(EXAMPLE_INPUT_1).unwrap();
        assert_eq!(instructions[0], Instruction::F(10));
        assert_eq!(instructions[1], Instruction::N(3));
    }

    #[test]
    fn test_rotation() {
        let mut ferry = Ferry::new();
        ferry.rotate(&Instruction::R(90));
        assert_eq!(ferry.facing, Direction::S);
        // ferry.rotate(&Instruction::L(90));
        // assert_eq!(ferry.facing, Direction::N);
    }

    #[test]
    fn test_movement() {
        let instructions = parse_instructions(EXAMPLE_INPUT_1).unwrap();
        let mut ferry = Ferry::new();
        for inst in instructions {
            ferry.apply_move(&inst);
        }
        assert_eq!(ferry.e, 17);
        assert_eq!(ferry.s, 8);
    }

    #[test]
    fn test_part_1() {
        let instructions = parse_instructions(EXAMPLE_INPUT_1).unwrap();
        let dist = part_1(&instructions);
        assert_eq!(dist, 25);
    }

    #[test]
    fn test_p2() {
        let instructions = parse_instructions(EXAMPLE_INPUT_1).unwrap();
        let dist = part_2(&instructions);
        assert_eq!(286, dist);
    }

    #[test]
    fn test_move_p2() {
        let instructions = parse_instructions(EXAMPLE_INPUT_2).unwrap();
        let dist = part_2(&instructions);
        let mut waypoint = Waypoint::new();
        waypoint.apply_move(&Instruction::R(90));
        assert_eq!(waypoint.e, 1);
        assert_eq!(waypoint.n, -10);
        waypoint.apply_move(&Instruction::R(90));
        assert_eq!(waypoint.e, -10);
        assert_eq!(waypoint.n, -1);
        // assert!(false);
    }
}
