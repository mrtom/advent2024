use crate::AOCDay;
use memoize::memoize;
use std::string::String;

const PART_1_EXAMPLE: &str = "6";
const PART_2_EXAMPLE: &str = "16";

fn parse_input(input: &[String]) -> (Vec<&str>, Vec<String>) {
  let mut splits = input.split(String::is_empty);
  let towels = splits
    .next()
    .unwrap()
    .iter()
    .flat_map(|line| line.split(", "))
    .collect::<Vec<&str>>();
  let designs = splits.next().unwrap().to_vec();

  (towels, designs)
}

fn is_possible(towels: &[&str], design: &str) -> bool {
  let design_length = design.len();
  let mut matches = vec![false; design_length + 1];
  matches[0] = true;

  for i in 1..=design_length {
    for towel in towels {
      if towel.len() <= i && *towel == &design[i - towel.len()..i] && matches[i - towel.len()] {
        matches[i] = true;
        break;
      }
    }
  }

  matches[design_length]
}

#[memoize]
#[allow(clippy::needless_pass_by_value)]
fn valid_paths(towels: Vec<String>, design: String) -> usize {
  if design.is_empty() {
    return 1;
  }

  let mut count = 0;
  for towel in &towels {
    if let Some(prefix) = design.strip_prefix(towel) {
      count += valid_paths(towels.clone(), prefix.into());
    }
  }

  count
}

pub struct Day19 {}

impl AOCDay for Day19 {
  fn name(&self) -> String {
    "day19".to_string()
  }

  fn test_answer_part1(&self) -> String {
    PART_1_EXAMPLE.to_string()
  }

  fn test_answer_part2(&self) -> String {
    PART_2_EXAMPLE.to_string()
  }

  fn solve_part1(&self, input: &[String]) -> String {
    let (towels, designs) = parse_input(input);

    let result = designs
      .iter()
      .filter(|design| is_possible(&towels, design.as_str()))
      .count();
    result.to_string()
  }

  fn solve_part2(&self, input: &[String]) -> String {
    let (towels, designs) = parse_input(input);
    let towels = towels
      .iter()
      .map(ToString::to_string)
      .collect::<Vec<String>>();

    let result = designs
      .iter()
      .map(|design| valid_paths(towels.clone(), design.clone()))
      .sum::<usize>();
    result.to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::read_file;

  #[test]
  fn test_part_1_example() {
    let day = Day19 {};
    assert_eq!(
      PART_1_EXAMPLE,
      day.solve_part1(&read_file("input/day19/test1.txt"))
    );
  }

  #[test]
  fn test_part_1() {
    let day = Day19 {};
    assert_eq!("290", day.solve_part1(&read_file("input/day19/part1.txt")));
  }

  #[test]
  fn test_part_2_example() {
    let day = Day19 {};
    assert_eq!(
      PART_2_EXAMPLE,
      day.solve_part2(&read_file("input/day19/test1.txt"))
    );
  }

  #[test]
  fn test_part_2() {
    let day = Day19 {};
    assert_eq!(
      "712058625427487",
      day.solve_part2(&read_file("input/day19/part1.txt"))
    );
  }
}
