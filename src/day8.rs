use crate::AOCDay;

use std::collections::HashSet;

type Map = Vec<Vec<Cell>>;

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

#[derive(Debug, Clone)]
struct Cell {
  row: usize,
  col: usize,
  tile: char,
  antinodes: Vec<char>,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
  row: i64,
  col: i64,
}

impl Cell {
  fn new(row: usize, col: usize, tile: char) -> Cell {
    Cell {
      row,
      col,
      tile,
      antinodes: Vec::new(),
    }
  }
}

fn parse_input(input: &[String]) -> (Map, Vec<Antenna>) {
  let mut map: Vec<Vec<Cell>> = Vec::new();
  let mut antennas: Vec<Antenna> = Vec::new();

  for (row_idx, line) in input.iter().enumerate() {
    let mut row: Vec<Cell> = Vec::new();
    for (col_idx, char) in line.chars().enumerate() {
      row.push(Cell::new(row_idx, col_idx, char));
      match char {
        '.' => {}
        value => antennas.push(Antenna {
          row: row_idx,
          col: col_idx,
          frequency: value,
        }),
      }
    }
    map.push(row);
  }

  (map, antennas)
}

fn get_cell(map: &Map, position: Position) -> Option<&Cell> {
  if position.row < 0 || position.col < 0 {
    return None;
  }

  let Some(row_idx) = usize::try_from(position.row).ok() else {
    return None;
  };
  let Some(col_idx) = usize::try_from(position.col).ok() else {
    return None;
  };

  match map.get(row_idx) {
    Some(row) => row.get(col_idx),
    None => None,
  }
}

fn get_antinodes(map: &Map, first: Position, second: Position) -> HashSet<Position> {
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

  if let Some(_cell) = get_cell(map, antinode_1) {
    antinodes.insert(antinode_1);
  }

  if let Some(_cell) = get_cell(map, antinode_2) {
    antinodes.insert(antinode_2);
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
    panic!("Not implemented");
    "FAIL".to_string()
  }

  fn solve_part1(&self, input: &[String]) -> String {
    let (map, antennas) = parse_input(input);

    let mut antinodes: HashSet<Position> = HashSet::new();
    antennas.iter().enumerate().for_each(|(idx, antenna)| {
      antennas
        .iter()
        .enumerate()
        .filter(|(i, inner)| *i != idx && antenna.frequency == inner.frequency)
        .flat_map(|(_, inner)| {
          get_antinodes(
            &map,
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
    panic!("Not implemented");
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
    let day = Day8 {};
    assert_eq!("359", day.solve_part1(&read_file("input/day8/part1.txt")));
  }

  #[test]
  fn test_part_2() {
    let day = Day8 {};
    assert_eq!("TODO", day.solve_part2(&read_file("input/day8/part1.txt")));
  }
}
