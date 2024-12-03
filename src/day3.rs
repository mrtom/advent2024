
use core::panic;

use regex::Regex;

use crate::AOCDay;

pub struct Day3 {}

fn parse_line(line: &str) -> i32 {
  let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
  let result: i32 = re.captures_iter(line).map(|capture| {
    let first = capture.get(1).map(|m| m.as_str().parse::<i32>());
    let second = capture.get(2).map(|m| m.as_str().parse::<i32>());
    if let (Some(Ok(first)), Some(Ok(second))) = (first, second) {
      first * second
    } else {
      0
    }
  })
  .sum();
  
  result
}

fn parse_line_p2(line: &str) -> i32 {
  let re = Regex::new(r"mul\((\d+),(\d+)\)|(don)'t\(\)|(do)\(\)").unwrap();
  
  let mut result = 0;
  let mut should_multiply = true;
  
  for capture in re.captures_iter(line) {
    match (
      capture.get(1).map(|m| m.as_str().parse::<i32>()), 
      capture.get(2).map(|m| m.as_str().parse::<i32>()), 
      capture.get(3).map(|m| m.as_str()), 
      capture.get(4).map(|m| m.as_str())
    ) {
      (Some(first), Some(second), None, None) => {
        if should_multiply {
          if let (Ok(first), Ok(second)) = (first, second) {
            result += first * second;
          } else {
            panic!("Error parsing integers");
          }
        }
      }
      (None, None, Some("don"), None) => should_multiply = false,
      (None, None, None, Some("do")) => should_multiply = true,
      _ => panic!("impossible"),
    }
  }
  
  result
}

impl AOCDay for Day3 {
  fn name(&self) -> String {
    "day3".to_string()
  }
  
  fn test_answer_part1(&self) -> String {
    "161".to_string()
  }
  
  fn test_answer_part2(&self) -> String {
    "48".to_string()
  }
  
  fn solve_part1(&self, input: &[String]) -> String {
    input.iter()
    .map(|line| parse_line(line))
    .sum::<i32>()
    .to_string()
  }
  
  fn solve_part2(&self, input: &[String]) -> String {
    let input_single_line = input.join("");
    parse_line_p2(&input_single_line).to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::read_file;
  
  #[test]
  fn test_parse_line() {
    assert_eq!(
      161, 
      parse_line("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
    );
  }
  
  #[test]
  fn test_parse_line_p2() {
    assert_eq!(
      48,
      parse_line_p2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))")
    );
  }
  
  #[test]
  fn test_part_1() {
    let day = Day3 {};
    assert_eq!(
      "167090022",
      day.solve_part1(&read_file("input/day3/part1.txt"))
    );
  }
  
  #[test]
  fn test_part_2() {
    let day = Day3 {};
    assert_eq!(
      "89823704",
      day.solve_part2(&read_file("input/day3/part1.txt"))
    );
  }
}