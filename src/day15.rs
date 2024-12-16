use std::cmp::{max, min};
use crate::AOCDay;

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

fn calculate_result(map: &Map) -> usize {
  map
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
    .sum::<usize>()
}

#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
fn make_move(map: &mut Map, instruction: char) {
  let robot_loc = find_robot(map);
  let (x_dir, y_dir): (isize, isize) = match instruction {
    '^' => (0, -1),
    'v' => (0, 1),
    '<' => (-1, 0),
    '>' => (1, 0),
    _ => panic!("Invalid instruction"),
  };

  let new_x = robot_loc.0 as isize + x_dir;
  let new_y = robot_loc.1 as isize + y_dir;

  assert!(
    !(new_x < 0 || new_y < 0 || new_y as usize >= map.len() || new_x as usize >= map[0].len()),
    "Move out of bounds"
  );

  match map[new_y as usize].get(new_x as usize) {
    // If empty space, just move robot into space
    Some('.') => {
      map[robot_loc.1][robot_loc.0] = '.';
      map[new_y as usize][new_x as usize] = '@';
    }
    Some('O') => {
      // If box, keep looking along the same direction to
      // see if you find a wall or a space next
      let mut cur_loc = (new_x as usize, new_y as usize);
      while map[cur_loc.1][cur_loc.0] == 'O' {
        cur_loc = (
          (cur_loc.0 as isize + x_dir) as usize,
          (cur_loc.1 as isize + y_dir) as usize,
        );
        assert!(
          !(cur_loc.0 >= map[0].len() || cur_loc.1 >= map.len()),
          "Move out of bounds"
        );
      }
      match map[cur_loc.1].get(cur_loc.0) {
        Some('#') => {
          // If wall, do nothing
        }
        Some('.') => {
          // If space, move robot and boxes
          map[robot_loc.1][robot_loc.0] = '.';
          map[(robot_loc.1 as isize + y_dir) as usize][(robot_loc.0 as isize + x_dir) as usize] =
            '@';
          map[cur_loc.1][cur_loc.0] = 'O';
        }
        _ => panic!("Invalid move"),
      }
    }
    Some('#') => {
      // Do nothing
    }
    _ => panic!("Invalid move"),
  }
}

#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
fn make_move2(map: &mut Map, instruction: char) {
  let robot_loc = find_robot(map);
  let (x_dir, y_dir): (isize, isize) = match instruction {
    '^' => (0, -1),
    'v' => (0, 1),
    '<' => (-1, 0),
    '>' => (1, 0),
    _ => panic!("Invalid instruction"),
  };

  let new_x = robot_loc.0 as isize + x_dir;
  let new_y = robot_loc.1 as isize + y_dir;

  assert!(
    !(new_x < 0 || new_y < 0 || new_y as usize >= map.len() || new_x as usize >= map[0].len()),
    "Move out of bounds"
  );

  match map[new_y as usize].get(new_x as usize) {
    // If empty space, just move robot into space
    Some('.') => {
      map[robot_loc.1][robot_loc.0] = '.';
      map[new_y as usize][new_x as usize] = '@';
    }
    Some('[' | ']') => {
      match instruction {
        '<' | '>' => {
          // If box, keep looking along the same direction to
          // see if you find a wall or a space next
          let mut cur_loc = (new_x as usize, new_y as usize);
          while map[cur_loc.1][cur_loc.0] == '[' || map[cur_loc.1][cur_loc.0] == ']' {
            cur_loc = (
              (cur_loc.0 as isize + 2* x_dir) as usize,
              cur_loc.1,
            );
            assert!(
              !(cur_loc.0 >= map[0].len() || cur_loc.1 >= map.len()),
              "Move out of bounds"
            );
          }
          match map[cur_loc.1].get(cur_loc.0) {
            Some('#') => {
              // If wall, do nothing
            }
            Some('.') => {
              // If space, move robot and boxes
              map[robot_loc.1][robot_loc.0] = '.';
              map[robot_loc.1][(robot_loc.0 as isize + x_dir) as usize] =
                '@';

              let first_box_idx = (robot_loc.0 as isize + 2 * x_dir) as usize;
              for i in min(cur_loc.0, first_box_idx)..=max(cur_loc.0, first_box_idx) {
                match map[cur_loc.1].get(i) {
                  Some('[') => {
                    map[cur_loc.1][i] = ']';
                  }
                  Some(']') => {
                    map[cur_loc.1][i] = '[';
                  }
                  Some('@') => {
                    // Do nothing
                  }
                  Some('.') => {
                    if instruction == '<' {
                      map[cur_loc.1][i] = '[';
                    } else {
                      map[cur_loc.1][i] = ']';
                    }
                  }
                  Some(value) => panic!("Unexpected value: {value}"),
                  _ => panic!("Invalid move"),
                }
              }
              map[cur_loc.1][cur_loc.0] = '[';
            }
            _ => panic!("Invalid move"),
          }
        }
        '^' | 'v' => {
          // This is the tricky part. If we have boxes above the box above/below us, and those boxes
          // are offset by one then we need to grow the width of our search space. And if _those_ boxes
          // have boxes above them offset by one we need to grow again.
          // i.e:
          // ##############
          // ##......##..##
          // ##..[][][]..##
          // ##...[][]...##
          // ##....[]....##
          // ##.....@....##
          // ##############
          //
          // Need to think about a case like this:
          // ##############
          // ##....[]....##
          // ##..[]..[]..##
          // ##...[][]...##
          // ##....[]....##
          // ##.....@....##
          // ##############
          // 
          // And this:
          // ##############
          // ##..........##
          // ##...[]..[].##
          // ##..[][][]..##
          // ##...[][]...##
          // ##....[]....##
          // ##.....@....##
          // ##############
          let mut boxes: Vec<Vec<(usize, usize)>> = vec![];
          if map[new_y as usize][robot_loc.0] == '[' {
            boxes.push(vec!((robot_loc.0, robot_loc.0 + 1); 1));
          } else {
            boxes.push(vec!((robot_loc.0 - 1, robot_loc.0); 1));
          }
          
          // Grow the width
          for box_ in  boxes.last().unwrap() {
            println!("{}", box_.0);
          }
        }
        _ => panic!("Invalid move"),
      }
    }
    Some('#') => {
      // Do nothing
    }
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
      make_move(&mut map, instruction);
    }

    let result = calculate_result(&map);
    result.to_string()
  }

  fn solve_part2(&self, input: &[String]) -> String {
    let (start_map, instructions) = parse_input(input);
    let mut map = double_map(&start_map);
    print_map(&map);

    for instruction in instructions.chars() {
      make_move2(&mut map, instruction);
      print_map(&map);
    }

    let result = calculate_result(&map);
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
    assert_eq!("TODO", day.solve_part2(&read_file("input/day15/part1.txt")));
  }
}
