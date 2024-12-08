use crate::AOCDay;

use std::collections::HashSet;

const PART_1_EXAMPLE: &str = "14";
const PART_2_EXAMPLE: &str = "34";

#[derive(Debug, Copy, Clone)]
struct Antenna {
  row: usize,
  col: usize,
  frequency: char,
}

impl Antenna {
  fn get_location_as_position(&self) -> Position {
    Position {
      row: i64::try_from(self.row).unwrap(),
      col: i64::try_from(self.col).unwrap(),
    }
  }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
  row: i64,
  col: i64,
}

fn parse_input(input: &[String]) -> (usize, usize, Vec<Antenna>) {
  let mut antennas: Vec<Antenna> = Vec::new();

  let row_count = input.len();
  let col_count = input.iter().len();

  for (row_idx, line) in input.iter().enumerate() {
    for (col_idx, char) in line.chars().enumerate() {
      match char {
        '.' => {}
        value => antennas.push(Antenna {
          row: row_idx,
          col: col_idx,
          frequency: value,
        }),
      }
    }
  }

  (row_count, col_count, antennas)
}

fn is_in_bounds(row_count: usize, col_count: usize, position: Position) -> bool {
  let row = position.row;
  let col = position.col;

  row >= 0
    && col >= 0
    && usize::try_from(row)
      .ok()
      .map_or(false, |row_idx| row_idx < row_count)
    && usize::try_from(col)
      .ok()
      .map_or(false, |col_idx| col_idx < col_count)
}

fn get_antinodes(
  row_count: usize,
  col_count: usize,
  first: Position,
  second: Position,
) -> HashSet<Position> {
  let mut antinodes: HashSet<Position> = HashSet::new();

  let row_delta = (first.row - second.row).abs();
  let col_delta = (first.col - second.col).abs();

  let row_direction = if first.row < second.row { 1 } else { -1 };
  let col_direction = if first.col < second.col { 1 } else { -1 };

  let (antinode_1, antinode_2) = (
    Position {
      row: first.row - row_delta * row_direction,
      col: first.col - col_delta * col_direction,
    },
    Position {
      row: second.row + row_delta * row_direction,
      col: second.col + col_delta * col_direction,
    },
  );

  if is_in_bounds(row_count, col_count, antinode_1) {
    antinodes.insert(antinode_1);
  }

  if is_in_bounds(row_count, col_count, antinode_2) {
    antinodes.insert(antinode_2);
  }

  antinodes
}

fn get_antinodes_part2(
  row_count: usize,
  col_count: usize,
  first: Position,
  second: Position,
) -> HashSet<Position> {
  let mut antinodes: HashSet<Position> = HashSet::new();

  let row_delta = (first.row - second.row).abs();
  let col_delta = (first.col - second.col).abs();

  let row_direction = if first.row < second.row { 1 } else { -1 };
  let col_direction = if first.col < second.col { 1 } else { -1 };

  let mut antinode_1 = Position {
    row: first.row - row_delta * row_direction,
    col: first.col - col_delta * col_direction,
  };
  while is_in_bounds(row_count, col_count, antinode_1) {
    antinodes.insert(antinode_1);
    antinode_1 = Position {
      row: antinode_1.row - row_delta * row_direction,
      col: antinode_1.col - col_delta * col_direction,
    };
  }

  let mut antinode_2 = Position {
    row: second.row - row_delta * row_direction,
    col: second.col - col_delta * col_direction,
  };
  while is_in_bounds(row_count, col_count, antinode_2) {
    antinodes.insert(antinode_2);
    antinode_2 = Position {
      row: antinode_2.row - row_delta * row_direction,
      col: antinode_2.col - col_delta * col_direction,
    };
  }

  antinodes
}

pub struct Day8 {}

impl AOCDay for Day8 {
  fn name(&self) -> String {
    "day8".to_string()
  }

  fn test_answer_part1(&self) -> String {
    "14".to_string()
  }

  fn test_answer_part2(&self) -> String {
    "34".to_string()
  }

  fn solve_part1(&self, input: &[String]) -> String {
    let (row_count, col_count, antennas) = parse_input(input);

    let mut antinodes: HashSet<Position> = HashSet::new();
    antennas.iter().enumerate().for_each(|(idx, antenna)| {
      antennas
        .iter()
        .enumerate()
        .filter(|(i, inner)| *i != idx && antenna.frequency == inner.frequency)
        .flat_map(|(_, inner)| {
          get_antinodes(
            row_count,
            col_count,
            antenna.get_location_as_position(),
            inner.get_location_as_position(),
          )
        })
        .for_each(|position| {
          antinodes.insert(position);
        });
    });

    antinodes.len().to_string()
  }

  fn solve_part2(&self, input: &[String]) -> String {
    let (row_count, col_count, antennas) = parse_input(input);

    let mut antinodes: HashSet<Position> = HashSet::new();
    antennas.iter().enumerate().for_each(|(idx, antenna)| {
      antennas
        .iter()
        .enumerate()
        .filter(|(i, inner)| *i != idx && antenna.frequency == inner.frequency)
        .flat_map(|(_, inner)| {
          get_antinodes_part2(
            row_count,
            col_count,
            antenna.get_location_as_position(),
            inner.get_location_as_position(),
          )
        })
        .for_each(|position| {
          antinodes.insert(position);
        });
    });

    antinodes.len().to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::read_file;

  #[test]
  fn test_part_1_example() {
    let day = Day8 {};
    assert_eq!(
      PART_1_EXAMPLE,
      day.solve_part1(&read_file("input/day8/test1.txt"))
    );
  }

  #[test]
  fn test_part_1() {
    let day = Day8 {};
    assert_eq!("359", day.solve_part1(&read_file("input/day8/part1.txt")));
  }

  #[test]
  fn test_part_2_example() {
    let day = Day8 {};
    assert_eq!(
      PART_2_EXAMPLE,
      day.solve_part2(&read_file("input/day8/test1.txt"))
    );
  }

  #[test]
  fn test_part_2() {
    let day = Day8 {};
    assert_eq!("1293", day.solve_part2(&read_file("input/day8/part1.txt")));
  }
}
