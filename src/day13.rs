use itertools::Itertools;
use regex::Regex;

use crate::AOCDay;

const PART_1_EXAMPLE: &str = "480";
const PART_2_EXAMPLE: &str = "FAIL";

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

fn build_machine(input: &str) -> Machine {
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
          x: prize_x,
          y: prize_y,
        }
      }
    },
    (_, _, _) => {
      panic!("Could not parse machien input");
    }
  }
}

// fn make_move(machine: &Machine, remaining: (isize, isize), button_a_count: isize, button_b_count: isize, valid_turns: &mut HashSet<(isize, isize)>) {
//   if remaining.0 == 0 && remaining.1 == 0 {
//     valid_turns.insert((button_a_count, button_b_count));
//   }

//   if button_a_count + button_b_count > 100 {
//     return
//   }

//   // Push A Button
//   if remaining.0 > machine.button_A.x_const && remaining.1 > machine.button_A.y_const {
//     make_move(
//       machine,
//       (remaining.0 - machine.button_A.x_const, remaining.1 - machine.button_A.y_const),
//       button_a_count + 1,
//       button_b_count,
//       valid_turns,
//     );
//   }

//   // Push B Button
//   if remaining.0 > machine.button_B.x_const && remaining.1 > machine.button_B.y_const {
//     make_move(
//       machine,
//       (remaining.0 - machine.button_B.x_const, remaining.1 - machine.button_B.y_const),
//       button_a_count,
//       button_b_count + 1,
//       valid_turns,
//     );
//   }
// }


// fn valid_turns(machine: &Machine) -> HashSet<(isize, isize)> {
//   let mut results = HashSet::new();
//   let remaining = (machine.prize.x, machine.prize.y);

//   make_move(machine, remaining, 0, 0, &mut results);

//   results
// }

fn solve_for(machine: &Machine) -> isize {
  let button_a = &machine.button_a;
  let button_b = &machine.button_b;
  let prize = &machine.prize;
  let determinent = button_a.x_const * button_b.y_const - button_a.y_const * button_b.x_const;

  if determinent == 0 {
    return 0
  }

  if (prize.x * button_b.y_const - prize.y * button_b.x_const) % determinent != 0 {
    return 0
  }

  if (button_a.x_const * prize.y - button_a.y_const * prize.x) % determinent != 0 {
    return 0
  }

  let button_a_pushes = (prize.x * button_b.y_const - prize.y * button_b.x_const) / determinent;
  let button_b_pushes = (button_a.x_const * prize.y - button_a.y_const * prize.x) / determinent;
  
  // a1x + b1y = c1
  // a1: button_A.x_const
  // b1: button_B.x_const
  // c1: prize.x
  // x: button_a_pushes
  // y: button_b_pushes

  // a2x + b2y = c2
  // a2: button_A.y_const
  // b2: button_B.y_const
  // c2: prize.y

  button_a_pushes * 3 + button_b_pushes
}

fn parse_input(input: &[String]) -> Vec<Machine> {
  let combined = input.iter().join("_");

  combined.split("__").map(|machine_input| {
    build_machine(machine_input)
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
    let machines = parse_input(input);
    machines.iter().map(|machine| {
      // valid_turns(machine).iter().map(|(a, b)| 3 * a + b).min().unwrap()
      solve_for(machine)
    }).sum::<isize>().to_string()
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
      "TODO",
      day.solve_part2(&read_file("input/day13/part1.txt"))
    );
  }
}