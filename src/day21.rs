use crate::AOCDay;
use itertools::Itertools;
use memoize::memoize;
use std::cmp::Ordering;
use std::string::String;

const PART_1_EXAMPLE: &str = "126384";
const PART_2_EXAMPLE: &str = "SKIP";

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
#[memoize]
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

#[memoize]
fn move_numeric(from: Position, to: Position) -> Vec<Command> {
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
    commands = y_commands;
  } else if y_commands.is_empty() {
    commands = x_commands;
  } else {
    #[allow(clippy::match_same_arms)]
    match ((from.x), (to.x), (from.y), (to.y)) {
      (0, _, 0, _) | (_, 0, _, 0) => {
        panic!("Cannot start or end from 0,0")
      }
      // x starts on 0 and y ends on 0.
      // Must go x then y
      (0, _, _, 0) => {
        commands.append(&mut x_commands);
        commands.append(&mut y_commands);
      }
      // y starts on 0 and x ends on 0.
      // Must go y then x
      (_, 0, 0, _) => {
        commands.append(&mut y_commands);
        commands.append(&mut x_commands);
      }
      // Otherwise, if x changes go left, choose X first
      (_, _, _, _) => {
        if x_commands[0].direction == '<' {
          commands.append(&mut x_commands);
          commands.append(&mut y_commands);
        } else {
          commands.append(&mut y_commands);
          commands.append(&mut x_commands);
        }
      }
    };
  }

  commands.push(Command {
    direction: 'A',
    steps: 1,
  });

  commands
}

// MARK - Directional Helpers

// Directional Keypad:
//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
#[memoize]
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

#[memoize]
fn move_directional(from: Position, to: Position) -> Vec<Command> {
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
    commands = y_commands;
  } else if y_commands.is_empty() {
    commands = x_commands;
  } else {
    #[allow(clippy::match_same_arms)]
    match ((from.x), (to.x), (from.y), (to.y)) {
      (0, _, 1, _) | (_, 0, _, 1) => {
        panic!("Cannot start or end from 0,1")
      }
      // x starts on 0 and y starts on 0.
      // Must go x then y
      (0, _, 0, _) => {
        commands.append(&mut x_commands);
        commands.append(&mut y_commands);
      }
      // y starts on 1 and x ends on 0.
      // Must go y then x
      (_, 0, _, _) => {
        commands.append(&mut y_commands);
        commands.append(&mut x_commands);
      }
      // Otherwise, if x changes go left, choose X first
      (_, _, _, _) => {
        if x_commands[0].direction == '<' {
          commands.append(&mut x_commands);
          commands.append(&mut y_commands);
        } else {
          commands.append(&mut y_commands);
          commands.append(&mut x_commands);
        }
      }
    };
  }

  commands.push(Command {
    direction: 'A',
    steps: 1,
  });

  commands
}

#[memoize]
fn generate_directional_command_len(
  position: Position,
  sequence: Vec<char>,
  level: usize,
) -> usize {
  if level == 0 {
    return sequence.len();
  }

  let mut result: usize = 0;
  let mut current_pos = position;

  for c in sequence {
    let next_pos = pos_for_directional_keypad_char(c);
    let commands = move_directional(current_pos, next_pos);
    let path = path_from_commands(commands);
    let segment_len = generate_directional_command_len(position, path, level - 1);
    result += segment_len;
    current_pos = next_pos;
  }

  result
}

#[memoize]
fn generate_shortest_path_len_for(code: Vec<char>, is_part_2: bool) -> usize {
  let mut result: usize = 0;
  let mut current_pos = pos_for_numeric_keypad_char('A');

  for c in code {
    let next_pos = pos_for_numeric_keypad_char(c);
    let first_robot_command = move_numeric(current_pos, next_pos);
    let first_path = path_from_commands(first_robot_command);

    let final_path_len = generate_directional_command_len(
      pos_for_directional_keypad_char('A'),
      first_path,
      if is_part_2 { 25 } else { 2 },
    );
    result += final_path_len;
    current_pos = next_pos;
  }

  result
}

// Helper functions

#[memoize]
#[allow(clippy::needless_pass_by_value)]
fn path_from_commands(commands: Vec<Command>) -> Vec<char> {
  commands
    .iter()
    .flat_map(|c| {
      std::iter::repeat(c.direction)
        .take(c.steps)
        .collect::<Vec<char>>()
    })
    .collect::<Vec<char>>()
}

