use crate::AOCDay;

use itertools::Itertools;

fn parse_input(input: &[String]) -> Vec<(u64, Vec<u64>)> {
  input
    .iter()
    .map(|x| {
      if let Some((test, values)) = x.split(": ").collect_tuple() {
        let Some(test) = test.parse::<u64>().ok() else {
          panic!("Failed to parse test value into u64");
        };
        let values = values
          .split(' ')
          .map(|x| match x.parse::<u64>() {
            Ok(num) => num,
            Err(msg) => panic!("Failed to parse values into Vec<u64>: {msg}"),
          })
          .collect();
        (test, values)
      } else {
        panic!("Invalid input");
      }
    })
    .collect()
}

fn concatenate_numbers(a: u64, b: u64) -> u64 {
  let concatenated = format!("{a}{b}");
  match concatenated.parse::<u64>() {
    Ok(num) => num,
    Err(msg) => panic!("Failed to parse concatenated string into u64: {msg}"),
  }
}

fn is_valid(test: u64, values: &[u64], current: u64) -> bool {
  if values.is_empty() {
    return test == current;
  }

  let Some((next, new_values)) = values.split_first() else {
    panic!("Impossible!");
  };

  is_valid(test, new_values, current + next) || is_valid(test, new_values, current * next)
}

fn is_valid_part2(test: u64, values: &[u64], current: u64) -> bool {
  if values.is_empty() {
    return test == current;
  }
  let Some((next, new_values)) = values.split_first() else {
    panic!("Impossible!");
  };

  is_valid_part2(test, new_values, current + next)
    || is_valid_part2(test, new_values, current * next)
    || is_valid_part2(test, new_values, concatenate_numbers(current, *next))
}

pub struct Day7 {}

impl AOCDay for Day7 {
  fn name(&self) -> String {
    "day7".to_string()
  }

  fn test_answer_part1(&self) -> String {
    "3749".to_string()
  }

  fn test_answer_part2(&self) -> String {
    "11387".to_string()
  }

  fn solve_part1(&self, input: &[String]) -> String {
    let input = parse_input(input);
    input
      .iter()
      .filter(|row| is_valid(row.0, row.1.split_first().unwrap().1, row.1[0]))
      .map(|x| x.0)
      .sum::<u64>()
      .to_string()
  }

  fn solve_part2(&self, input: &[String]) -> String {
    let input = parse_input(input);
    input
      .iter()
      .filter(|row| is_valid_part2(row.0, row.1.split_first().unwrap().1, row.1[0]))
      .map(|x| x.0)
      .sum::<u64>()
      .to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::read_file;

  #[test]
  fn test_part_1() {
    let day = Day7 {};
    assert_eq!(
      "1153997401072",
      day.solve_part1(&read_file("input/day7/part1.txt"))
    );
  }

  #[test]
  fn test_part_2() {
    let day = Day7 {};
    assert_eq!(
      "97902809384118",
      day.solve_part2(&read_file("input/day7/part1.txt"))
    );
  }
}
