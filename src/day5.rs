use std::result::Result;

use regex::Regex;

use crate::AOCDay;

pub struct Day5 {}

struct Rule {
  page_before: i32,
  page_after: i32,
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
        rules.push(Rule {
          page_before,
          page_after,
        });
      }
    } else {
      let update = update_re
        .captures_iter(line)
        .filter_map(|capture| capture.get(1).map(|m| m.as_str().parse::<i32>()))
        .filter_map(Result::ok)
        .collect();
      updates.push(update);
    }
  }

  Input { rules, updates }
}

fn does_update_follow_rule(update: &Update, rule: &Rule) -> bool {
  let before = update.iter().position(|x| *x == rule.page_before);
  let after = update.iter().position(|x| *x == rule.page_after);
  match (before, after) {
    (Some(before), Some(after)) => before < after,
    (None, Some(_) | None) | (Some(_), None) => true,
  }
}

fn is_update_correct(update: &Update, rules: &[Rule]) -> bool {
  rules
    .iter()
    .all(|rule| does_update_follow_rule(update, rule))
}

fn get_middle_value(update: &Update) -> i32 {
  match update.get(update.len() / 2) {
    Some(&middle) => middle,
    None => panic!("Update is empty. This should be impossible."),
  }
}

fn fix_update(update: &Update, rules: &[Rule]) -> Update {
  let mut fixed_update = update.clone();
  while !is_update_correct(&fixed_update, rules) {
    if let Some(next_broken_rule) = rules
      .iter()
      .find(|rule| !does_update_follow_rule(&fixed_update, rule))
    {
      if let (Some(before_idx), Some(after_idx)) = (
        fixed_update
          .iter()
          .position(|x| *x == next_broken_rule.page_before),
        fixed_update
          .iter()
          .position(|x| *x == next_broken_rule.page_after),
      ) {
        let element = fixed_update.remove(before_idx);
        fixed_update.insert(after_idx, element);
      } else {
        panic!("Broken rule not found in update");
      }
    } else {
      panic!("No broken rule found");
    }
  }
  fixed_update
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
    let result = input
      .updates
      .iter()
      .filter(|update| is_update_correct(update, &input.rules))
      .map(get_middle_value)
      .sum::<i32>();

    result.to_string()
  }

  fn solve_part2(&self, input: &[String]) -> String {
    let input = parse_input(input);
    let bad_updates = input
      .updates
      .iter()
      .filter(|update| !is_update_correct(update, &input.rules));

    let result = bad_updates
      .map(|update| fix_update(update, &input.rules))
      .map(|update| get_middle_value(&update))
      .sum::<i32>();

    result.to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::read_file;

  #[test]
  fn test_part_1() {
    let day = Day5 {};
    assert_eq!("7365", day.solve_part1(&read_file("input/day5/part1.txt")));
  }

  #[test]
  fn test_part_2() {
    let day = Day5 {};
    assert_eq!("5770", day.solve_part2(&read_file("input/day5/part1.txt")));
  }
}
