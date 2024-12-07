use regex::Regex;

use crate::AOCDay;

pub struct Day1 {}

fn parse_input(input: &[String]) -> (Vec<i32>, Vec<i32>) {
  let mut first: Vec<i32> = Vec::new();
  let mut second: Vec<i32> = Vec::new();

  for line in input {
    let re = match Regex::new(r"(\d+)\s+(\d+)") {
      Ok(re) => re,
      Err(msg) => panic!("Failed to create regex: {msg}"),
    };

    let Some(caps) = re.captures(line) else {
      panic!("Failed to match regex");
    };

    let left = match caps.get(1) {
      Some(m) => match m.as_str().parse::<i32>() {
        Ok(num) => num,
        Err(msg) => panic!("Failed to parse left number, {msg}"),
      },
      None => panic!("Failed to get left capture group"),
    };

    let right = match caps.get(2) {
      Some(m) => match m.as_str().parse::<i32>() {
        Ok(num) => num,
        Err(msg) => panic!("Failed to parse right number: {msg}"),
      },
      None => panic!("Failed to get right capture group"),
    };

    first.push(left);
    second.push(right);
  }

  (first, second)
}

impl AOCDay for Day1 {
  fn name(&self) -> String {
    "day1".to_string()
  }

  fn test_answer_part1(&self) -> String {
    "11".to_string()
  }

  fn test_answer_part2(&self) -> String {
    "31".to_string()
  }

  fn solve_part1(&self, input: &[String]) -> String {
    let (mut first, mut second) = parse_input(input);

    first.sort_unstable();
    second.sort_unstable();

    let result: i32 = first
      .iter()
      .enumerate()
      .map(|(i, &x)| (second[i] - x).abs())
      .sum();

    result.to_string()
  }

  fn solve_part2(&self, input: &[String]) -> String {
    let (first, second): (Vec<i32>, Vec<i32>) = parse_input(input);

    let result: i32 = first
      .iter()
      .map(|&x| {
        let count = match i32::try_from(second.iter().filter(|&y| *y == x).count()) {
          Ok(count) => count,
          Err(msg) => panic!("Failed to convert count to i32: {msg}"),
        };
        x * count
      })
      .sum();

    result.to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::read_file;

  #[test]
  fn test_part_1() {
    let day = Day1 {};
    assert_eq!(
      "1882714",
      day.solve_part1(&read_file("input/day1/part1.txt"))
    );
  }

  #[test]
  fn test_part_2() {
    let day = Day1 {};
    assert_eq!(
      "19437052",
      day.solve_part2(&read_file("input/day1/part2.txt"))
    );
  }
}
