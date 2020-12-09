use crate::Op::{Acc, Jmp, Nop};
use helpers::read_file;
use std::collections::HashSet;
use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_file("./assets/days/day8.txt")?;
    let program = BootCode::from_str(&input)?;

    // Day 1
    // Run your copy of the boot code.
    // Immediately before any instruction is executed a second time, what value is in the accumulator?
    let mut day1 = program.clone();
    day1.run_until_loop()?;
    println!(
        "Accumulator value before running the same instruction twice: {}",
        day1.accumulator
    );

    // Day 2
    // Fix the program so that it terminates normally by changing exactly one jmp (to nop) or
    // nop (to jmp). What is the value of the accumulator after the program terminates?
    for (idx, &instruction) in program.instructions.iter().enumerate() {
        let substitute = match instruction {
            Acc(_) => continue,
            Jmp(v) => Nop(v),
            Nop(v) => Jmp(v),
        };

        // Check if modified program terminates
        let mut program = program.clone();
        program.instructions[idx] = substitute;
        if ExitStatus::Terminated == program.run_until_loop()? {
            println!(
                "Accumulator value when terminating program, substituting op {}: {}",
                idx, program.accumulator
            );
            break
        }
    }

    Ok(())
}

// Each instruction consists of an operation (acc, jmp, or nop) and an argument (a signed number like +4 or -20).
type Data = i32;
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Op {
    Acc(Data),
    Jmp(Data),
    Nop(Data),
}

// The boot code is represented as a text file with one instruction per line of text.
// For now instructions are [a-z]{3} [+-][0-9]+
// nop +0
// acc +1
// jmp -4
impl FromStr for Op {
    type Err = BootCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, arg) = s.split_at(3);
        if !arg.starts_with(' ') {
            return Err(BootCodeError::ParseOp(s.to_string()));
        }
        // skip leading space
        let arg = arg[1..].parse()?;
        match op {
            "nop" => Ok(Nop(arg)),
            "jmp" => Ok(Jmp(arg)),
            "acc" => Ok(Acc(arg)),
            _ => Err(BootCodeError::ParseOp(s.to_string())),
        }
    }
}

#[derive(Clone, Debug)]
struct BootCode {
    instruction_pointer: usize,
    accumulator: Data,
    instructions: Vec<Op>,
}

#[derive(Copy,Clone,Debug, Eq, PartialEq)]
enum ExitStatus {
    Terminated,
    LoopDetected
}

impl BootCode {
    fn new(state: Vec<Op>) -> Self {
        BootCode {
            instruction_pointer: 0,
            accumulator: 0,
            instructions: state,
        }
    }

    fn fetch_instruction(&self) -> Op {
        self.instructions[self.instruction_pointer]
    }

    // returns true if program terminated
    fn next_cycle(&mut self) -> Result<Option<ExitStatus>, BootCodeError> {
        let op = self.fetch_instruction();

        let offset = match op {
            // acc increases or decreases a single global value called the accumulator by the value given in the argument.
            // For example, acc +7 would increase the accumulator by 7.
            // The accumulator starts at 0.
            // After an acc instruction, the instruction immediately below it is executed next.
            Acc(v) => {
                self.accumulator += v;
                1
            }
            // jmp jumps to a new instruction relative to itself.
            // The next instruction to execute is found using the argument as an offset from the jmp instruction;
            // for example
            //   jmp +2 would skip the next instruction,
            //   jmp +1 would continue to the instruction immediately below it,
            //   jmp -20 would cause the instruction 20 lines above to be executed next.
            Jmp(v) => v,
            //  nop stands for No Operation - it does nothing.
            // The instruction immediately below it is executed next.
            Nop(_) => 1,
        };

        if offset.is_positive() {
            self.instruction_pointer += offset as usize
        } else {
            self.instruction_pointer -= offset.abs() as usize
        }

        // Program terminates if the instruction pointer points to the one right after the last instruction
        if self.instruction_pointer == self.instructions.len() {
            Ok(Some(ExitStatus::Terminated))
        }  else {
            Ok(None)
        }
    }

    // true if the program terminates correctly
    // false if the instruction_pointer points to the same instruction twice
    fn run_until_loop(&mut self) -> Result<ExitStatus, BootCodeError> {
        let mut visited = HashSet::new();
        let mut index = 0_usize;
        loop {
            if visited.contains(&index) {
                break Ok(ExitStatus::LoopDetected);
            } else {
                visited.insert(index);
                if let Some(exit_status) = self.next_cycle()? {
                    break Ok(exit_status);
                }
                index = self.instruction_pointer;
            }
        }
    }
}

impl FromStr for BootCode {
    type Err = BootCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ops = s
            .lines()
            .map(|l| Op::from_str(l))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(BootCode::new(ops))
    }
}

#[derive(Error, Debug, Clone)]
enum BootCodeError {
    #[error("invalid op '{0}'")]
    ParseOp(String),
    #[error("could not parse op argument")]
    ParseArg(#[from] ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use helpers::read_file;

    #[test]
    fn test_parse_ops() {
        let input = read_file("../assets/days/day8_example.txt").unwrap();
        let ops = input
            .lines()
            .map(|l| l.parse::<Op>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let expected = vec![
            Nop(0),
            Acc(1),
            Jmp(4),
            Acc(3),
            Jmp(-3),
            Acc(-99),
            Acc(1),
            Jmp(-4),
            Acc(6),
        ];
        assert_eq!(expected, ops);
    }

    // For example, consider the following program:
    // These instructions are visited in this order:
    //
    // nop +0  | 1
    // acc +1  | 2, 8(!)
    // jmp +4  | 3
    // acc +3  | 6
    // jmp -3  | 7
    // acc -99 |
    // acc +1  | 4
    // jmp -4  | 5
    // acc +6  |
    #[test]
    fn test_example_day1_order() {
        let input = read_file("../assets/days/day8_example.txt").unwrap();
        let mut program = BootCode::from_str(&input).unwrap();

        let expected = vec![
            Nop(0),
            Acc(1),
            Jmp(4),
            Acc(1),
            Jmp(-4),
            Acc(3),
            Jmp(-3),
            Acc(1),
        ];
        let visited_ops: Vec<_> = (0..8)
            .map(|_| {
                let next = program.fetch_instruction();
                program.next_cycle().unwrap();
                next
            })
            .collect();

        assert_eq!(expected, visited_ops);
    }

    // Immediately before the program would run an instruction a second time, the value in the accumulator is 5.
    #[test]
    fn test_example_day1_instruction_index() {
        let input = read_file("../assets/days/day8_example.txt").unwrap();
        let mut program = BootCode::from_str(&input).unwrap();

        program.run_until_loop().unwrap();

        assert_eq!(program.accumulator, 5);
    }
}
