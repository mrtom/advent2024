use crate::AOCDay;

const PART_1_EXAMPLE: &str = "55312";
const PART_2_EXAMPLE: &str = "FAIL";

fn parse_input(input: &str) -> Vec<&str> {
  input.split(' ').collect()
}

fn apply_rules(input: &mut Vec<&str>) {
  let original_last_idx = input.len() - 1;
  for i in 0..input.len() {
    let idx = original_last_idx - i;
    match input[idx] {
      "0" => input[idx] = "1",
      value if value.len() % 2 == 0 => {
        let (left, right) = value.split_at(value.len() / 2);
        let parsed_right = right.parse::<u64>().unwrap();
        input[idx] = left;
        input.insert(idx + 1,  Box::leak(u64::to_string(&parsed_right).into_boxed_str()));
      },
      value => {
        let new_value = value.parse::<u64>().unwrap() * 2024;
        let new_value_str = u64::to_string(&new_value);
        input[idx] = Box::leak(new_value_str.into_boxed_str()); // TODO
      }
    }
  }
}

pub struct Day11 {}

impl AOCDay for Day11 {
  fn name(&self) -> String {
    "day11".to_string()
  }
  
  fn test_answer_part1(&self) -> String {
    PART_1_EXAMPLE.to_string()
  }
  
  fn test_answer_part2(&self) -> String {
    PART_2_EXAMPLE.to_string()
  }
  
  fn solve_part1(&self, input: &[String]) -> String {
    let mut input = parse_input(&input[0]);
    for _ in 0..25 {
      apply_rules(&mut input);
    }
    input.len().to_string()
  }
  
  fn solve_part2(&self, input: &[String]) -> String {
    let input = parse_input(&input[0]);
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
    let day = Day11 {};
    assert_eq!(
      PART_1_EXAMPLE,
      day.solve_part1(&read_file("input/day11/test1.txt"))
    );
  }
  
  #[test]
  fn test_part_1() {
    let day = Day11 {};
    assert_eq!(
      "218956",
      day.solve_part1(&read_file("input/day11/part1.txt"))
    );
  }

  #[test]
  fn test_part_2_example() {
    let day = Day11 {};
    assert_eq!(
      PART_2_EXAMPLE,
      day.solve_part2(&read_file("input/day11/test1.txt"))
    );
  }
  
  #[test]
  fn test_part_2() {
    let day = Day11 {};
    assert_eq!(
      "TODO",
      day.solve_part2(&read_file("input/day11/part1.txt"))
    );
  }
}