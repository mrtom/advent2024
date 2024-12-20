use crate::utils::Point;
use crate::{utils, AOCDay};
use pathfinding::prelude::astar;
use std::collections::HashSet;

const PART_1_EXAMPLE: &str = "5";
const PART_2_EXAMPLE: &str = "285";

type Map = Vec<Vec<char>>;

fn parse_input(input: &[String]) -> (Map, Point, Point) {
  let mut map = input
    .iter()
    .map(|line| line.chars().collect())
    .collect::<Vec<Vec<char>>>();
  let start = map
    .iter()
    .enumerate()
    .find_map(|(y, row)| {
      row.iter().enumerate().find_map(|(x, &c)| {
        if c == 'S' {
          Some((
            utils::usize_to_i32(x).unwrap(),
            utils::usize_to_i32(y).unwrap(),
          ))
        } else {
          None
        }
      })
    })
    .expect("No start position found");
  let end = map
    .iter()
    .enumerate()
    .find_map(|(y, row)| {
      row.iter().enumerate().find_map(|(x, &c)| {
        if c == 'E' {
          Some((
            utils::usize_to_i32(x).unwrap(),
            utils::usize_to_i32(y).unwrap(),
          ))
        } else {
          None
        }
      })
    })
    .expect("No end position found");

  map[utils::i32_to_usize_x(start.1)][utils::i32_to_usize_x(start.0)] = '.';
  map[utils::i32_to_usize_x(end.1)][utils::i32_to_usize_x(end.0)] = '.';

  (
    map,
    Point {
      x: start.0,
      y: start.1,
    },
    Point { x: end.0, y: end.1 },
  )
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

fn get_neighbours(
  map: &Map,
  location: Point,
  cheat_start: Option<Point>,
  cheat_end: Option<Point>,
  cheat_distance: Option<i32>,
) -> Vec<(Point, i32)> {
  // If we've ended up on a cheat start, the only neighbour is the end of the cheat_path
  if let (Some(cheat_start), Some(cheat_end), Some(cheat_distance)) =
    (cheat_start, cheat_end, cheat_distance)
  {
    if location == cheat_start {
      return vec![(cheat_end, cheat_distance)];
    }
  }

  // Otherwise, we are on the track and move as normal
  let mut neighbours = vec![];
  for (dx, dy) in &[(0, -1), (0, 1), (-1, 0), (1, 0)] {
    let new_pos = (location.x + dx, location.y + dy);
    let col_max = utils::usize_to_i32_x(map[0].len());
    let row_max = utils::usize_to_i32_x(map.len());

    if new_pos.1 >= 0
      && new_pos.1 < row_max
      && new_pos.0 >= 0
      && new_pos.0 < col_max
      && map[utils::i32_to_usize_x(new_pos.1)][utils::i32_to_usize_x(new_pos.0)] == '.'
    {
      neighbours.push((
        Point {
          x: new_pos.0,
          y: new_pos.1,
        },
        1,
      ));
    }
  }

  neighbours
}

fn generate_cheat_paths_for_tile(
  map: &Map,
  cheat_start: Point,
  path_length: usize,
) -> HashSet<(Point, Point, i32)> {
  // Take all the tiles in the map
  map
    .iter()
    .enumerate()
    .flat_map(|(row_idx, col)| {
      col.iter().enumerate().map(move |(col_idx, _)| Point {
        x: utils::usize_to_i32_x(col_idx),
        y: utils::usize_to_i32_x(row_idx),
      })
    })
    // Then filter only the track tiles
    .filter(|cheat_end| {
      map[utils::i32_to_usize_x(cheat_end.y)][utils::i32_to_usize_x(cheat_end.x)] == '.'
    })
    // get the manhatten distance from the cheat start to the end
    // and filter the ones that are within the path length
    .filter_map(|cheat_end| {
      let distance = utils::get_manhattan_distance(cheat_start, cheat_end);
      if distance <= utils::usize_to_i32_x(path_length) {
        Some((cheat_start, cheat_end, distance))
      } else {
        None
      }
    })
    .collect::<HashSet<(Point, Point, i32)>>()
}

fn get_cheat_paths(map: &Map, path_length: usize) -> HashSet<(Point, Point, i32)> {
  let track_tiles = map
    .iter()
    .enumerate()
    .flat_map(|(row_idx, col)| {
      col
        .iter()
        .enumerate()
        .filter(|(_, value)| **value == '.')
        .map(move |(col_idx, _)| Point {
          x: utils::usize_to_i32_x(col_idx),
          y: utils::usize_to_i32_x(row_idx),
        })
    })
    .collect::<Vec<Point>>();

  let mut result = HashSet::new();
  for tile in track_tiles {
    let paths = generate_cheat_paths_for_tile(map, tile, path_length);
    for (cheat_start, cheat_end, cheat_path_length) in paths {
      result.insert((cheat_start, cheat_end, cheat_path_length));
    }
  }

  result
}

fn find_shortest_path(
  map: &Map,
  start: Point,
  end: Point,
  cheat_start: Option<Point>,
  cheat_end: Option<Point>,
  cheat_distance: Option<i32>,
) -> Option<(Vec<Point>, i32)> {
  astar(
    &start,
    |loc| get_neighbours(map, *loc, cheat_start, cheat_end, cheat_distance),
    |loc| utils::get_manhattan_distance(end, *loc),
    |loc| *loc == end,
  )
}

fn count_cheat_paths(
  map: &Map,
  start: Point,
  end: Point,
  path_length: usize,
  required_saving: i32,
) -> usize {
  let total_cost = find_shortest_path(map, start, end, None, None, None)
    .expect("Could not find valid path through maze")
    .1;

  get_cheat_paths(map, path_length)
    .iter()
    .filter_map(|(cheat_start, cheat_end, cheat_distance)| {
      let result = find_shortest_path(
        map,
        start,
        end,
        Some(*cheat_start),
        Some(*cheat_end),
        Some(*cheat_distance),
      );

      result.map(|(_, cost)| (*cheat_start, *cheat_end, cost, total_cost - cost))
    })
    .filter(|(_, _, cost, _)| *cost <= total_cost - required_saving)
    .count()
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
    let required_saving = if is_test(&map) { 20 } else { 100 };

    let result = count_cheat_paths(&map, start, end, 2, required_saving);

    result.to_string()
  }

  fn solve_part2(&self, input: &[String]) -> String {
    let (map, start, end) = parse_input(input);
    let required_saving = if is_test(&map) { 50 } else { 100 };

    let result = count_cheat_paths(&map, start, end, 20, required_saving);

    result.to_string()
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
    assert_eq!("1321", day.solve_part1(&read_file("input/day20/part1.txt")));
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
      "971737",
      day.solve_part2(&read_file("input/day20/part1.txt"))
    );
  }
}
