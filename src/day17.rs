use regex::Regex;
use std::string::ToString;

use crate::AOCDay;

const PART_1_EXAMPLE: &str = "4,6,3,5,6,3,5,2,1,0";
const PART_2_EXAMPLE: &str = "117440";

#[derive(Clone)]
struct Computer {
  reg_a: i64,
  reg_b: i64,
  reg_c: i64,
  instruction_ptr: i64,
}

fn parse_input(input: &[String]) -> (Computer, Vec<u8>) {
  let regex = Regex::new(r"^Register [A|B|C]: (\d+)$").unwrap();
  let Some(caps_a) = regex.captures(&input[0]) else {
    panic!("Failed to parse first line");
  };
  let Some(caps_b) = regex.captures(&input[1]) else {
    panic!("Failed to parse second line");
  };
  let Some(caps_c) = regex.captures(&input[2]) else {
    panic!("Failed to parse third line");
  };


  let reg_a = caps_a.get(1).unwrap().as_str().parse::<i64>().unwrap();
  let reg_b = caps_b.get(1).unwrap().as_str().parse::<i64>().unwrap();
  let reg_c = caps_c.get(1).unwrap().as_str().parse::<i64>().unwrap();

  let program_regex = Regex::new(r"Program: ([0-9,]+)").unwrap();
  let Some(caps_program) = program_regex.captures(&input[4]) else {
    panic!("Failed to parse program line");
  };
  let program = caps_program.get(1).unwrap().as_str().split(",").map(|x| x.parse::<u8>().unwrap()).collect();

  (Computer { reg_a, reg_b, reg_c, instruction_ptr: 0 }, program)
}

fn combo_operand(operand: u8, computer: &Computer) -> i64 {
  match operand {
    0 => 0,
    1 => 1,
    2 => 2,
    3 => 3,
    4 => computer.reg_a,
    5 => computer.reg_b,
    6 => computer.reg_c,
    7 => panic!("Reserved. Impossible"),
    _ => panic!("Invalid operand")
  }
}

fn perform_opcode(opcode: u8, operand: u8, computer: &mut Computer) -> Option<String> {
  match opcode {
    0 => {
      computer.reg_a = computer.reg_a / (2 as i64).pow(combo_operand(operand, computer) as u32) as i64;
      computer.instruction_ptr += 2;
    }
    1 => {
      computer.reg_b = computer.reg_b ^ operand as i64;
      computer.instruction_ptr += 2;
    }
    2 => {
      computer.reg_b = combo_operand(operand, computer) % 8;
      computer.instruction_ptr += 2;
    }
    3 => {
      match computer.reg_a {
        0 => {
          computer.instruction_ptr += 2;
        }
        _ => { 
          computer.instruction_ptr = operand as i64; 
        }
      }
    }
    4 => {
      computer.reg_b = computer.reg_b ^ computer.reg_c;
      computer.instruction_ptr += 2;
    }
    5 => {
      let output = combo_operand(operand, computer) % 8;
      computer.instruction_ptr += 2;
      return Some(format!("{output}"));
    }
    6 => {
      computer.reg_b = computer.reg_a / (2 as i64).pow(combo_operand(operand, computer) as u32) as i64;
      computer.instruction_ptr += 2;
    }
    7 => {
      computer.reg_c = computer.reg_a / (2 as i64).pow(combo_operand(operand, computer) as u32) as i64;
      computer.instruction_ptr += 2;
    }
    
    _ => panic!("Invalid opcode")
  }

  None
}

fn run_program(program: &Vec<u8>, computer: &mut Computer) -> Vec<String> {
  let mut output = Vec::new();

  while computer.instruction_ptr < program.len() as i64 {
    let opcode = program[computer.instruction_ptr as usize];
    let operand = program[(computer.instruction_ptr + 1) as usize];
    match perform_opcode(opcode, operand, computer) {
      Some(o) => output.push(o),
      None => {}
    }
  }

  output
}

pub struct Day17 {}

impl AOCDay for Day17 {
  fn name(&self) -> String {
    "day17".to_string()
  }
  
  fn test_answer_part1(&self) -> String {
    PART_1_EXAMPLE.to_string()
  }
  
  fn test_answer_part2(&self) -> String {
    PART_2_EXAMPLE.to_string()
  }
  
  fn solve_part1(&self, input: &[String]) -> String {
    let (mut computer, program) = parse_input(input);
    let output = run_program(&program, &mut computer);
    let result = output.join(",");

    result.to_string()
  }
  
