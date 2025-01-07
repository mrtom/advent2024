use crate::utils::Point;
use crate::{utils, AOCDay};
use pathfinding::prelude::astar;
use std::collections::{HashSet, VecDeque};

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

fn get_neighbours(map: &Map, location: Point) -> Vec<(Point, usize)> {
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
) -> HashSet<(Point, Point, usize)> {
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
      let distance = utils::i32_to_usize_x(utils::get_manhattan_distance(cheat_start, cheat_end));
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
          x: utils::usize_to_i32_x(col_idx),
          y: utils::usize_to_i32_x(row_idx),
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
    |loc| utils::i32_to_usize_x(utils::get_manhattan_distance(end, *loc)),
    |loc| *loc == end,
  )
  .expect("Could not find valid path through maze")
  .1;

  let mut from_start = vec![vec![usize::MAX; map[0].len()]; map.len()];

  let mut from_start_queue: VecDeque<(Point, usize)> = VecDeque::new();
  from_start_queue.push_back((start, 0));
  from_start[utils::i32_to_usize_x(start.y)][utils::i32_to_usize_x(start.x)] = 0;

  while !from_start_queue.is_empty() {
    let (current, cost) = from_start_queue.pop_front().unwrap();

    for (neighbour, neighbour_cost) in get_neighbours(map, current) {
      let new_cost = cost + neighbour_cost;
      if new_cost
        < from_start[utils::i32_to_usize_x(neighbour.y)][utils::i32_to_usize_x(neighbour.x)]
      {
        from_start[utils::i32_to_usize_x(neighbour.y)][utils::i32_to_usize_x(neighbour.x)] =
          new_cost;
        from_start_queue.push_back((neighbour, new_cost));
      }
    }
  }

  let mut to_end = vec![vec![usize::MAX; map[0].len()]; map.len()];
  let mut to_end_queue = VecDeque::new();
  to_end_queue.push_back((end, 0));
  to_end[utils::i32_to_usize_x(end.y)][utils::i32_to_usize_x(end.x)] = 0;

  while !to_end_queue.is_empty() {
    let (current, cost) = to_end_queue.pop_front().unwrap();

    for (neighbour, neighbour_cost) in get_neighbours(map, current) {
      let new_cost = cost + neighbour_cost;
      if new_cost < to_end[utils::i32_to_usize_x(neighbour.y)][utils::i32_to_usize_x(neighbour.x)] {
        to_end[utils::i32_to_usize_x(neighbour.y)][utils::i32_to_usize_x(neighbour.x)] = new_cost;
        to_end_queue.push_back((neighbour, new_cost));
      }
    }
  }

  let all_cheat_paths = get_track_tiles(map)
    .iter()
    .flat_map(|point| generate_cheat_paths_for_tile(map, *point, path_length))
    .filter_map(|(cheat_start, cheat_end, cheat_distance)| {
      let start_cost =
        from_start[utils::i32_to_usize_x(cheat_start.y)][utils::i32_to_usize_x(cheat_start.x)];
      let end_cost = to_end[utils::i32_to_usize_x(cheat_end.y)][utils::i32_to_usize_x(cheat_end.x)];
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