fn calculate_complexity(path_len: usize, code: &[char]) -> usize {
  let code = code
    .iter()
    .skip_while(|c| **c == '0')
    .filter(|c| c.is_numeric())
    .collect::<String>()
    .parse::<usize>()
    .unwrap();

  path_len * code
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
      .map(|code| (code, generate_shortest_path_len_for(code.clone(), false)))
      .map(|(code, path_len)| calculate_complexity(path_len, code))
      .collect::<Vec<usize>>();

    let result = values.iter().sum::<usize>();
    result.to_string()
  }

  fn solve_part2(&self, input: &[String]) -> String {
    let codes = parse_input(input);

    // No test to run
    if codes[0].iter().join("") == "029A" {
      return "SKIP".to_string();
    }

    let values = codes
      .iter()
      .map(|code| (code, generate_shortest_path_len_for(code.clone(), true)))
      .map(|(code, path_len)| calculate_complexity(path_len, code))
      .collect::<Vec<usize>>();

    let result = values.iter().sum::<usize>();
    result.to_string()
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
    assert_eq!(calculate_complexity(sequence.len(), &code), 1972);
  }

  #[test]
  fn test_calculate_complexity_full() {
    let code = "029A".chars().collect::<Vec<char>>();
    let path_len = generate_shortest_path_len_for(code.clone(), false);
    assert_eq!(calculate_complexity(path_len, &code), 1972);
  }

  #[test]
  fn test_0_numeric() {
    let commands = move_numeric(
      pos_for_numeric_keypad_char('A'),
      pos_for_numeric_keypad_char('0'),
    );
    let path = path_from_commands(commands);
    assert_eq!(path, vec!['<', 'A']);
    assert_eq!(path.len(), 2);
  }

  #[test]
  fn test_0_first_robot() {
    let first_commands = move_numeric(
      pos_for_numeric_keypad_char('A'),
      pos_for_numeric_keypad_char('0'),
    );
    let first_path = path_from_commands(first_commands);
    let second_robot_path_len =
      generate_directional_command_len(pos_for_directional_keypad_char('A'), first_path, 1);

    assert_eq!(second_robot_path_len, 8);
  }

  #[test]
  fn test_0() {
    let code = "0".chars().collect::<Vec<char>>();
    let path_len = generate_shortest_path_len_for(code, false);
    assert_eq!(path_len, 18);
  }

  #[test]
  fn test_029a() {
    let code = "029A".chars().collect::<Vec<char>>();
    let path_len = generate_shortest_path_len_for(code.clone(), false);
    let complexity = calculate_complexity(path_len, &code);
    assert_eq!(path_len, 68);
    assert_eq!(complexity, 1972);
  }

  #[test]
  fn test_980a() {
    let code = "980A".chars().collect::<Vec<char>>();
    let path_len = generate_shortest_path_len_for(code.clone(), false);
    let complexity = calculate_complexity(path_len, &code);
    assert_eq!(path_len, 60);
    assert_eq!(complexity, 58800);
  }

  #[test]
  fn test_179a() {
    let code = "179A".chars().collect::<Vec<char>>();
    let path_len = generate_shortest_path_len_for(code.clone(), false);
    let complexity = calculate_complexity(path_len, &code);
    assert_eq!(path_len, 68);
    assert_eq!(complexity, 12172);
  }

  #[test]
  fn test_456a() {
    let code = "456A".chars().collect::<Vec<char>>();
    let path_len = generate_shortest_path_len_for(code.clone(), false);
    let complexity = calculate_complexity(path_len, &code);
    assert_eq!(path_len, 64);
    assert_eq!(complexity, 29184);
  }

  #[test]
  fn test_379a() {
    let code = "379A".chars().collect::<Vec<char>>();
    let path_len = generate_shortest_path_len_for(code.clone(), false);
    let complexity = calculate_complexity(path_len, &code);
    assert_eq!(path_len, 64);
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

  #[test]
  fn test_part_2() {
    let day = Day21 {};
    assert_eq!(
      "230049027535970",
      day.solve_part2(&read_file("input/day21/part1.txt"))
    );
  }
}
