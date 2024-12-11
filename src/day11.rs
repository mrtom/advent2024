use crate::AOCDay;
use std::collections::HashMap;

const PART_1_EXAMPLE: &str = "55312";
const PART_2_EXAMPLE: &str = "65601038650482";

fn parse_input(input: &str) -> Vec<usize> {
  input.split(' ').map(|s| s.parse().unwrap()).collect()
}

fn apply_rules(rock: usize) -> (usize, Option<usize>) {
  match rock {
    0 => (1, None),
    value => {
      let as_str = value.to_string();
      if as_str.len() % 2 == 0 {
        let (left, right) = as_str.split_at(as_str.len() / 2);
        (left.parse().unwrap(), Some(right.parse().unwrap()))
      } else {
        (value * 2024, None)
      }
    }
  }
}

fn blink(input: &mut Vec<usize>) {
  let original_last_idx = input.len() - 1;
  for i in 0..input.len() {
    let idx = original_last_idx - i;
    let (left, right) = apply_rules(input[idx]);
    input[idx] = left;
    if let Some(new_rock) = right {
      input.insert(idx + 1, new_rock);
    }
  }
}

fn blink_part2(map: &HashMap<usize, usize>) -> HashMap<usize, usize> {
  let mut new_map = HashMap::new();

  for (rock, count) in map {
    let (left, right) = apply_rules(*rock);

    *new_map.entry(left).or_insert(0) += count;

    if let Some(new_rock) = right {
      *new_map.entry(new_rock).or_insert(0) += count;
    }
  }

  new_map
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
      blink(&mut input);
    }
    input.len().to_string()
  }
  
  fn solve_part2(&self, input: &[String]) -> String {
    let input = parse_input(&input[0]);
    let mut map: HashMap<usize, usize> = input.iter().map(|value| (*value, 1)).collect();

    for _ in 0..75 {
      map = blink_part2(&map);
    }

    let result = map.values().sum::<usize>();
    result.to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::read_file;
  
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
      "259593838049805",
      day.solve_part2(&read_file("input/day11/part1.txt"))
    );
  }
}