use std::string::ToString;
use crate::AOCDay;
use crate::utils::{isize_to_usize, usize_to_isize};

pub struct Day4 {}

#[derive(Debug, Clone, Copy)]
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

struct DirectionIter {
  current: usize,
}

impl DirectionIter {
  fn new() -> Self {
    DirectionIter { current: 0 }
  }
}

impl Iterator for DirectionIter {
  type Item = Direction;
  
  fn next(&mut self) -> Option<Self::Item> {
    let direction = match self.current {
      0 => Some(Direction::Up),
      1 => Some(Direction::Down),
      2 => Some(Direction::Left),
      3 => Some(Direction::Right),
      4 => Some(Direction::UpLeft),
      5 => Some(Direction::UpRight),
      6 => Some(Direction::DownLeft),
      7 => Some(Direction::DownRight),
      _ => None,
    };
    self.current += 1;
    direction
  }
}

impl Direction {
  fn iter() -> DirectionIter {
    DirectionIter::new()
  }
}

fn move_in_direction(direction: Direction, position: (isize, isize)) -> (isize, isize) {
  match direction {
    Direction::Up => (position.0, position.1 - 1),
    Direction::Down => (position.0, position.1 + 1),
    Direction::Left => (position.0 - 1, position.1),
    Direction::Right => (position.0 + 1, position.1),
    Direction::UpLeft => (position.0 - 1, position.1 - 1),
    Direction::UpRight => (position.0 + 1, position.1 - 1),
    Direction::DownLeft => (position.0 - 1, position.1 + 1),
    Direction::DownRight => (position.0 + 1, position.1 + 1),
  }
}

fn parse_input(input: &[String]) -> Vec<Vec<String>> {
  let mut result: Vec<Vec<String>> = Vec::new();
  
  for line in input {
    if !line.is_empty() {
      let line_by_character = line.clone().split("").map(ToString::to_string).collect::<Vec<String>>();
      let trimmed_line_by_character = line_by_character[1..line_by_character.len()-1].to_vec();            
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

fn count_xmas_from_pos(grid: &[Vec<String>], row_idx: usize, col_idx: usize) -> i32 {
  Direction::iter().map(|direction| {
    if let (Some(row), Some(col)) = (usize_to_isize(row_idx), usize_to_isize(col_idx)) {
      let mut position: (isize, isize) = (row, col);
      let mut current_letter = "X";
      
      loop {
        if let (Some(row), Some(col)) = (isize_to_usize(position.0), isize_to_usize(position.1)) {
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
          position = move_in_direction(direction, position);
          current_letter = next_letter(current_letter);      
        } else {
          return 0;
        } 
      } 
    } else {
      panic!("Impossible row or column index");
    }
  }).sum()
}

fn letter_at_position(grid: &[Vec<String>], position: (isize, isize)) -> Option<&str> {
  if let (Some(row), Some(col)) = (isize_to_usize(position.0), isize_to_usize(position.1)) {
    if row >= grid.len() || col >= grid.len() {
      return None;
    }
    Some(grid[row][col].as_str())
  } else {
    None
  }
}

fn letter_in_direction(grid: &[Vec<String>], position: (isize, isize), direction: Direction) -> Option<&str> {
  let new_position = move_in_direction(direction, position);
  letter_at_position(grid, new_position)
}

fn is_x_mas_from_pos(grid: &[Vec<String>], row_idx: usize, col_idx: usize) -> bool {
  if let (Some(row), Some(col)) = (usize_to_isize(row_idx), usize_to_isize(col_idx)) {
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

    return first && second;
  }

  panic!("Impossible row or column index");
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

    let mut count = 0;
    grid.iter().enumerate().for_each(|(col_idx, line)| {
      line.iter().enumerate().for_each(|(row_idx, _)| {
        if is_x_mas_from_pos(&grid, row_idx, col_idx) { 
          count += 1 
        };
      });
    });
    
    count.to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::read_file;
  
  #[test]
  fn test_part_1() {
    let day = Day4 {};
    assert_eq!(
      "2401",
      day.solve_part1(&read_file("input/day4/part1.txt"))
    );
  }
  
  #[test]
  fn test_part_2() {
    let day = Day4 {};
    assert_eq!(
      "1822",
      day.solve_part2(&read_file("input/day4/part1.txt"))
    );
  }
}