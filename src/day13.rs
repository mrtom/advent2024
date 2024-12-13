use itertools::Itertools;
use regex::Regex;

use crate::AOCDay;

const PART_1_EXAMPLE: &str = "480";
const PART_2_EXAMPLE: &str = "875318608908";

struct Button {
  x_const: isize,
  y_const: isize,
}

struct Prize {
  x: isize,
  y: isize,
}

struct Machine {
  button_a: Button,
  button_b: Button,
  prize: Prize,
}

fn build_machine(input: &str, is_part_two: bool) -> Machine {
  let machine_input = input.split('_').collect::<Vec<&str>>();
  let (
    a_input, 
    b_input, 
    prize_input
  ) = (machine_input[0], machine_input[1], machine_input[2]);

  let button_regex = Regex::new(r"Button (A|B): X\+(\d+), Y\+(\d+)").unwrap();
  let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

  let capture_a = button_regex.captures(a_input);
  let capture_b = button_regex.captures(b_input);
  let capture_prize = prize_regex.captures(prize_input);

  let extra: isize = match is_part_two {
    true => 10_000_000_000_000,
    false => 0,
  };

  match (capture_a, capture_b, capture_prize) {
    (Some(cap_a), Some(cap_b), Some(cap_prize)) => {
      let a_x = cap_a.get(2).map(|m| m.as_str().parse::<isize>()).unwrap().ok().unwrap();
      let a_y = cap_a.get(3).map(|m| m.as_str().parse::<isize>()).unwrap().ok().unwrap();
      let b_x = cap_b.get(2).map(|m| m.as_str().parse::<isize>()).unwrap().ok().unwrap();
      let b_y = cap_b.get(3).map(|m| m.as_str().parse::<isize>()).unwrap().ok().unwrap();
      let prize_x = cap_prize.get(1).map(|m| m.as_str().parse::<isize>()).unwrap().ok().unwrap();
      let prize_y = cap_prize.get(2).map(|m| m.as_str().parse::<isize>()).unwrap().ok().unwrap();

      Machine {
        button_a: Button {
          x_const: a_x,
          y_const: a_y,
        },
        button_b: Button {
          x_const: b_x,
          y_const: b_y,
        },
        prize: Prize {
          x: prize_x + extra,
          y: prize_y + extra,
        }
      }
    },
    (_, _, _) => {
      panic!("Could not parse machine input");
    }
  }
}

#[allow(clippy::similar_names)]
fn solve_for(machine: &Machine) -> isize {
  let button_a = &machine.button_a;
  let button_b = &machine.button_b;
  let prize = &machine.prize;
  let determinent = button_a.x_const * button_b.y_const - button_a.y_const * button_b.x_const;

  if determinent == 0 {
    return 0
  }

  let button_a_numerator = prize.x * button_b.y_const - prize.y * button_b.x_const;
  let button_b_numerator = button_a.x_const * prize.y - button_a.y_const * prize.x;
  if button_a_numerator % determinent != 0 ||  button_b_numerator % determinent != 0 {
    return 0
  }

  let button_a_pushes = button_a_numerator / determinent;
  let button_b_pushes = button_b_numerator / determinent;

  button_a_pushes * 3 + button_b_pushes
}

fn parse_input(input: &[String], is_part_two: bool) -> Vec<Machine> {
  let combined = input.iter().join("_");

  combined.split("__").map(|machine_input| {
    build_machine(machine_input, is_part_two)
  }).collect::<Vec<Machine>>()
}

pub struct Day13 {}

impl AOCDay for Day13 {
  fn name(&self) -> String {
    "day13".to_string()
  }
  
  fn test_answer_part1(&self) -> String {
    PART_1_EXAMPLE.to_string()
  }
  
  fn test_answer_part2(&self) -> String {
    PART_2_EXAMPLE.to_string()
  }
  
  fn solve_part1(&self, input: &[String]) -> String {
    let machines = parse_input(input, false);
    machines.iter().map(|machine| {
      solve_for(machine)
    }).sum::<isize>().to_string()
  }
  
  fn solve_part2(&self, input: &[String]) -> String {
    let machines = parse_input(input, true);

    let answer = machines.iter().map(|machine| {
      solve_for(machine)
    }).sum::<isize>();

    answer.to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::read_file;

  #[test]
  fn test_part_1_example() {
    let day = Day13 {};
    assert_eq!(
      PART_1_EXAMPLE,
      day.solve_part1(&read_file("input/day13/test1.txt"))
    );
  }
  
  #[test]
  fn test_part_1() {
    let day = Day13 {};
    assert_eq!(
      "33481",
      day.solve_part1(&read_file("input/day13/part1.txt"))
    );
  }

  #[test]
  fn test_part_2_example() {
    let day = Day13 {};
    assert_eq!(
      PART_2_EXAMPLE,
      day.solve_part2(&read_file("input/day13/test1.txt"))
    );
  }
  
  #[test]
  fn test_part_2() {
    let day = Day13 {};
    assert_eq!(
      "92572057880885",
      day.solve_part2(&read_file("input/day13/part1.txt"))
    );
  }
}