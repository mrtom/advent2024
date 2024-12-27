use crate::AOCDay;
use itertools::Itertools;
use std::cmp::Ordering;
use std::string::String;

const PART_1_EXAMPLE: &str = "126384";
const PART_2_EXAMPLE: &str = "FAIL";

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Position {
  x: usize,
  y: usize,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Command {
  direction: char,
  steps: usize,
}

fn parse_input(input: &[String]) -> Vec<Vec<char>> {
  input
    .iter()
    .map(|line| line.parse::<String>().unwrap().chars().collect())
    .collect()
}

// MARK - Numeric helpers

// Numeric keypad:
// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
fn pos_for_numeric_keypad_char(c: char) -> Position {
  match c {
    '0' => Position { x: 1, y: 0 },
    'A' => Position { x: 2, y: 0 },
    '1' => Position { x: 0, y: 1 },
    '2' => Position { x: 1, y: 1 },
    '3' => Position { x: 2, y: 1 },
    '4' => Position { x: 0, y: 2 },
    '5' => Position { x: 1, y: 2 },
    '6' => Position { x: 2, y: 2 },
    '7' => Position { x: 0, y: 3 },
    '8' => Position { x: 1, y: 3 },
    '9' => Position { x: 2, y: 3 },
    _ => panic!("Invalid numeric keypad char: {c}"),
  }
}

fn move_numeric(from: Position, to: Position) -> Vec<Vec<Command>> {
  let mut x_commands = Vec::new();
  let mut y_commands = Vec::new();
  let mut commands = Vec::new();

  match from.x.cmp(&to.x) {
    Ordering::Less => x_commands.push(Command {
      direction: '>',
      steps: to.x - from.x,
    }),
    Ordering::Greater => x_commands.push(Command {
      direction: '<',
      steps: from.x - to.x,
    }),
    Ordering::Equal => (),
  }

  match from.y.cmp(&to.y) {
    Ordering::Less => y_commands.push(Command {
      direction: '^',
      steps: to.y - from.y,
    }),
    Ordering::Greater => y_commands.push(Command {
      direction: 'v',
      steps: from.y - to.y,
    }),
    Ordering::Equal => (),
  }

  if x_commands.is_empty() {
    commands.push(y_commands);
  } else if y_commands.is_empty() {
    commands.push(x_commands);
  } else {
    #[allow(clippy::match_same_arms)]
    match ((from.x), (to.x), (from.y), (to.y)) {
      (0, _, 0, _) | (_, 0, _, 0) => {
        panic!("Cannot start or end from 0,0")
      }
      // x starts on 0 and y ends on 0.
      // Must go x then y
      (0, _, _, 0) => {
        let mut first = x_commands;
        first.append(&mut y_commands);
        commands.push(first);
      }
      // y starts on 0 and x ends on 0.
      // Must go y then x
      (_, 0, 0, _) => {
        let mut first = y_commands;
        first.append(&mut x_commands);
        commands.push(first);
      }
      // Otherwise, try both
      (_, _, _, _) => {
        let mut first = x_commands.clone();
        first.append(&mut y_commands.clone());
        commands.push(first);

        let mut second = y_commands;
        second.append(&mut x_commands);
        commands.push(second);
      }
    };
  }

  for command_set in &mut commands {
    command_set.push(Command {
      direction: 'A',
      steps: 1,
    });
  }

  commands
}

// MARK - Directional Helpers

// Directional Keypad:
//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
fn pos_for_directional_keypad_char(c: char) -> Position {
  match c {
    '<' => Position { x: 0, y: 0 },
    'v' => Position { x: 1, y: 0 },
    '>' => Position { x: 2, y: 0 },
    '^' => Position { x: 1, y: 1 },
    'A' => Position { x: 2, y: 1 },
    _ => panic!("Invalid directional keypad char: {c}"),
  }
}

fn move_directional(from: Position, to: Position) -> Vec<Vec<Command>> {
  let mut x_commands = Vec::new();
  let mut y_commands = Vec::new();
  let mut commands = Vec::new();

  match from.x.cmp(&to.x) {
    Ordering::Less => x_commands.push(Command {
      direction: '>',
      steps: to.x - from.x,
    }),
    Ordering::Greater => x_commands.push(Command {
      direction: '<',
      steps: from.x - to.x,
    }),
    Ordering::Equal => (),
  }

  match from.y.cmp(&to.y) {
    Ordering::Less => y_commands.push(Command {
      direction: '^',
      steps: to.y - from.y,
    }),
    Ordering::Greater => y_commands.push(Command {
      direction: 'v',
      steps: from.y - to.y,
    }),
    Ordering::Equal => (),
  }

  if x_commands.is_empty() {
    commands.push(y_commands);
  } else if y_commands.is_empty() {
    commands.push(x_commands);
  } else {
    #[allow(clippy::match_same_arms)]
    match ((from.x), (to.x), (from.y), (to.y)) {
      (0, _, 1, _) | (_, 0, _, 1) => {
        panic!("Cannot start or end from 0,1")
      }
      // x starts on 0 and y starts on 0.
      // Must go x then y
      (0, _, 0, _) => {
        let mut first = x_commands;
        first.append(&mut y_commands);
        commands.push(first);
      }
      // y starts on 1 and x ends on 0.
      // Must go y then x
      (_, 0, _, _) => {
        let mut first = y_commands;
        first.append(&mut x_commands);
        commands.push(first);
      }
      // Otherwise, try both
      (_, _, _, _) => {
        let mut first = x_commands.clone();
        first.append(&mut y_commands.clone());
        commands.push(first);

        let mut second = y_commands;
        second.append(&mut x_commands);
        commands.push(second);
      }
    };
  }

  for command_set in &mut commands {
    command_set.push(Command {
      direction: 'A',
      steps: 1,
    });
  }

  commands
}

fn generate_directional_command_segment(
  current_pos: Position,
  next_pos: Position,
) -> Vec<Vec<char>> {
  let second_robot_commands = move_directional(current_pos, next_pos);
  let second_paths = second_robot_commands
    .iter()
    .map(|second_robot_commands| path_from_commands(second_robot_commands))
    .collect();
  second_paths
}

fn generate_directional_command(current_pos: Position, sequence: &[char]) -> Vec<Vec<char>> {
  let mut partial: Vec<Vec<char>> = vec![];
  let mut current_pos = current_pos;

  for c in sequence {
    let next_pos = pos_for_directional_keypad_char(*c);
    let paths = generate_directional_command_segment(current_pos, next_pos);
    let mut new_partial = Vec::new();
    for path in paths {
      if partial.is_empty() {
        new_partial.push(path);
      } else {
        for p in &partial {
          let mut new_path = p.clone();
          new_path.extend(path.clone());
          new_partial.push(new_path);
        }
      }
    }

    current_pos = next_pos;
    partial = new_partial;
  }

  partial
}

// MARK - Primary functions

fn generate_final_command_segment(current_pos: Position, next_pos: Position) -> String {
  let first_robot_commands = move_numeric(current_pos, next_pos);
  let first_paths = first_robot_commands
    .iter()
    .map(|first_robot_command| path_from_commands(first_robot_command))
    .collect::<Vec<Vec<char>>>();

  let second_robot_paths = first_paths
    .iter()
    .flat_map(|path| generate_directional_command(pos_for_directional_keypad_char('A'), path))
    .collect::<Vec<Vec<char>>>();

  let mut third_robot_commands = second_robot_paths
    .iter()
    .flat_map(|path| generate_directional_command(pos_for_directional_keypad_char('A'), path))
    .map(|path| path.into_iter().join(""))
    .collect::<Vec<String>>();

  third_robot_commands.sort_by_key(String::len);
  third_robot_commands.first().unwrap().to_string()
}

fn generate_shortest_path(sequence: &[char]) -> String {
  let mut partial = Vec::new();
  let mut current_pos = pos_for_numeric_keypad_char('A');

  for c in sequence {
    let next_pos = pos_for_numeric_keypad_char(*c);
    partial.push(generate_final_command_segment(current_pos, next_pos));
    current_pos = next_pos;
  }

  partial.join("")
}

fn path_from_commands(commands: &[Command]) -> Vec<char> {
  commands
    .iter()
    .flat_map(|c| {
      std::iter::repeat(c.direction)
        .take(c.steps)
        .collect::<Vec<char>>()
    })
    .collect::<Vec<char>>()
}

fn calculate_complexity(path: &str, code: &[char]) -> usize {
  let len = path.len();
  let code = code
    .iter()
    .skip_while(|c| **c == '0')
    .filter(|c| c.is_numeric())
    .collect::<String>()
    .parse::<usize>()
    .unwrap();

  len * code
}

pub struct Day21 {}

impl AOCDay for Day21 {
  fn name(&self) -> String {
    "day21".to_string()
  }

  fn test_answer_part1(&self) -> String {
    PART_1_EXAMPLE.to_string()
  }

  fn test_answer_part2(&self) -> String {
    PART_2_EXAMPLE.to_string()
  }

  fn solve_part1(&self, input: &[String]) -> String {
    let codes = parse_input(input);
    let values = codes
      .iter()
      .map(|code| (code, generate_shortest_path(code)))
      .map(|(code, path)| calculate_complexity(&path, code))
      .collect::<Vec<usize>>();

    let result = values.iter().sum::<usize>();
    result.to_string()
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
  fn test_calculate_complexity() {
    let sequence =
      "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".to_string();
    let code = "029A".chars().collect::<Vec<char>>();
    assert_eq!(calculate_complexity(&sequence, &code), 1972);
  }

  #[test]
  fn test_calculate_complexity_full() {
    let code = "029A".chars().collect::<Vec<char>>();
    let path = generate_shortest_path(&code);
    assert_eq!(calculate_complexity(&path, &code), 1972);
  }

  #[test]
  fn test_029a() {
    let code = "029A".chars().collect::<Vec<char>>();
    let path = generate_shortest_path(&code);
    let complexity = calculate_complexity(&path, &code);
    assert_eq!(path.len(), 68);
    assert_eq!(complexity, 1972);
  }

  #[test]
  fn test_980a() {
    let code = "980A".chars().collect::<Vec<char>>();
    let path = generate_shortest_path(&code);
    let complexity = calculate_complexity(&path, &code);
    assert_eq!(path.len(), 60);
    assert_eq!(complexity, 58800);
  }

  #[test]
  fn test_179a() {
    let code = "179A".chars().collect::<Vec<char>>();
    let path = generate_shortest_path(&code);
    let complexity = calculate_complexity(&path, &code);
    assert_eq!(path.len(), 68);
    assert_eq!(complexity, 12172);
  }

  #[test]
  fn test_456a() {
    let code = "456A".chars().collect::<Vec<char>>();
    let path = generate_shortest_path(&code);
    let complexity = calculate_complexity(&path, &code);
    assert_eq!(path.len(), 64);
    assert_eq!(complexity, 29184);
  }

  #[test]
  fn test_379a() {
    let code = "379A".chars().collect::<Vec<char>>();
    let path = generate_shortest_path(&code);
    let complexity = calculate_complexity(&path, &code);
    assert_eq!(path.len(), 64);
    assert_eq!(complexity, 24256);
  }

  #[test]
  fn test_part_1_example() {
    let day = Day21 {};
    assert_eq!(
      PART_1_EXAMPLE,
      day.solve_part1(&read_file("input/day21/test1.txt"))
    );
  }

  #[test]
  fn test_part_1() {
    let day = Day21 {};
    assert_eq!(
      "188398",
      day.solve_part1(&read_file("input/day21/part1.txt"))
    );
  }

  // #[test]
  // fn test_part_2_example() {
  //   let day = Day21 {};
  //   assert_eq!(
  //     PART_2_EXAMPLE,
  //     day.solve_part2(&read_file("input/day21/test1.txt"))
  //   );
  // }

  // #[test]
  // fn test_part_2() {
  //   let day = Day21 {};
  //   assert_eq!("TODO", day.solve_part2(&read_file("input/day21/part1.txt")));
  // }
}
