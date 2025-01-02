use crate::{utils, AOCDay};

// Two inputs. Answers are:
// 10092 and
// 2028
const PART_1_EXAMPLE: &str = "10092";
const PART_2_EXAMPLE: &str = "9021";

type Map = Vec<Vec<char>>;

fn parse_input(input: &[String]) -> (Map, String) {
  let empty_line_idx = input
    .iter()
    .enumerate()
    .find(|(_, line)| line.is_empty())
    .unwrap()
    .0;

  let (map, instructions) = input.split_at(empty_line_idx);

  let map = map.iter().map(|line| line.chars().collect()).collect();

  (map, instructions.join(""))
}

#[allow(dead_code)]
fn double_map(map: &Map) -> Map {
  let mut new = vec![vec!['.'; map[0].len() * 2]; map.len()];

  for (y, row) in map.iter().enumerate() {
    for (x, cell) in row.iter().enumerate() {
      match cell {
        '.' | '#' => {
          new[y][x * 2] = *cell;
          new[y][x * 2 + 1] = *cell;
        }
        'O' => {
          new[y][x * 2] = '[';
          new[y][x * 2 + 1] = ']';
        }
        '@' => {
          new[y][x * 2] = '@';
          new[y][x * 2 + 1] = '.';
        }
        _ => panic!("Invalid cell"),
      }
    }
  }

  new
}

fn find_robot(map: &Map) -> (usize, usize) {
  for (y, row) in map.iter().enumerate() {
    for (x, cell) in row.iter().enumerate() {
      if *cell == '@' {
        return (x, y);
      }
    }
  }

  panic!("Could not find robot");
}

#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
fn can_make_move(
  original_map: &Map,
  map: &mut Map,
  instruction: char,
  current_loc: (usize, usize),
) -> bool {
  let (x_dir, y_dir): (isize, isize) = match instruction {
    '^' => (0, -1),
    'v' => (0, 1),
    '<' => (-1, 0),
    '>' => (1, 0),
    _ => panic!("Invalid instruction"),
  };

  let new_x = current_loc.0 as isize + x_dir;
  let new_y = current_loc.1 as isize + y_dir;

  assert!(
    !(new_x < 0 || new_y < 0 || new_y as usize >= map.len() || new_x as usize >= map[0].len()),
    "Move out of bounds"
  );

  let new_x = utils::isize_to_usize_x(new_x);
  let new_y = utils::isize_to_usize_x(new_y);

  match original_map[new_y].get(new_x) {
    // If empty space, move contents of current location into new location and return true
    Some('.') => {
      map[new_y][new_x] = original_map[current_loc.1][current_loc.0];
      true
    }
    Some('O') => {
      // If box, keep looking along the same direction to
      // see if you find a wall or a space next
      if can_make_move(original_map, map, instruction, (new_x, new_y)) {
        map[new_y][new_x] = original_map[current_loc.1][current_loc.0];
        true
      } else {
        false
      }
    }
    Some('[') => match instruction {
      '^' | 'v' => {
        if can_make_move(original_map, map, instruction, (new_x, new_y))
          && can_make_move(original_map, map, instruction, (new_x + 1, new_y))
        {
          map[new_y][new_x] = original_map[current_loc.1][current_loc.0];
          if original_map[current_loc.1][current_loc.0] == '@' {
            map[new_y][new_x + 1] = '.';
          }
          true
        } else {
          false
        }
      }
      '<' | '>' => {
        if can_make_move(original_map, map, instruction, (new_x, new_y)) {
          map[new_y][new_x] = original_map[current_loc.1][current_loc.0];
          true
        } else {
          false
        }
      }
      _ => panic!("Invalid instruction"),
    },
    Some(']') => match instruction {
      '^' | 'v' => {
        if can_make_move(original_map, map, instruction, (new_x, new_y))
          && can_make_move(original_map, map, instruction, (new_x - 1, new_y))
        {
          map[new_y][new_x] = original_map[current_loc.1][current_loc.0];
          if original_map[current_loc.1][current_loc.0] == '@' {
            map[new_y][new_x - 1] = '.';
          }
          true
        } else {
          false
        }
      }
      '<' | '>' => {
        if can_make_move(original_map, map, instruction, (new_x, new_y)) {
          map[new_y][new_x] = map[current_loc.1][current_loc.0];
          true
        } else {
          false
        }
      }
      _ => panic!("Invalid instruction"),
    },
    Some('#') => false,
    _ => panic!("Invalid move"),
  }
}

