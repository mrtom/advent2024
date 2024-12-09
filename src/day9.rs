use crate::AOCDay;

const PART_1_EXAMPLE: &str = "1928";
const PART_2_EXAMPLE: &str = "2858";

const MARKER: usize = usize::MAX;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum BlockType {
  File,
  FreeSpace,
}

struct Block {
  id: usize,
  length: usize,
  r#type: BlockType,
}

fn parse_input(input: &[String]) -> Vec<Block> {
  let line = &input[0];
  line.chars().map(|char| format!("{char}").parse::<usize>().unwrap()).enumerate().map(|(idx, value)| {
    if idx % 2 == 0 {
      Block {
        id: idx / 2,
        length: value,
        r#type:  BlockType::File,      }
    } else {
      Block {
        id: 0, // TODO: Not needed
        length: value,
        r#type: BlockType::FreeSpace,
      }
    }
  }).collect()
}

fn expand(input: &[Block]) -> Vec<usize> {
  let mut output: Vec<usize> = Vec::new();
  for block in input {
    let value = match block.r#type {
      BlockType::File => block.id,
      BlockType::FreeSpace => MARKER,
    };
    output.resize(output.len() + block.length, value);
  }
  output
}

pub struct Day9 {}

impl AOCDay for Day9 {
  fn name(&self) -> String {
    "day9".to_string()
  }
  
  fn test_answer_part1(&self) -> String {
    PART_1_EXAMPLE.to_string()
  }
  
  fn test_answer_part2(&self) -> String {
    PART_2_EXAMPLE.to_string()
  }
  
  fn solve_part1(&self, input: &[String]) -> String {
    let input = parse_input(input);
    let mut expanded = expand(&input);

    let mut first_free_block = expanded.iter().position(|&v| v == MARKER).unwrap();
    let mut last_file_block = expanded.iter().rposition(|&v| v != MARKER).unwrap();
    while first_free_block < last_file_block {
      expanded[first_free_block] = expanded[last_file_block];
      expanded[last_file_block] = MARKER;

      first_free_block = expanded.iter().position(|&v| v == MARKER).unwrap();
      last_file_block = expanded.iter().rposition(|&v| v != MARKER).unwrap();
    }

    let result: usize = expanded.iter().enumerate().map(|(idx, value)| {
      match value {
       &v if v == MARKER => 0,
        _ => idx * value
      }
    }).collect::<Vec<usize>>().iter().sum();

    result.to_string()
  }
  
  fn solve_part2(&self, input: &[String]) -> String {
    let input = parse_input(input);
    "Not implemented".to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::read_file;
  
  #[test]
  fn test_something() {
    let result = "";
    assert_eq!(result, "");
  }

  #[test]
  fn test_part_1_example() {
    let day = Day9 {};
    assert_eq!(
      PART_1_EXAMPLE,
      day.solve_part1(&read_file("input/day9/test1.txt"))
    );
  }
  
  #[test]
  fn test_part_1() {
    let day = Day9 {};
    assert_eq!(
      "6288707484810",
      day.solve_part1(&read_file("input/day9/part1.txt"))
    );
  }

  #[test]
  fn test_part_2_example() {
    let day = Day9 {};
    assert_eq!(
      PART_2_EXAMPLE,
      day.solve_part2(&read_file("input/day9/test1.txt"))
    );
  }
  
  #[test]
  fn test_part_2() {
    let day = Day9 {};
    assert_eq!(
      "TODO",
      day.solve_part2(&read_file("input/day9/part1.txt"))
    );
  }
}