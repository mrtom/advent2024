use crate::AOCDay;
use crate::utils::a_star;
use crate::utils;

const PART_1_EXAMPLE: &str = "11048";
const PART_2_EXAMPLE: &str = "FAIL";

type Map = Vec<Vec<char>>;
type Location = (i32, i32);

#[allow(dead_code)]
fn print_map(map: &Map) {
  for row in map {
    println!("{}", row.iter().collect::<String>());
  }
  println!("\n----------------\n\n");
}

#[allow(dead_code)]
fn print_map_with_path(map: &Map) {
  for row in map {
    println!("{}", row.iter().map(|c| {
      match c {
        '.' => ' ',
        '$' => '.',
        other => *other,
      }
    }).collect::<String>());
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
    
    match a_star::with_direction_cost(
      &map,
      start_position,
      end_position,
      1,
      1000,
    ) {
      Some((path, cost)) => {
        // If the first move isn't in the East direction, we need to add a direction change cost
        let addition_cost = if path[1].0 <= path[0].0 { 1000 } else { 0 };
        let final_cost = cost + addition_cost;
        final_cost.to_string()
      }
      None => {
        panic!("No path found.");
      }
    }
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
      "101496",
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
      "TODO",
      day.solve_part2(&read_file("input/day16/part1.txt"))
    );
  }
}