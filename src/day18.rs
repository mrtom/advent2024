use core::panic;

use pathfinding::prelude::astar;
use crate::{utils, AOCDay};

const PART_1_EXAMPLE: &str = "22";
const PART_2_EXAMPLE: &str = "6,1";

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Point {
  x: i32,
  y: i32,
}

impl std::fmt::Display for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{},{}", self.x, self.y)
  }
}

type Map = Vec<Vec<char>>;

#[allow(dead_code)]
fn print_map(map: &Map) {
  for row in map {
    println!("{}", row.iter().collect::<String>());
  }
  println!("----------------\n");
}

fn parse_input(input: &[String]) -> Vec<Point> {
  input.iter().map(|line| { 
    let split = line.split(',').collect::<Vec<&str>>();
    Point {
      x: split[0].parse().unwrap(),
      y: split[1].parse().unwrap(),
    }
}).collect()
}

fn is_test(input: &[Point]) -> bool {
  input.len() == 25
}

fn get_neighbours(map: &Map, location: Point) -> Vec<(Point, i32)> {
  let mut neighbours = vec![];
  
  for (dx, dy) in &[(0, -1), (0, 1), (-1, 0), (1, 0)] {
    let new_pos = (location.x + dx, location.y + dy);
    let col_map = utils::usize_to_i32_x(map[0].len());
    let row_max = utils::usize_to_i32_x(map.len());
    
    if new_pos.1 >= 0 && new_pos.1 < row_max && 
        new_pos.0 >= 0 && new_pos.0 < col_map && 
        map[utils::i32_to_usize_x(new_pos.1)][utils::i32_to_usize_x(new_pos.0)] != '#' 
    {
      neighbours.push((Point {
        x: new_pos.0,
        y: new_pos.1,
      }, 1));
    }
  }
  
  neighbours
}

pub struct Day18 {}

impl AOCDay for Day18 {
  fn name(&self) -> String {
    "day18".to_string()
  }
  
  fn test_answer_part1(&self) -> String {
    PART_1_EXAMPLE.to_string()
  }
  
  fn test_answer_part2(&self) -> String {
    PART_2_EXAMPLE.to_string()
  }
  
  fn solve_part1(&self, input: &[String]) -> String {
    let input = parse_input(input);
    let is_test_run = is_test(&input);

    let grid_dim = if is_test_run { 7 } else { 71 };
    let num_steps = if is_test_run { 12 } else { 1024 };

    let mut map = vec![vec!['.'; grid_dim]; grid_dim];

    for next_byte in input.iter().take(num_steps) {
      let col_idx = utils::i32_to_usize(next_byte.x).unwrap();
      let row_idx = utils::i32_to_usize(next_byte.y).unwrap();

      map[row_idx][col_idx] = '#';
    };

    let start =  Point { x: 0, y: 0 };
    let end = Point { x: utils::usize_to_i32_x(grid_dim - 1), y: utils::usize_to_i32_x(grid_dim - 1) };

    match astar(
      &start,
      |location| get_neighbours(&map, *location),
      |location| utils::u32_to_i32(end.x.abs_diff(location.x) + end.y.abs_diff(location.y)).unwrap(),
      |location| location == &end,
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
    let input = parse_input(input);
    let is_test_run = is_test(&input);

    let grid_dim = if is_test_run { 7 } else { 71 };
    let num_steps = if is_test_run { 12 } else { 1024 };

    let mut map = vec![vec!['.'; grid_dim]; grid_dim];

    for next_byte in input.iter().take(num_steps) {
      let col_idx = utils::i32_to_usize(next_byte.x).unwrap();
      let row_idx = utils::i32_to_usize(next_byte.y).unwrap();

      map[row_idx][col_idx] = '#';
    };

    let start =  Point { x: 0, y: 0 };
    let end = Point { x: utils::usize_to_i32_x(grid_dim - 1), y: utils::usize_to_i32_x(grid_dim - 1) };

    for (step, next_byte) in input.iter().enumerate().take(input.len()).skip(num_steps) {
      let col_idx = utils::i32_to_usize(next_byte.x).unwrap();
      let row_idx = utils::i32_to_usize(next_byte.y).unwrap();
      map[row_idx][col_idx] = '#';

      if let Some((_, _)) = astar(
        &start,
        |location| get_neighbours(&map, *location),
        |location| utils::u32_to_i32(end.x.abs_diff(location.x) + end.y.abs_diff(location.y)).unwrap(),
        |location| location == &end,
      ) {
        {}
      } else {
        return input[step].to_string();
      }
    };
    panic!("No blocked path found.");
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::read_file;

  #[test]
  fn test_part_1_example() {
    let day = Day18 {};
    assert_eq!(
      PART_1_EXAMPLE,
      day.solve_part1(&read_file("input/day18/test1.txt"))
    );
  }
  
  #[test]
  fn test_part_1() {
    let day = Day18 {};
    assert_eq!(
      "356",
      day.solve_part1(&read_file("input/day18/part1.txt"))
    );
  }

  #[test]
  fn test_part_2_example() {
    let day = Day18 {};
    assert_eq!(
      PART_2_EXAMPLE,
      day.solve_part2(&read_file("input/day18/test1.txt"))
    );
  }
  
  #[test]
  fn test_part_2() {
    let day = Day18 {};
    assert_eq!(
      "22,33",
      day.solve_part2(&read_file("input/day18/part1.txt"))
    );
  }
}