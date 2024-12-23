use regex::Regex;
use std::collections::HashSet;
use strum_macros::EnumIter;

use crate::AOCDay;

const PART_1_EXAMPLE: &str = "126384";
const PART_2_EXAMPLE: &str = "FAIL";

fn parse_input(input: &[String]) -> Vec<i32> {
  input.iter().map(|line| line.parse().unwrap()).collect()
}

pub struct Day21 {}

impl AOCDay for Day21 {
  fn name(&self) -> String {
    "day21".to_string()
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
    let day = Day21 {};
    assert_eq!(
      PART_1_EXAMPLE,
      day.solve_part1(&read_file("input/day21/test1.txt"))
    );
  }

  #[test]
  fn test_part_1() {
    let day = Day21 {};
    assert_eq!("TODO", day.solve_part1(&read_file("input/day21/part1.txt")));
  }

  #[test]
  fn test_part_2_example() {
    let day = Day21 {};
    assert_eq!(
      PART_2_EXAMPLE,
      day.solve_part2(&read_file("input/day21/test1.txt"))
    );
  }

  #[test]
  fn test_part_2() {
    let day = Day21 {};
    assert_eq!("TODO", day.solve_part2(&read_file("input/day21/part1.txt")));
  }
}
