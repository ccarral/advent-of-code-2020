use aoc_runner_derive::aoc;
use simple_error::{bail, SimpleError};
use std::collections::HashSet;
use std::error::Error;
use std::num::ParseIntError;

fn load_assembler(input: &str) -> Result<Assembler, Box<dyn Error>> {
    let mut assembler = Assembler::new();
    assembler.load_program(input)?;
    return Ok(assembler);
}

#[derive(Debug, PartialEq)]
enum AssemblerResult {
    Acc(i64),
    InfiniteLoopErr,
}

#[aoc(day8, part1)]
fn exec_until_inf_loop(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut assembler = Assembler::new();
    assembler.load_program(input)?;
    loop {
        assembler.execute_next_instruction()?;
        if assembler.check_executed(assembler.instruction_pointer)? {
            return Ok(assembler.global_counter);
        } else {
            ()
        }
    }
    bail!("Logical error");
}

#[aoc(day8, part2)]
fn part_2(input: &str) -> i64 {
    let assembler = load_assembler(input).unwrap();
    let first_clone = assembler.clone();
    let mut count_jmp_nop_occ: HashSet<usize> = HashSet::new();

    let instructions = &first_clone.loaded_program.unwrap();
    for idx in 0..instructions.len() {
        match instructions[idx] {
            Instruction::NOP(_v) => {
                count_jmp_nop_occ.insert(idx);
            }
            Instruction::JMP(_v) => {
                count_jmp_nop_occ.insert(idx);
            }
            _ => (),
        }
    }

    let switch_op = |op| match op {
        Instruction::JMP(v) => Instruction::NOP(v),
        Instruction::NOP(v) => Instruction::JMP(v),
        Instruction::ACC(v) => Instruction::ACC(v),
    };

    let mut ret = 0;

    for idx in count_jmp_nop_occ.iter() {
        let mut cloned = assembler.clone();
        assert!(idx < &cloned.program_len);
        let inst = assembler.get_instruction_at_idx(*idx).unwrap();
        cloned
            .set_instruction_at_idx(*idx, switch_op(*inst))
            .unwrap();
        let result = cloned.execute_program().unwrap();
        match result {
            AssemblerResult::Acc(v) => {
                ret = v;
                break;
            }
            AssemblerResult::InfiniteLoopErr => (),
        }
    }

    return ret;
}

#[derive(Clone)]
struct Assembler {
    loaded_program: Option<Vec<Instruction>>,
    executed_flag: Option<Vec<bool>>,
    program_len: usize,
    global_counter: i64,
    instruction_pointer: usize,
}

impl Assembler {
    pub fn new() -> Self {
        Assembler {
            loaded_program: None,
            executed_flag: None,
            program_len: 0,
            instruction_pointer: 0,
            global_counter: 0,
        }
    }

    pub fn execute_program(&mut self) -> Result<AssemblerResult, Box<dyn Error>> {
        if self.loaded_program.is_none() {
            bail!("Program not loaded");
        }

        loop {
            let result = self.execute_next_instruction()?;
            if result.is_some() {
                if self.check_executed(self.instruction_pointer)? {
                    return Ok(AssemblerResult::InfiniteLoopErr);
                } else {
                    ()
                }
            } else {
                break;
            }
        }
        return Ok(AssemblerResult::Acc(self.global_counter));
    }

    pub fn set_instruction_at_idx(
        &mut self,
        idx: usize,
        instruction: Instruction,
    ) -> Result<(), SimpleError> {
        if let Some(instructions) = &mut self.loaded_program {
            instructions[idx] = instruction;
            Ok(())
        } else {
            bail!("No program loaded");
        }
    }
    pub fn get_instruction_at_idx(&self, idx: usize) -> Result<&Instruction, SimpleError> {
        if let Some(instructions) = &self.loaded_program {
            Ok(&instructions[idx])
        } else {
            bail!("No program loaded");
        }
    }

    pub fn parse_instruction(instruction: &str) -> Result<Instruction, Box<dyn Error>> {
        let splitted: Vec<&str> = instruction.split_whitespace().collect();
        let instruction = match splitted[0] {
            "acc" => Instruction::ACC(parse_int(splitted[1])?),
            "jmp" => Instruction::JMP(parse_int(splitted[1])?),
            "nop" => Instruction::NOP(parse_int(splitted[1])?),
            _ => unreachable!(),
        };

        Ok(instruction)
    }

