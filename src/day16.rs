use pathfinding::prelude::astar;
use pathfinding::prelude::astar_bag;
use std::collections::HashSet;

use crate::AOCDay;
use crate::utils;

const PART_1_EXAMPLE: &str = "11048";
const PART_2_EXAMPLE: &str = "64";

type Map = Vec<Vec<char>>;
type Location = (i32, i32);

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Reindeer {
  location: Location,
  direction: (i32, i32),
}

#[allow(dead_code)]
fn print_map(map: &Map) {
  for row in map {
    println!("{}", row.iter().collect::<String>());
  }
  println!("\n----------------\n\n");
}

fn parse_input(input: &[String]) -> (Map, Location, Location) {
  let map = input.iter().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();
  
  let start_position = map.iter().enumerate().find_map(|(y, row)| {
    row.iter().enumerate().find_map(|(x, &c)| {
      if c == 'S' {
        Some((utils::usize_to_i32(x).unwrap(), utils::usize_to_i32(y).unwrap()))
      } else {
        None
      }
    })
  }).unwrap_or_else(|| panic!("No start position found"));
  
  let end_position = map.iter().enumerate().find_map(|(y, row)| {
    row.iter().enumerate().find_map(|(x, &c)| {
      if c == 'E' {
        Some((utils::usize_to_i32(x).unwrap(), utils::usize_to_i32(y).unwrap()))
      } else {
        None
      }
    })
  }).unwrap_or_else(|| panic!("No end position found"));
  
  (map, start_position, end_position)
}

fn get_neighbours(map: &Map, reindeer: &Reindeer) -> Vec<(Reindeer, i32)> {
  let mut neighbours = vec![];
  
  for (dx, dy) in &[(0, -1), (0, 1), (-1, 0), (1, 0)] {
    let new_pos = (reindeer.location.0 + dx, reindeer.location.1 + dy);
    if new_pos.1 >= 0 && new_pos.1 < utils::usize_to_i32(map.len()).unwrap() && 
        new_pos.0 >= 0 && new_pos.0 < utils::usize_to_i32( map[0].len()).unwrap() && 
        map[utils::i32_to_usize(new_pos.1).unwrap()][utils::i32_to_usize(new_pos.0).unwrap()] != '#' 
    {
      let cost = if (*dx, *dy) == reindeer.direction {
        1
      } else {
        // Note: If we turn 180 degrees this should be 2001
        // But we don't account for this as turning back on yourself will never be the best option
        1001
      };
      neighbours.push((Reindeer {
        location: new_pos,
        direction: (*dx, *dy),
      }, cost));
    }
  }
  
  neighbours
}

pub struct Day16 {}

impl AOCDay for Day16 {
  fn name(&self) -> String {
    "day16".to_string()
  }
  
  fn test_answer_part1(&self) -> String {
    PART_1_EXAMPLE.to_string()
  }
  
  fn test_answer_part2(&self) -> String {
    PART_2_EXAMPLE.to_string()
  }
  
  fn solve_part1(&self, input: &[String]) -> String {
    let (map, start_position, end_position) = parse_input(input);

    let reindeer = Reindeer{
      location: start_position,
      direction: (1, 0), // East
    };

    match astar(
      &reindeer,
      |reindeer| get_neighbours(&map, reindeer),
      |reindeer| utils::u32_to_i32(end_position.0.abs_diff(reindeer.location.0) + end_position.1.abs_diff(reindeer.location.1)).unwrap(),
      |reindeer| reindeer.location == end_position,
    ) {
      Some((_, cost)) => {
        cost.to_string()
      }
      None => {
        panic!("No path found.");
      }
    }
  }
  
  fn solve_part2(&self, input: &[String]) -> String {
    let (map, start_position, end_position) = parse_input(input);

    let reindeer = Reindeer{
      location: start_position,
      direction: (1, 0), // East
    };

    match astar_bag(
      &reindeer,
      |reindeer| get_neighbours(&map, reindeer),
      |reindeer| utils::u32_to_i32(end_position.0.abs_diff(reindeer.location.0) + end_position.1.abs_diff(reindeer.location.1)).unwrap(),
      |reindeer| reindeer.location == end_position,
    ) {
      Some((solution, _)) => {
        let mut visited = HashSet::new();
        solution.flatten().for_each(|reindeer| {
          visited.insert(reindeer.location);
        });
        let result = visited.len();
        result.to_string()
      }
      None => {
        panic!("No path found.");
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::read_file;
  
  #[test]
  fn test_part_1_example() {
    let day = Day16 {};
    assert_eq!(
      PART_1_EXAMPLE,
      day.solve_part1(&read_file("input/day16/test1.txt"))
    );
  }
  
  #[test]
  fn test_part_1() {
    let day = Day16 {};
    assert_eq!(
      "101492",
      day.solve_part1(&read_file("input/day16/part1.txt"))
    );
  }
  
  #[test]
  fn test_part_2_example() {
    let day = Day16 {};
    assert_eq!(
      PART_2_EXAMPLE,
      day.solve_part2(&read_file("input/day16/test1.txt"))
    );
  }
  
  #[test]
  fn test_part_2() {
    let day = Day16 {};
    assert_eq!(
      "543",
      day.solve_part2(&read_file("input/day16/part1.txt"))
    );
  }
}