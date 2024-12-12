use std::collections::HashSet;
use strum_macros::EnumIter;
use regex::Regex;

use crate::AOCDay;

const PART_1_EXAMPLE: &str = "FAIL";
const PART_2_EXAMPLE: &str = "FAIL";

fn parse_input(input: &[String]) -> Vec<i32> {
  input.iter().map(|line| line.parse().unwrap()).collect()
}

pub struct DayN {}

impl AOCDay for DayN {
  fn name(&self) -> String {
    "dayN".to_string()
  }
  
  fn test_answer_part1(&self) -> String {
    PART_1_EXAMPLE.to_string()
  }
  
  fn test_answer_part2(&self) -> String {
    PART_2_EXAMPLE.to_string()
  }
  
  fn solve_part1(&self, input: &[String]) -> String {
    let input = parse_input(input);
    "Not implemented".to_string()
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
  fn test_part_1_example() {
    let day = DayN {};
    assert_eq!(
      PART_1_EXAMPLE,
      day.solve_part1(&read_file("input/dayN/test1.txt"))
    );
  }
  
  #[test]
  fn test_part_1() {
    let day = DayN {};
    assert_eq!(
      "TODO",
      day.solve_part1(&read_file("input/dayN/part1.txt"))
    );
  }

  #[test]
  fn test_part_2_example() {
    let day = DayN {};
    assert_eq!(
      PART_2_EXAMPLE,
      day.solve_part2(&read_file("input/dayN/test1.txt"))
    );
  }
  
  #[test]
  fn test_part_2() {
    let day = DayN {};
    assert_eq!(
      "TODO",
      day.solve_part2(&read_file("input/dayN/part1.txt"))
    );
  }
}