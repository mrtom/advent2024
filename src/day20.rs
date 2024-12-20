use std::collections::HashSet;
use pathfinding::prelude::astar;
use crate::{utils, AOCDay};

const PART_1_EXAMPLE: &str = "5";
const PART_2_EXAMPLE: &str = "FAIL";

type Map = Vec<Vec<char>>;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Point {
  x: i32,
  y: i32,
}

fn parse_input(input: &[String]) -> (Map, Point, Point) {
  let mut map = input.iter().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();
  let start = map.iter().enumerate().find_map(|(y, row)| {
    row.iter().enumerate().find_map(|(x, &c)| {
      if c == 'S' {
        Some((utils::usize_to_i32(x).unwrap(), utils::usize_to_i32(y).unwrap()))
      } else {
        None
      }
    })
  }).expect("No start position found");
  let end = map.iter().enumerate().find_map(|(y, row)| {
    row.iter().enumerate().find_map(|(x, &c)| {
      if c == 'E' {
        Some((utils::usize_to_i32(x).unwrap(), utils::usize_to_i32(y).unwrap()))
      } else {
        None
      }
    })
  }).expect("No end position found");

  map[utils::i32_to_usize_x(start.1)][utils::i32_to_usize_x(start.0)] = '.';
  map[utils::i32_to_usize_x(end.1)][utils::i32_to_usize_x(end.0)] = '.';

  (map, Point { x: start.0, y: start.1 }, Point { x: end.0, y: end.1 })
}

#[allow(dead_code)]
fn print_map(map: &Map) {
  for row in map {
    println!("{}", row.iter().collect::<String>());
  }
  println!("\n----------------\n\n");
}

fn is_test(input: &[Vec<char>]) -> bool {
  input.len() == 15
}

fn get_path_neighbours(map: &Map, location: Point) -> Vec<(Point, i32)> {
  get_neighbours(map, location, '.')
}

fn get_neighbours(map: &Map, location: Point, of_type: char) -> Vec<(Point, i32)> {
  let mut neighbours = vec![];
  
  for (dx, dy) in &[(0, -1), (0, 1), (-1, 0), (1, 0)] {
    let new_pos = (location.x + dx, location.y + dy);
    let col_max = utils::usize_to_i32_x(map[0].len());
    let row_max = utils::usize_to_i32_x(map.len());
    
    if new_pos.1 >= 0 && new_pos.1 < row_max && 
        new_pos.0 >= 0 && new_pos.0 < col_max && 
        map[utils::i32_to_usize_x(new_pos.1)][utils::i32_to_usize_x(new_pos.0)] == of_type 
    {
      neighbours.push((Point {
        x: new_pos.0,
        y: new_pos.1,
      }, 1));
    }
  }
  
  neighbours
}

fn get_wall_pairs(map: &Map) -> HashSet<(Point, Point)> {
  let wall_tiles = map.iter().enumerate().flat_map(|(row_idx, col)| {
    col.iter().enumerate().filter(|(_, value)| **value == '#').map(move |(col_idx, _)| {
      Point { x: utils::usize_to_i32_x(col_idx), y: utils::usize_to_i32_x(row_idx) }
    })
  }).collect::<Vec<Point>>();

  let mut result = HashSet::new();

  for tile in wall_tiles {
    for neighbour in get_path_neighbours(map, tile) {
      result.insert((
        Point { x: tile.x, y: tile.y },
        Point { x: neighbour.0.x, y: neighbour.0.y },
      ));
    }
  }

  result
}

pub struct Day20 {}

impl AOCDay for Day20 {
  fn name(&self) -> String {
    "day20".to_string()
  }
  
  fn test_answer_part1(&self) -> String {
    PART_1_EXAMPLE.to_string()
  }
  
  fn test_answer_part2(&self) -> String {
    PART_2_EXAMPLE.to_string()
  }
  
  fn solve_part1(&self, input: &[String]) -> String {
    let (map, start, end) = parse_input(input);
    let is_test_run = is_test(&map);
    let required_saving = if is_test_run { 20 } else { 100 };
    let mut big_cheat_count = 0;

    let total_cost =  astar(
      &start,
      |loc| get_path_neighbours(&map, *loc),
      |loc| utils::u32_to_i32(end.x.abs_diff(loc.x) + end.y.abs_diff(loc.y)).unwrap(),
      |loc| *loc == end,
    ).expect("Could not find valid path through maze").1;

    let cheat_pairs = get_wall_pairs(&map);
    for cheat_pair in cheat_pairs {
      let mut new_map = map.clone();
      let first = cheat_pair.0;
      new_map[utils::i32_to_usize_x(first.y)][utils::i32_to_usize_x(first.x)] = '.';

      if let Some((_, new_cost)) = astar(
        &start,
        |loc| get_path_neighbours(&new_map, *loc),
        |loc| utils::u32_to_i32(end.x.abs_diff(loc.x) + end.y.abs_diff(loc.y)).unwrap(),
        |loc| *loc == end,
      ) {
        if total_cost - new_cost >= required_saving {
          big_cheat_count += 1;
        }
      }
    }

    (big_cheat_count / 2).to_string()
  }
  
  fn solve_part2(&self, input: &[String]) -> String {
    let input = parse_input(input);
    "Not implemented".to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::read_file;

  #[test]
  fn test_part_1_example() {
    let day = Day20 {};
    assert_eq!(
      PART_1_EXAMPLE,
      day.solve_part1(&read_file("input/day20/test1.txt"))
    );
  }
  
  #[test]
  fn test_part_1() {
    let day = Day20 {};
    assert_eq!(
      "1321",
      day.solve_part1(&read_file("input/day20/part1.txt"))
    );
  }

  #[test]
  fn test_part_2_example() {
    let day = Day20 {};
    assert_eq!(
      PART_2_EXAMPLE,
      day.solve_part2(&read_file("input/day20/test1.txt"))
    );
  }
  
  #[test]
  fn test_part_2() {
    let day = Day20 {};
    assert_eq!(
      "TODO",
      day.solve_part2(&read_file("input/day20/part1.txt"))
    );
  }
}