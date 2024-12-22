use crate::AOCDay;

const PART_1_EXAMPLE: &str = "37327623";
const PART_2_EXAMPLE: &str = "23";

fn parse_input(input: &[String]) -> Vec<u64> {
  input.iter().map(|line| line.parse().unwrap()).collect()
}

fn mix_number(first: u64, second: u64) -> u64 {
  first ^ second
}

fn prune_number(secret_number: u64) -> u64 {
  secret_number % 16_777_216
}

fn evolve_number(secret_number: u64) -> u64 {
  let first = secret_number * 64;
  let first_mixed = mix_number(secret_number, first);
  let first_pruned = prune_number(first_mixed);

  let second = first_pruned / 32;
  let second_mixed = mix_number(first_pruned, second);
  let second_pruned = prune_number(second_mixed);

  let third = second_pruned * 2048;
  let third_mixed = mix_number(second_pruned, third);

  prune_number(third_mixed)
}

pub struct Day22 {}

impl AOCDay for Day22 {
  fn name(&self) -> String {
    "day22".to_string()
  }

  fn test_answer_part1(&self) -> String {
    PART_1_EXAMPLE.to_string()
  }

  fn test_answer_part2(&self) -> String {
    PART_2_EXAMPLE.to_string()
  }

  fn solve_part1(&self, input: &[String]) -> String {
    let mut secret_numbers = parse_input(input);
    for _ in 0..2000 {
      secret_numbers = secret_numbers
        .iter()
        .map(|secret_number| evolve_number(*secret_number))
        .collect();
    }

    secret_numbers.iter().sum::<u64>().to_string()
  }

  fn solve_part2(&self, input: &[String]) -> String {
    let input = parse_input(input);
    "Not implemented".to_string()
  }
}

#[cfg(test)]
mod tests {
  use std::vec;

  use super::*;
  use crate::utils::read_file;

  #[test]
  fn test_first_example() {
    let mut secret_number = 123;
    let mut results = vec![];

    for _ in 0..10 {
      secret_number = evolve_number(secret_number);
      results.push(secret_number);
    }

    assert_eq!(
      results,
      vec![
        15_887_950, 16_495_136, 527_345, 704_524, 1_553_684, 12_683_156, 11_100_544, 12_249_484,
        7_753_432, 5_908_254,
      ]
    );
  }

  #[test]
  fn test_part_1_example() {
    let day = Day22 {};
    assert_eq!(
      PART_1_EXAMPLE,
      day.solve_part1(&read_file("input/day22/test1.txt"))
    );
  }

  #[test]
  fn test_part_1() {
    let day = Day22 {};
    assert_eq!(
      "20332089158",
      day.solve_part1(&read_file("input/day22/part1.txt"))
    );
  }

  #[test]
  fn test_part_2_example() {
    let day = Day22 {};
    assert_eq!(
      PART_2_EXAMPLE,
      day.solve_part2(&read_file("input/day22/test1.txt"))
    );
  }

  #[test]
  fn test_part_2() {
    let day = Day22 {};
    assert_eq!("TODO", day.solve_part2(&read_file("input/day22/part1.txt")));
  }
}
