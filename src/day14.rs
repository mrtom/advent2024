use core::panic;
use regex::Regex;
use std::collections::HashMap;

use crate::AOCDay;

const PART_1_EXAMPLE: &str = "12";
const PART_2_EXAMPLE: &str = "SKIP";

struct Robot {
  px: isize,
  py: isize,
  vx: isize,
  vy: isize,
}

fn parse_input(input: &[String]) -> Vec<Robot> {
  let robot_regex = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

  input
    .iter()
    .map(|line| {
      let capture = robot_regex.captures(line).expect("Could not parse robot");
      let px = capture
        .get(1)
        .unwrap()
        .as_str()
        .parse::<isize>()
        .expect("Could not parse px");
      let py = capture
        .get(2)
        .unwrap()
        .as_str()
        .parse::<isize>()
        .expect("Could not parse py");
      let vx = capture
        .get(3)
        .unwrap()
        .as_str()
        .parse::<isize>()
        .expect("Could not parse vx");
      let vy = capture
        .get(4)
        .unwrap()
        .as_str()
        .parse::<isize>()
        .expect("Could not parse vy");

      Robot { px, py, vx, vy }
    })
    .collect()
}

#[allow(dead_code)]
fn print_locations(locations: &[(usize, usize)]) {
  let (width, height): (usize, usize) = match locations.len() {
    12 => (11, 7),
    _ => (101, 103),
  };

  let mut map = vec![vec![0; width]; height];

  for location in locations {
    map[location.1][location.0] += 1;
  }

  for row in &map {
    for value in row {
      match value {
        0 => print!("."),
        value => print!("{value}"),
      }
    }
    println!();
  }
  println!("\n---------------------\n\n");
}

fn is_tightly_packed(location: (usize, usize), locations: &HashMap<(usize, usize), isize>) -> bool {
  let mut neighbours = 0;
  if location.0 > 0 && locations.contains_key(&(location.0 - 1, location.1)) {
    neighbours += 1;
  };
  if locations.contains_key(&(location.0 + 1, location.1)) {
    neighbours += 1;
  };
  if location.1 > 0 && locations.contains_key(&(location.0, location.1 - 1)) {
    neighbours += 1;
  };
  if locations.contains_key(&(location.0, location.1 + 1)) {
    neighbours += 1;
  };

  let count = locations.get(&location).unwrap();

  neighbours >= 2 || *count > 1
}

pub struct Day14 {}

impl AOCDay for Day14 {
  fn name(&self) -> String {
    "day14".to_string()
  }

  fn test_answer_part1(&self) -> String {
    PART_1_EXAMPLE.to_string()
  }

  fn test_answer_part2(&self) -> String {
    PART_2_EXAMPLE.to_string()
  }

  fn solve_part1(&self, input: &[String]) -> String {
    let robots = parse_input(input);
    let (width, height) = match robots.len() {
      12 => (11, 7),
      _ => (101, 103),
    };
    let mut results = [0; 4];

    let locations = robots
      .iter()
      .map(|robot| {
        let x = (robot.px + 100 * robot.vx) % width;
        let y = (robot.py + 100 * robot.vy) % height;

        let x_result = if x >= 0 { x } else { width + x };
        let y_result = if y >= 0 { y } else { height + y };

        (x_result, y_result)
      })
      .collect::<Vec<(isize, isize)>>();

    for (x, y) in &locations {
      match (x, y) {
        (x, y) if *x < width / 2 && *y < height / 2 => results[0] += 1,
        (x, y) if *x < width / 2 && *y > height / 2 => results[1] += 1,
        (x, y) if *x > width / 2 && *y < height / 2 => results[2] += 1,
        (x, y) if *x > width / 2 && *y > height / 2 => results[3] += 1,
        _ => {}
      }
    }

    results.iter().product::<i32>().to_string()
  }

  #[allow(clippy::cast_sign_loss)]
  fn solve_part2(&self, input: &[String]) -> String {
    let robots = parse_input(input);

    if robots.len() == 12 {
      // There is no example for part 2
      return "SKIP".to_string();
    }

    let width = 101;
    let height = 103;

    for i in 1..isize::MAX {
      let locations = robots
        .iter()
        .map(|robot| {
          let x = (robot.px + i * robot.vx) % width;
          let y = (robot.py + i * robot.vy) % height;

          let x_result = if x >= 0 { x } else { width + x };
          let y_result = if y >= 0 { y } else { height + y };

          (x_result as usize, y_result as usize)
        })
        .collect::<Vec<(usize, usize)>>();

      let mut locations_map = HashMap::new();
      for location in &locations {
        *locations_map.entry((location.0, location.1)).or_insert(0) += 1;
      }

      let num_tightly_packed = locations
        .iter()
        .filter(|location| is_tightly_packed(**location, &locations_map))
        .count();

      if num_tightly_packed > 300 {
        // println!("Possible solution found: {i}");
        // println!("Tightly packed: {num_tightly_packed}");
        // print_locations(&locations);
        // println!("---------------------\n\n");
        return i.to_string();
      }
    }

    panic!("Could not find solution");
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::read_file;

  #[test]
  fn test_part_1_example() {
    let day = Day14 {};
    assert_eq!(
      PART_1_EXAMPLE,
      day.solve_part1(&read_file("input/day14/test1.txt"))
    );
  }

  #[test]
  fn test_part_1() {
    let day = Day14 {};
    assert_eq!(
      "209409792",
      day.solve_part1(&read_file("input/day14/part1.txt"))
    );
  }

  #[test]
  fn test_part_2_example() {
    let day = Day14 {};
    assert_eq!(
      PART_2_EXAMPLE,
      day.solve_part2(&read_file("input/day14/test1.txt"))
    );
  }

  #[test]
  fn test_part_2() {
    let day = Day14 {};
    assert_eq!("8006", day.solve_part2(&read_file("input/day14/part1.txt")));
  }
}
