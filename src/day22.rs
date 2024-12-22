use std::collections::{HashMap, VecDeque};

use crate::AOCDay;

const PART_1_EXAMPLE: &str = "37327623";
const PART_2_EXAMPLE: &str = "23";

fn parse_input(input: &[String]) -> Vec<i64> {
  input.iter().map(|line| line.parse().unwrap()).collect()
}

fn mix_number(first: i64, second: i64) -> i64 {
  first ^ second
}

fn prune_number(secret_number: i64) -> i64 {
  secret_number % 16_777_216
}

fn evolve_number(secret_number: i64) -> i64 {
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

fn generate_sequence_key(sequence: &VecDeque<(i64, i64)>) -> String {
  sequence.iter().fold(String::new(), |acc, (_, diff)| {
    if acc.is_empty() {
      return diff.to_string();
    }
    format!("{acc},{diff}")
  })
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

    secret_numbers.iter().sum::<i64>().to_string()
  }

  fn solve_part2(&self, input: &[String]) -> String {
    let mut secret_numbers = parse_input(input);
    let mut sequences: Vec<VecDeque<(i64, i64)>> =
      vec![vec![(0_i64, 0_i64); 4].into(); secret_numbers.len()];
    let mut sequence_to_price: Vec<HashMap<String, i64>> =
      vec![HashMap::new(); secret_numbers.len()];

    for _ in 0..4 {
      let next = secret_numbers
        .iter()
        .enumerate()
        .map(|(idx, secret_number)| {
          sequences[idx].pop_front();
          let next_secret = evolve_number(*secret_number);
          let last_price = secret_numbers[idx] % 10;
          let next_price = next_secret % 10;
          let diff = next_price - last_price;
          sequences[idx].push_back((next_price, diff));

          next_secret
        })
        .collect::<Vec<i64>>();

      secret_numbers = next;
    }

    sequences.iter().enumerate().for_each(|(idx, sequence)| {
      sequence_to_price[idx]
        .entry(generate_sequence_key(sequence))
        .or_insert(sequence[3].0);
    });

    for _ in 4..2000 {
      let next = secret_numbers
        .iter()
        .enumerate()
        .map(|(idx, secret_number)| {
          sequences[idx].pop_front();
          let next_secret = evolve_number(*secret_number);
          let last_price = secret_numbers[idx] % 10;
          let next_price = next_secret % 10;
          let diff = next_price - last_price;
          sequences[idx].push_back((next_price, diff));

          next_secret
        })
        .collect::<Vec<i64>>();

      sequences.iter().enumerate().for_each(|(idx, sequence)| {
        sequence_to_price[idx]
          .entry(generate_sequence_key(sequence))
          .or_insert(sequence[3].0);
      });

      secret_numbers = next;
    }

    let all_sequences = sequence_to_price
      .iter()
      .flat_map(|map_| map_.keys())
      .collect::<Vec<&String>>();

    let mut max_bananas = 0;

    for sequence in all_sequences {
      let bananas = sequence_to_price
        .iter()
        .fold(0, |acc, map| acc + map.get(sequence).unwrap_or(&0));

      if bananas > max_bananas {
        max_bananas = bananas;
      }
    }

    max_bananas.to_string()
  }
}

#[cfg(test)]
mod tests {
  use std::vec;

  use super::*;
  use crate::utils::read_file;

  #[test]
  fn test_generate_sequence_key() {
    let sequence = vec![(0, -3), (6, 6), (5, -1), (4, -1)].into();
    let sequence_key = generate_sequence_key(&sequence);
    assert_eq!(sequence_key, "-3,6,-1,-1");
  }

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
  fn test_part_2_first_example() {
    let mut secret_numbers = vec![123_i64];
    let mut sequences: Vec<VecDeque<(i64, i64)>> =
      vec![vec![(0_i64, 0_i64); 4].into(); secret_numbers.len()];

    for _ in 0..4 {
      let next = secret_numbers
        .iter()
        .enumerate()
        .map(|(idx, secret_number)| {
          sequences[idx].pop_front();
          let next_secret = evolve_number(*secret_number);
          let last_price = secret_numbers[idx] % 10;
          let next_price = next_secret % 10;
          let diff = next_price - last_price;
          sequences[idx].push_back((next_price, diff));

          next_secret
        })
        .collect::<Vec<i64>>();

      secret_numbers = next;
    }

    assert_eq!(sequences, vec![vec![(0, -3), (6, 6), (5, -1), (4, -1)]]);
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
