use std::collections::HashSet;

use crate::AOCDay;

const PART_1_EXAMPLE: &str = "36";
const PART_2_EXAMPLE: &str = "81";

type Map = Vec<Vec<u32>>;
type Position = (usize, usize);

fn parse_input(input: &[String]) -> Vec<Vec<u32>> {
  input
    .iter()
    .map(|str| {
      str
        .chars()
        .map(|char| char.to_digit(10).unwrap_or(u32::MAX))
        .collect::<Vec<u32>>()
    })
    .collect::<Vec<Vec<u32>>>()
}

fn find_valid_neighbours(
  map: &Map,
  current_position: Position,
  previous_position: Position,
) -> Vec<Position> {
  let (row, col) = current_position;
  let (prev_row, prev_col) = previous_position;
  let mut neighbors = Vec::new();

  if row > 0 && row - 1 != prev_row {
    neighbors.push((row - 1, col)); // Up
  }
  if row < map.len() - 1 && row + 1 != prev_row {
    neighbors.push((row + 1, col)); // Down
  }
  if col > 0 && col - 1 != prev_col {
    neighbors.push((row, col - 1)); // Left
  }
  if col < map[0].len() - 1 && col + 1 != prev_col {
    neighbors.push((row, col + 1)); // Right
  }

  neighbors
}

fn walk(
  map: &Map,
  peaks: &mut HashSet<Position>,
  trails: &mut HashSet<String>,
  current_position: Position,
  previous_position: Position,
  route: &str,
) {
  let (row, col) = current_position;

  if map[row][col] == 9 {
    peaks.insert(current_position);
    trails.insert(format!("{route}:{row},{col},"));
    return;
  }

  let neighbours = find_valid_neighbours(map, current_position, previous_position);
  for &neighbour in &neighbours {
    if map[neighbour.0][neighbour.1] == map[row][col] + 1 {
      let new_route = format!("{route}:{row},{col},");
      walk(map, peaks, trails, neighbour, current_position, &new_route);
    }
  }
}

fn score_trail(map: &Map, trailhead: Position) -> (usize, usize) {
  let mut peaks: HashSet<Position> = HashSet::new();
  let mut trails: HashSet<String> = HashSet::new();
  walk(
    map,
    &mut peaks,
    &mut trails,
    trailhead,
    (usize::MAX, usize::MAX),
    "",
  );

  (peaks.len(), trails.len())
}

pub struct Day10 {}

impl AOCDay for Day10 {
  fn name(&self) -> String {
    "day10".to_string()
  }

  fn test_answer_part1(&self) -> String {
    PART_1_EXAMPLE.to_string()
  }

  fn test_answer_part2(&self) -> String {
    PART_2_EXAMPLE.to_string()
  }

  fn solve_part1(&self, input: &[String]) -> String {
    let input = parse_input(input);

    let result = input
      .iter()
      .enumerate()
      .flat_map(|(row, _)| {
        input[row]
          .iter()
          .enumerate()
          .filter(|(_, value)| **value == 0)
          .map(|(col, _)| score_trail(&input, (row, col)).0)
          .collect::<Vec<usize>>()
      })
      .sum::<usize>();

    result.to_string()
  }

  fn solve_part2(&self, input: &[String]) -> String {
    let input = parse_input(input);

    let result = input
      .iter()
      .enumerate()
      .flat_map(|(row, _)| {
        input[row]
          .iter()
          .enumerate()
          .filter(|(_, value)| **value == 0)
          .map(|(col, _)| score_trail(&input, (row, col)).1)
          .collect::<Vec<usize>>()
      })
      .sum::<usize>();

    result.to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::read_file;

  #[test]
  fn test_part_1_example() {
    let day = Day10 {};
    assert_eq!(
      PART_1_EXAMPLE,
      day.solve_part1(&read_file("input/day10/test1.txt"))
    );
  }

  #[test]
  fn test_part_1() {
    let day = Day10 {};
    assert_eq!("557", day.solve_part1(&read_file("input/day10/part1.txt")));
  }

  #[test]
  fn test_3() {
    let day = Day10 {};
    let input = vec![
      ".....0.".to_string(),
      "..4321.".to_string(),
      "..5..2.".to_string(),
      "..6543.".to_string(),
      "..7..4.".to_string(),
      "..8765.".to_string(),
      "..9....".to_string(),
    ];

    assert_eq!("3", day.solve_part2(&input));
  }

  #[test]
  fn test_13() {
    let day = Day10 {};
    let input = vec![
      "..90..9".to_string(),
      "...1.98".to_string(),
      "...2..7".to_string(),
      "6543456".to_string(),
      "765.987".to_string(),
      "876....".to_string(),
      "987....".to_string(),
    ];

    assert_eq!("13", day.solve_part2(&input));
  }

  #[test]
  fn test_part_2_example() {
    let day = Day10 {};
    assert_eq!(
      PART_2_EXAMPLE,
      day.solve_part2(&read_file("input/day10/test1.txt"))
    );
  }

  #[test]
  fn test_part_2() {
    let day = Day10 {};
    assert_eq!("1062", day.solve_part2(&read_file("input/day10/part1.txt")));
  }
}
