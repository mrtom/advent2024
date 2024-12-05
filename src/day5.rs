use std::ops::Index;

use regex::Regex;

use crate::AOCDay;

pub struct Day5 {}

struct Rule {
  pageBefore: i32,
  pageAfter: i32,
}

type Update = Vec<i32>;

struct Input {
  rules: Vec<Rule>,
  updates: Vec<Update>,
}

fn parse_input(input: &[String]) -> Input {
  let mut rules = Vec::new();
  let mut updates = Vec::new();
  
  let rule_re = Regex::new(r"(\d+)\|(\d+)").unwrap();
  let update_re = Regex::new(r"(\d+)").unwrap();
  
  for line in input {
    if line.is_empty() {
      continue;
    }
    
    if let Some(capture) = rule_re.captures(line) {
      let page_before = capture.get(1).map(|m| m.as_str().parse::<i32>());
      let page_after = capture.get(2).map(|m| m.as_str().parse::<i32>());
      if let (Some(Ok(page_before)), Some(Ok(page_after))) = (page_before, page_after) {
        rules.push(Rule { pageBefore: page_before, pageAfter: page_after });
      }
    } else {
      let update = update_re.captures_iter(line).map(|capture| {
        capture.get(1).map(|m| m.as_str().parse::<i32>())
      })
      .filter_map(|x| x)
      .filter_map(|x| x.ok())
      .collect();
      updates.push(update);
    }
  }
  
  Input { rules, updates }
}

fn is_update_correct(update: &Update, rules: &[Rule]) -> bool {
  rules.iter().all(|rule| {
    let before = update.iter().position(|x| *x == rule.pageBefore);
    let after = update.iter().position(|x| *x == rule.pageAfter);
    match (before, after) {
      (Some(before), Some(after)) => before < after,
      (None, Some(_) | None) | (Some(_), None) => true,
    }
  })
}

fn get_middle_value(update: &Update) -> i32 {
  let middle_idx = ((update.len() as f64) / 2.0).floor() as usize;
  update[middle_idx]
}

impl AOCDay for Day5 {
  fn name(&self) -> String {
    "day5".to_string()
  }
  
  fn test_answer_part1(&self) -> String {
    "143".to_string()
  }
  
  fn test_answer_part2(&self) -> String {
    "123".to_string()
  }
  
  fn solve_part1(&self, input: &[String]) -> String {
    let input = parse_input(input);
    let result = input.updates.iter()
    .filter(|update| is_update_correct(update, &input.rules))
    .map(get_middle_value)
    .sum::<i32>();
    
    result.to_string()
  }
  
  fn solve_part2(&self, input: &[String]) -> String {
    "".to_string()
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
  fn test_part_1() {
    let day = Day5 {};
    assert_eq!(
      "7365",
      day.solve_part1(&read_file("input/day5/part1.txt"))
    );
  }
  
  #[test]
  fn test_part_2() {
    let day = Day5 {};
    assert_eq!(
      "TODO",
      day.solve_part2(&read_file("input/day5/part1.txt"))
    );
  }
}