    pub fn load_program(&mut self, input: &str) -> Result<(), Box<dyn Error>> {
        let lines = input.lines();
        let mut program = vec![];
        let mut program_len = 0;

        for line in lines {
            let instruction = Assembler::parse_instruction(line)?;
            program.push(instruction);
            program_len += 1;
        }

        self.program_len = program_len;
        self.loaded_program = Some(program);
        let executed_flag = vec![false; program_len];
        self.executed_flag = Some(executed_flag);

        Ok(())
    }

    pub fn set_executed(&mut self, idx: usize) -> Result<(), SimpleError> {
        if idx >= self.program_len {
            bail!("Out of bounds  se instruction")
        }

        match &mut self.executed_flag {
            Some(ef) => Ok(ef[idx] = true),
            None => bail!("No program loaded!"),
        }
    }

    pub fn check_executed(&self, idx: usize) -> Result<bool, SimpleError> {
        if idx >= self.program_len {
            bail!("Out of bounds  ce instruction")
        }

        match &self.executed_flag {
            Some(v) => Ok(v[idx]),
            None => bail!("No program loaded!"),
        }
    }

    pub fn reset_program(&mut self) {
        let reseted_flags = vec![false; self.program_len];
        self.executed_flag = std::mem::take(&mut Some(reseted_flags));
        self.instruction_pointer = 0;
    }

    fn execute_next_instruction(&mut self) -> Result<Option<Instruction>, SimpleError> {
        if self.instruction_pointer >= self.program_len {
            bail!("Unexpected EOF");
        }

        let mut result: Option<(usize, i64)> = None;

        if let Some(instructions) = &self.loaded_program {
            result = match instructions[self.instruction_pointer] {
                Instruction::JMP(v) => {
                    let new_ins_ptr: i64 = (self.instruction_pointer as i64) + v;
                    Some((new_ins_ptr as usize, self.global_counter))
                }
                Instruction::NOP(_v) => Some((self.instruction_pointer + 1, self.global_counter)),
                Instruction::ACC(v) => {
                    Some((self.instruction_pointer + 1, self.global_counter + v))
                }
            };
        } else {
            bail!("Program not loaded yet");
        }

        if let Some((new_ins_ptr, new_global_ctr)) = result {
            if new_ins_ptr == self.program_len {
                self.set_executed(self.instruction_pointer)?;
                self.global_counter = new_global_ctr;
                Ok(None)
            } else if new_ins_ptr > self.program_len {
                bail!("Invalid new idx");
            } else {
                self.set_executed(self.instruction_pointer)?;
                self.instruction_pointer = new_ins_ptr;
                self.global_counter = new_global_ctr;
                let instructions = self.loaded_program.as_ref().unwrap();
                Ok(Some(instructions[new_ins_ptr].clone()))
            }
        } else {
            bail!("Unexpected error occurred");
        }
    }
}

pub fn parse_int(input: &str) -> Result<i64, ParseIntError> {
    i64::from_str_radix(input, 10)
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Instruction {
    JMP(i64),
    NOP(i64),
    ACC(i64),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PROGRAM: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_parse_int() {
        assert_eq!(parse_int("-1"), Ok(-1));
        assert_eq!(parse_int("+1"), Ok(1));
    }

    #[test]
    fn test_part_2() {
        part_2(EXAMPLE_PROGRAM);
    }

    #[test]
    fn test_instructions() {
        let mut assembler = load_assembler(EXAMPLE_PROGRAM).unwrap();
        assembler.execute_next_instruction();
        assembler.execute_next_instruction();
        assembler.execute_next_instruction();
        assert_eq!(assembler.instruction_pointer, 6);
        assert_eq!(assembler.global_counter, 1);
    }

    #[test]
    fn test_usize_cast() {
        let x = 5usize;
        let y = -6isize;
        assert_eq!(x as isize + y, -1);
    }

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            Assembler::parse_instruction("acc +1").unwrap(),
            Instruction::ACC(1)
        );
    }
}
