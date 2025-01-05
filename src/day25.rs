use crate::AOCDay;

const PART_1_EXAMPLE: &str = "3";
const PART_2_EXAMPLE: &str = "SKIP";

fn parse_input(input: &[String]) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
  let mut locks = Vec::new();
  let mut keys = Vec::new();

  input.split(String::is_empty).for_each(|schematic| {
    let grid = schematic
      .iter()
      .map(|line| line.chars().collect::<Vec<char>>())
      .collect::<Vec<Vec<char>>>();
    let col_lengths = (0..grid[0].len())
      .map(|col_idx| grid.iter().map(|row| row[col_idx]).collect::<Vec<char>>())
      .map(|col| col.iter().filter(|c| **c == '#').count() - 1)
      .collect::<Vec<usize>>();

    let is_lock = grid[0].iter().filter(|c| **c == '.').count() == 0;
    if is_lock {
      locks.push(col_lengths);
    } else {
      keys.push(col_lengths);
    }
  });

  (locks, keys)
}

pub struct Day25 {}

impl AOCDay for Day25 {
  fn name(&self) -> String {
    "day25".to_string()
  }

  fn test_answer_part1(&self) -> String {
    PART_1_EXAMPLE.to_string()
  }

  fn test_answer_part2(&self) -> String {
    PART_2_EXAMPLE.to_string()
  }

  fn solve_part1(&self, input: &[String]) -> String {
    let (keys, locks) = parse_input(input);
    let mut result = 0;
    for key in keys {
      for lock in &locks {
        if lock
          .iter()
          .enumerate()
          .filter(|(idx, lock_val)| key[*idx] + *lock_val <= 5)
          .count()
          == lock.len()
        {
          result += 1;
        }
      }
    }
    result.to_string()
  }

  fn solve_part2(&self, _input: &[String]) -> String {
    "SKIP".to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::read_file;

  #[test]
  fn test_part_1_example() {
    let day = Day25 {};
    assert_eq!(
      PART_1_EXAMPLE,
      day.solve_part1(&read_file("input/day25/test1.txt"))
    );
  }

  #[test]
  fn test_part_1() {
    let day = Day25 {};
    assert_eq!("2978", day.solve_part1(&read_file("input/day25/part1.txt")));
  }
}