#[allow(dead_code)]
fn print_map(map: &Map) {
  for row in map {
    println!("{}", row.iter().collect::<String>());
  }
  println!("----------------\n");
}

fn is_valid_map(map: &Map) -> bool {
  let mut robot_count = 0;
  let mut open_count = 0;
  let mut close_count = 0;

  for row in map {
    for cell in row {
      match cell {
        '@' => robot_count += 1,
        '[' => open_count += 1,
        ']' => close_count += 1,
        _ => (),
      }
    }
  }

  let counts = robot_count == 1 && open_count == close_count;

  for (y, row) in map.iter().enumerate() {
    for (x, cell) in row.iter().enumerate() {
      if *cell == '[' && map[y][x + 1] != ']' {
        return false;
      }

      if *cell == ']' && map[y][x - 1] != '[' {
        return false;
      }
    }
  }

  counts
}

fn fix_map(map: &mut Map) {
  let new_map = map.clone();
  for (y, row) in new_map.iter().enumerate() {
    for (x, cell) in row.iter().enumerate() {
      if *cell == '[' && map[y][x - 1] == '[' {
        map[y][x - 1] = '.';
      }

      if *cell == ']' && map[y][x + 1] == ']' {
        map[y][x + 1] = '.';
      }
    }
  }
}

pub struct Day15 {}

impl AOCDay for Day15 {
  fn name(&self) -> String {
    "day15".to_string()
  }

  fn test_answer_part1(&self) -> String {
    PART_1_EXAMPLE.to_string()
  }

  fn test_answer_part2(&self) -> String {
    PART_2_EXAMPLE.to_string()
  }

  fn solve_part1(&self, input: &[String]) -> String {
    let (mut map, instructions) = parse_input(input);

    for instruction in instructions.chars() {
      let robot_loc = find_robot(&map);
      let mut new_map = map.clone();
      if can_make_move(&map, &mut new_map, instruction, robot_loc) {
        new_map[robot_loc.1][robot_loc.0] = '.';
        map = new_map;
      }
    }

    let result = map
      .iter()
      .enumerate()
      .flat_map(|(row_idx, row)| {
        row.iter().enumerate().map(move |(col_idx, cell)| {
          if *cell == 'O' {
            100 * row_idx + col_idx
          } else {
            0
          }
        })
      })
      .sum::<usize>();
    result.to_string()
  }

  fn solve_part2(&self, input: &[String]) -> String {
    let (start_map, instructions) = parse_input(input);
    let mut map = double_map(&start_map);

    for instruction in instructions.chars() {
      let robot_loc = find_robot(&map);
      let mut new_map = map.clone();
      if can_make_move(&map, &mut new_map, instruction, robot_loc) {
        new_map[robot_loc.1][robot_loc.0] = '.';
        map = new_map;
        if !is_valid_map(&map) {
          // I feel dirty, but I understand why this hapepens, and it was easier than
          // stopping it happening in the first place!
          fix_map(&mut map);
        }
      }
    }

    let result = map
      .iter()
      .enumerate()
      .flat_map(|(row_idx, row)| {
        row.iter().enumerate().map(move |(col_idx, cell)| {
          if *cell == '[' {
            100 * row_idx + col_idx
          } else {
            0
          }
        })
      })
      .sum::<usize>();
    result.to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::read_file;

  #[test]
  fn test_part_1_example() {
    let day = Day15 {};
    assert_eq!(
      PART_1_EXAMPLE,
      day.solve_part1(&read_file("input/day15/test1.txt"))
    );
  }

  #[test]
  fn test_part_1() {
    let day = Day15 {};
    assert_eq!(
      "1294459",
      day.solve_part1(&read_file("input/day15/part1.txt"))
    );
  }

  #[test]
  fn test_part_2_example() {
    let day = Day15 {};
    assert_eq!(
      PART_2_EXAMPLE,
      day.solve_part2(&read_file("input/day15/test1.txt"))
    );
  }

  #[test]
  fn test_part_2() {
    let day = Day15 {};
    assert_eq!(
      "1319212",
      day.solve_part2(&read_file("input/day15/part1.txt"))
    );
  }
}
