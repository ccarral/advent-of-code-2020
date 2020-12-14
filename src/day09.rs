use aoc_runner_derive::{aoc, aoc_generator};

#[aoc(day9, part1)]
fn find_outlier(input: &str) -> isize {
    let cypher = Cypher::from_capacity(25, input);
    cypher.find_outlier().unwrap()
}

#[aoc(day9, part2)]
fn find_contiguous(input: &str) -> isize {
    let cypher = Cypher::from_capacity(25, input);
    let slice = cypher.find_contiguous(375054920).unwrap();
    let mut numbers = slice.to_vec();
    numbers.sort();
    return numbers[0] + numbers[numbers.len() - 1];
}

struct Cypher {
    numbers: Vec<isize>,
    capacity: usize,
    len: usize,
}

impl Cypher {
    pub fn from_capacity(capacity: usize, input: &str) -> Self {
        let mut lines: Vec<&str> = input.lines().collect::<Vec<&str>>();
        let len = lines.len();
        let numbers: Vec<isize> = lines.iter().map(|x| x.parse().unwrap()).collect();
        Cypher {
            numbers,
            capacity,
            len,
        }
    }

    pub fn valid_number(&self, num: isize) -> Option<(isize, isize)> {
        let mut found = false;
        let (mut idx_i, mut idx_j) = (0, 0);
        let mut found_number = false;
        let mut idx_found = 0;

        // Find idx in list number
        for (idx, x) in self.numbers.iter().enumerate() {
            if *x == num && idx > self.capacity - 1 {
                found_number = true;
                idx_found = idx;
                break;
            }
        }

        if !found_number {
            return None;
        }
        for i in idx_found - self.capacity..idx_found {
            for j in idx_found - self.capacity..idx_found {
                if num == self.numbers[i] + self.numbers[j] {
                    idx_i = i;
                    idx_j = j;
                    found = true;
                    break;
                }
            }
        }

        if found {
            Some((self.numbers[idx_i], self.numbers[idx_j]))
        } else {
            None
        }
    }

    pub fn find_outlier(&self) -> Option<isize> {
        for n in self.numbers[self.capacity..].iter() {
            if self.valid_number(*n).is_none() {
                return Some(*n);
            }
        }

        None
    }

    pub fn find_contiguous(&self, target: isize) -> Option<&[isize]> {
        let find_contiguous = |slice_size, disp| {
            let mut result = None;
            let mut found = false;
            for start_idx in (0 + disp..self.len).step_by(slice_size) {
                let mut sum = 0;
                for idx in start_idx..slice_size {
                    sum += self.numbers[idx];
                    if sum == target {
                        result = Some(&self.numbers[start_idx..idx + 1]);
                        found = true;
                        break;
                    } else if sum > target {
                        break;
                    }
                }
                if found {
                    break;
                }
            }
            return result;
        };

        let mut result = None;
        let mut found = false;

        for slice_size in 2..self.len {
            for displacement in 0..slice_size {
                result = find_contiguous(slice_size, displacement);
                if result.is_some() {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }

        result
    }
}

#[cfg(test)]
pub mod test {
    const EXAMPLE_INPUT: &str =
        "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576";
    use super::*;
    #[test]
    fn test_cypher() {
        let cypher = Cypher::from_capacity(5, EXAMPLE_INPUT);
        assert_eq!(cypher.valid_number(127), None);
        assert!(cypher.valid_number(40).is_some());
        assert!(cypher.valid_number(62).is_some());
        assert!(cypher.valid_number(102).is_some());
        assert!(cypher.valid_number(576).is_some());
    }

    #[test]
    fn test_find_outlier() {
        let cypher = Cypher::from_capacity(5, EXAMPLE_INPUT);
        assert_eq!(cypher.find_outlier(), Some(127));
    }

    #[test]
    fn test_contiguous() {
        let cypher = Cypher::from_capacity(5, EXAMPLE_INPUT);
        let contiguous = cypher.find_contiguous(127);
        let slice = [15isize, 25, 47, 40];
        assert_eq!(contiguous, Some(&slice[..]));
    }
}