  fn solve_part2(&self, input: &[String]) -> String {
    let (computer, program) = parse_input(input);

    let program_as_str = program.iter().map(ToString::to_string).collect::<Vec<String>>();
    for a in 0..=i64::MAX {
      let mut next_computer = computer.clone();
      next_computer.reg_a = a;
      let output = run_program(&program, &mut next_computer);
      if output == program_as_str {
        return a.to_string();
      }
    }

   panic!("Failed to find a solution");
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::read_file;

  #[test]
  fn test_adv_1() {
    let mut computer = Computer { reg_a: 8, reg_b: 2, reg_c: 9, instruction_ptr: 0 };
    let program = [0,2];
    perform_opcode(program[0], program[1], &mut computer);

    assert!(computer.reg_a == 2);
  }

  #[test]
  fn test_adv_2() {
    let mut computer = Computer { reg_a: 16, reg_b: 2, reg_c: 9, instruction_ptr: 0 };
    let program = [0,5];
    perform_opcode(program[0], program[1], &mut computer);

    assert!(computer.reg_a == 4);
  }

  #[test]
  fn test_jnz_with_zero_reg_a() {
    let mut computer = Computer { reg_a: 0, reg_b: 2, reg_c: 9, instruction_ptr: 0 };
    let program = [3,5];
    perform_opcode(program[0], program[1], &mut computer);

    assert!(computer.instruction_ptr == 2);
  }

  #[test]
  fn test_jnz_with_non_zero_reg_a() {
    let mut computer = Computer { reg_a: 1, reg_b: 2, reg_c: 9, instruction_ptr: 0 };
    let program = [3,5];
    perform_opcode(program[0], program[1], &mut computer);

    assert!(computer.instruction_ptr == 5);
  }

  #[test]
  fn test_one() {
    let mut computer = Computer { reg_a: 1, reg_b: 2, reg_c: 9, instruction_ptr: 0 };
    let program = [2,6];
    perform_opcode(program[0], program[1], &mut computer);

    assert!(computer.reg_b == 1);
  }

  #[test]
  fn test_two() {
    let mut computer = Computer { reg_a: 10, reg_b: 2, reg_c: 9, instruction_ptr: 0 };
    let program: Vec<u8> = vec![5,0,5,1,5,4];
    let output = run_program(&program, &mut computer);

    assert!(output[0] == "0");
    assert!(output[1] == "1");
    assert!(output[2] == "2");
    assert!(output.len() == 3);
  }

  #[test]
  fn test_three() {
    let mut computer = Computer { reg_a: 2024, reg_b: 2, reg_c: 9, instruction_ptr: 0 };
    let program: Vec<u8> = vec![0,1,5,4,3,0];
    let output = run_program(&program, &mut computer);
    let result = output.join(",");

    assert!(computer.reg_a == 0);
    assert!(result == "4,2,5,6,7,7,7,7,3,1,0");
  }

  #[test]
  fn test_four() {
    let mut computer = Computer { reg_a: 0, reg_b: 29, reg_c: 9, instruction_ptr: 0 };
    let program: Vec<u8> = vec![1,7];
    run_program(&program, &mut computer);
    
    assert!(computer.reg_b == 26);
  }

  #[test]
  fn test_five() {
    let mut computer = Computer { reg_a: 0, reg_b: 2024, reg_c: 43690, instruction_ptr: 0 };
    let program: Vec<u8> = vec![4,0];
    run_program(&program, &mut computer);
    
    assert!(computer.reg_b == 44354);
  }

  /////////

  #[test]
  fn test_part_1_example() {
    let day = Day17 {};
    assert_eq!(
      PART_1_EXAMPLE,
      day.solve_part1(&read_file("input/day17/test1.txt"))
    );
  }
  
  #[test]
  fn test_part_1() {
    let day = Day17 {};
    assert_eq!(
      "5,1,3,4,3,7,2,1,7",
      day.solve_part1(&read_file("input/day17/part1.txt"))
    );
  }

  #[test]
  fn test_part_2_example() {
    let day = Day17 {};
    assert_eq!(
      PART_2_EXAMPLE,
      day.solve_part2(&read_file("input/day17/test2.txt"))
    );
  }
  
  // #[test]
  // fn test_part_2() {
  //   let day = Day17 {};
  //   assert_eq!(
  //     "TODO",
  //     day.solve_part2(&read_file("input/day17/part1.txt"))
  //   );
  // }
}