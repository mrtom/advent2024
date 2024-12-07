use std::string::ToString;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::AOCDay;

pub struct Day4 {}

#[derive(Debug, Clone, Copy, EnumIter)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
  UpLeft,
  UpRight,
  DownLeft,
  DownRight,
}

fn letter_at_position(grid: &[Vec<String>], position: (usize, usize)) -> Option<&str> {
  let (row, col) = position;
  if row >= grid.len() || col >= grid.len() {
    return None;
  }
  Some(grid[row][col].as_str())
}

fn letter_in_direction(
  grid: &[Vec<String>],
  position: (usize, usize),
  direction: Direction,
) -> Option<&str> {
  if let Some(new_position) = move_in_direction(direction, position) {
    return letter_at_position(grid, new_position);
  }
  None
}

fn move_in_direction(direction: Direction, position: (usize, usize)) -> Option<(usize, usize)> {
  match direction {
    Direction::Left => match position.1 {
      0 => None,
      _ => Some((position.0, position.1 - 1)),
    },
    Direction::Right => Some((position.0, position.1 + 1)),
    Direction::Up => match position.0 {
      0 => None,
      _ => Some((position.0 - 1, position.1)),
    },
    Direction::Down => Some((position.0 + 1, position.1)),
    Direction::UpLeft => match (position.0, position.1) {
      (0, _) | (_, 0) => None,
      _ => Some((position.0 - 1, position.1 - 1)),
    },
    Direction::DownLeft => match position.1 {
      0 => None,
      _ => Some((position.0 + 1, position.1 - 1)),
    },
    Direction::UpRight => match position.0 {
      0 => None,
      _ => Some((position.0 - 1, position.1 + 1)),
    },
    Direction::DownRight => Some((position.0 + 1, position.1 + 1)),
  }
}

fn parse_input(input: &[String]) -> Vec<Vec<String>> {
  let mut result: Vec<Vec<String>> = Vec::new();

  for line in input {
    if !line.is_empty() {
      let line_by_character = line
        .clone()
        .split("")
        .map(ToString::to_string)
        .collect::<Vec<String>>();
      let trimmed_line_by_character = line_by_character[1..line_by_character.len() - 1].to_vec();
      result.push(trimmed_line_by_character);
    }
  }

  result
}

fn next_letter(letter: &str) -> &str {
  match letter {
    "X" => "M",
    "M" => "A",
    "A" => "S",
    _ => panic!("Invalid letter"),
  }
}

fn count_xmas_from_pos(grid: &[Vec<String>], row_idx: usize, col_idx: usize) -> u32 {
  Direction::iter()
    .map(|direction| {
      let mut position = (row_idx, col_idx);
      let mut current_letter = "X";

      loop {
        let (row, col) = position;
        // If out of bounds, return 0
        // Note: Less than zero covered by the usize conversion
        if row >= grid.len() || col >= grid.len() {
          return 0;
        }

        // If current letter does not equal letter at position, return 0
        if grid[row][col].as_str() != current_letter {
          return 0;
        }

        // If current letter is S we've found a match
        if current_letter == "S" {
          return 1;
        }

        // Otherwise move in correct direction
        if let Some(new_position) = move_in_direction(direction, position) {
          position = new_position;
          current_letter = next_letter(current_letter);
        } else {
          return 0;
        }
      }
    })
    .sum()
}

fn is_x_mas_from_pos(grid: &[Vec<String>], row: usize, col: usize) -> bool {
  if letter_at_position(grid, (row, col)) != Some("A") {
    return false;
  }

  let up_left = letter_in_direction(grid, (row, col), Direction::UpLeft);
  let up_right = letter_in_direction(grid, (row, col), Direction::UpRight);
  let down_left = letter_in_direction(grid, (row, col), Direction::DownLeft);
  let down_right = letter_in_direction(grid, (row, col), Direction::DownRight);

  let first = up_left == Some("M") && down_right == Some("S")
    || up_left == Some("S") && down_right == Some("M");
  let second = up_right == Some("M") && down_left == Some("S")
    || up_right == Some("S") && down_left == Some("M");

  first && second
}

impl AOCDay for Day4 {
  fn name(&self) -> String {
    "day4".to_string()
  }

  fn test_answer_part1(&self) -> String {
    "18".to_string()
  }

  fn test_answer_part2(&self) -> String {
    "9".to_string()
  }

  fn solve_part1(&self, input: &[String]) -> String {
    let grid = parse_input(input);

    let mut count = 0;
    grid.iter().enumerate().for_each(|(col_idx, line)| {
      line.iter().enumerate().for_each(|(row_idx, _)| {
        if grid[row_idx][col_idx].as_str() == "X" {
          count += count_xmas_from_pos(&grid, row_idx, col_idx);
        }
      });
    });

    count.to_string()
  }

  fn solve_part2(&self, input: &[String]) -> String {
    let grid = parse_input(input);

    grid
      .iter()
      .enumerate()
      .map(|(row_idx, line)| {
        line
          .iter()
          .enumerate()
          .filter_map(|(col_idx, _)| {
            if is_x_mas_from_pos(&grid, row_idx, col_idx) {
              Some(1)
            } else {
              None
            }
          })
          .count()
      })
      .sum::<usize>()
      .to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::read_file;

  #[test]
  fn test_part_1() {
    let day = Day4 {};
    assert_eq!("2401", day.solve_part1(&read_file("input/day4/part1.txt")));
  }

  #[test]
  fn test_part_2() {
    let day = Day4 {};
    assert_eq!("1822", day.solve_part2(&read_file("input/day4/part1.txt")));
  }
}
