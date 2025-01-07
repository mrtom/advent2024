use crate::AOCDay;
use pathfinding::prelude::astar;
use std::collections::{HashSet, VecDeque};

const PART_1_EXAMPLE: &str = "5";
const PART_2_EXAMPLE: &str = "285";

type Map = Vec<Vec<char>>;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Point {
  x: usize,
  y: usize,
}

fn get_manhattan_distance(a: Point, b: Point) -> usize {
  a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

fn parse_input(input: &[String]) -> (Map, Point, Point) {
  let mut map = input
    .iter()
    .map(|line| line.chars().collect())
    .collect::<Vec<Vec<char>>>();
  let start = map
    .iter()
    .enumerate()
    .find_map(|(y, row)| {
      row
        .iter()
        .enumerate()
        .find_map(|(x, &c)| if c == 'S' { Some((x, y)) } else { None })
    })
    .expect("No start position found");
  let end = map
    .iter()
    .enumerate()
    .find_map(|(y, row)| {
      row
        .iter()
        .enumerate()
        .find_map(|(x, &c)| if c == 'E' { Some((x, y)) } else { None })
    })
    .expect("No end position found");

  map[start.1][start.0] = '.';
  map[end.1][end.0] = '.';

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

fn get_neighbours(map: &Map, location: Point) -> Vec<(Point, usize)> {
  let mut neighbours = vec![];
  if location.x > 0 {
    neighbours.push((
      Point {
        x: location.x - 1,
        y: location.y,
      },
      1,
    ));
  }

  if location.y > 0 {
    neighbours.push((
      Point {
        x: location.x,
        y: location.y - 1,
      },
      1,
    ));
  }

  if location.x < map[0].len() - 1 {
    neighbours.push((
      Point {
        x: location.x + 1,
        y: location.y,
      },
      1,
    ));
  }

  if location.y < map.len() - 1 {
    neighbours.push((
      Point {
        x: location.x,
        y: location.y + 1,
      },
      1,
    ));
  }

  neighbours
    .iter()
    .filter(|(point, _)| map[point.y][point.x] == '.')
    .copied()
    .collect()
}

fn generate_cheat_paths_for_tile(
  map: &Map,
  cheat_start: Point,
  path_length: usize,
) -> HashSet<(Point, Point, usize)> {
  // Take all the tiles in the map
  map
    .iter()
    .enumerate()
    .flat_map(|(row_idx, col)| {
      col.iter().enumerate().map(move |(col_idx, _)| Point {
        x: col_idx,
        y: row_idx,
      })
    })
    // Then filter only the track tiles
    .filter(|cheat_end| map[cheat_end.y][cheat_end.x] == '.')
    // get the manhatten distance from the cheat start to the end
    // and filter the ones that are within the path length
    .filter_map(|cheat_end| {
      let distance = get_manhattan_distance(cheat_start, cheat_end);
      if distance <= path_length {
        Some((cheat_start, cheat_end, distance))
      } else {
        None
      }
    })
    .collect::<HashSet<(Point, Point, usize)>>()
}

fn get_track_tiles(map: &Map) -> Vec<Point> {
  map
    .iter()
    .enumerate()
    .flat_map(|(row_idx, col)| {
      col
        .iter()
        .enumerate()
        .filter(|(_, value)| **value == '.')
        .map(move |(col_idx, _)| Point {
          x: col_idx,
          y: row_idx,
        })
    })
    .collect::<Vec<Point>>()
}

fn count_cheat_paths(
  map: &Map,
  start: Point,
  end: Point,
  path_length: usize,
  required_saving: usize,
) -> usize {
  let total_cost = astar(
    &start,
    |loc| get_neighbours(map, *loc),
    |loc| get_manhattan_distance(end, *loc),
    |loc| *loc == end,
  )
  .expect("Could not find valid path through maze")
  .1;

  let mut from_start = vec![vec![usize::MAX; map[0].len()]; map.len()];

  let mut from_start_queue: VecDeque<(Point, usize)> = VecDeque::new();
  from_start_queue.push_back((start, 0));
  from_start[start.y][start.x] = 0;

  while !from_start_queue.is_empty() {
    let (current, cost) = from_start_queue.pop_front().unwrap();

    for (neighbour, neighbour_cost) in get_neighbours(map, current) {
      let new_cost = cost + neighbour_cost;
      if new_cost < from_start[neighbour.y][neighbour.x] {
        from_start[neighbour.y][neighbour.x] = new_cost;
        from_start_queue.push_back((neighbour, new_cost));
      }
    }
  }

  let mut to_end = vec![vec![usize::MAX; map[0].len()]; map.len()];
  let mut to_end_queue = VecDeque::new();
  to_end_queue.push_back((end, 0));
  to_end[end.y][end.x] = 0;

  while !to_end_queue.is_empty() {
    let (current, cost) = to_end_queue.pop_front().unwrap();

    for (neighbour, neighbour_cost) in get_neighbours(map, current) {
      let new_cost = cost + neighbour_cost;
      if new_cost < to_end[neighbour.y][neighbour.x] {
        to_end[neighbour.y][neighbour.x] = new_cost;
        to_end_queue.push_back((neighbour, new_cost));
      }
    }
  }

  let all_cheat_paths = get_track_tiles(map)
    .iter()
    .flat_map(|point| generate_cheat_paths_for_tile(map, *point, path_length))
    .filter_map(|(cheat_start, cheat_end, cheat_distance)| {
      let start_cost = from_start[cheat_start.y][cheat_start.x];
      let end_cost = to_end[cheat_end.y][cheat_end.x];
      let cost = start_cost + cheat_distance + end_cost;
      let saving = total_cost.saturating_sub(cost);

      match saving {
        _ if saving >= required_saving => Some((cheat_start, cheat_end, saving)),
        _ => None,
      }
    })
    .collect::<HashSet<(Point, Point, usize)>>();

  all_cheat_paths.len()
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
