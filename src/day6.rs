use std::collections::HashSet;
use strum_macros::EnumIter; 

use crate::AOCDay;

pub struct Day6 {}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
enum Tile {
  Starting,
  Empty,
  Visited,
  Obstacle,
}

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, std::cmp::Eq, std::hash::Hash)]
enum Facing {
  Up,
  Down,
  Left,
  Right,
}

impl Facing {
  fn turn_clockwise(self) -> Facing {
    match self {
      Facing::Up => Facing::Right,
      Facing::Right => Facing::Down,
      Facing::Down => Facing::Left,
      Facing::Left => Facing::Up,
    }
  }
}

type Map = Vec<Vec<Tile>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
  row: usize,
  col: usize,
}

#[allow(dead_code)]
fn print_map(map: &Map) {
  for row in map {
    for tile in row {
      match tile {
        Tile::Starting => print!("^"),
        Tile::Empty => print!("."),
        Tile::Visited => print!("X"),
        Tile::Obstacle => print!("#"),
      }
    }
    println!();
  }
}

fn tile_from_string(string: &str) -> Tile {
  match string {
    "." => Tile::Empty,
    "^" => Tile::Starting,
    "#" => Tile::Obstacle,
    _ => panic!("Unexpected input")
  }
}

fn parse_input(input: &[String]) -> Map {
  let mut result: Map = Map::new();
  
  for line in input {
    if !line.is_empty() {
      let line_by_character = line.clone().split("").map(ToString::to_string).collect::<Vec<String>>();
      let trimmed_line_by_character = line_by_character[1..line_by_character.len()-1].to_vec();            
      result.push(
        trimmed_line_by_character
        .into_iter()
        .map(|char| tile_from_string(&char))
        .collect::<Vec<Tile>>()
      );
    }
  }
  
  result
}

fn get_tile(map: &Map, row: usize, col: usize) -> &Tile {
  match map.get(row) {
    Some(row) => row.get(col).expect("Invalid location"),
    None => panic!("Invalid location"),
  }
}

fn next_location(map: &Map, location: Location, facing: Facing) -> Option<(Location, Facing)> {
  let row = location.row; 
  let col = location.col;
  let max_row = map.len() - 1;
  let max_col = map[0].len() - 1;

  match (facing, row, col) {
    (Facing::Up, 0, _) | (Facing::Left, _, 0) => None,
    (Facing::Down, row, _) if row == max_row => None,
    (Facing::Right, _, col) if col == max_col => None,
    _ => {
      let new_location = match facing {
        Facing::Up => Location { row: row - 1, col },
        Facing::Down => Location { row: row + 1, col },
        Facing::Left => Location { row, col: col - 1 },
        Facing::Right => Location { row, col: col + 1 },
      };

      let tile = get_tile(map, new_location.row, new_location.col);
      match tile {
        Tile::Obstacle => Some((location, facing.turn_clockwise())),
        Tile::Empty | Tile::Visited | Tile::Starting => {
          Some((new_location, facing))
        },
      }
    },
  }
}

fn find_starting_location(map: &Map) -> Location {
  for (row_index, row) in map.iter().enumerate() {
    for (col_index, tile) in row.iter().enumerate() {
      if *tile == Tile::Starting {
        return Location { row: row_index, col: col_index };
      }
    }
  }
  panic!("No starting location found");
}

fn run_simulation(map: &Map, starting_location: Location, facing: Facing) -> HashSet<Location> {
  let mut current_location = starting_location;
  let mut current_facing = facing;

  let mut visited = HashSet::new();
  visited.insert((starting_location, facing));

  loop {
    let next = next_location(map, current_location, current_facing);
    match next {
      Some(( new_location, new_facing)) => {
        current_location = new_location;
        current_facing = new_facing;
        visited.insert((new_location, new_facing));
      },
      None => break,
    }
  }

  visited.iter().map(|(location, _)| * location).collect()
}

fn contains_loop(map: &Map, starting_location: Location, facing: Facing) -> bool {
  let mut current_location = starting_location;
  let mut current_facing = facing;

  let mut visited = HashSet::new();
  visited.insert((starting_location, facing));

  loop {
    let next = next_location(map, current_location, current_facing);
    match next {
      Some(( new_location, new_facing)) => {
        current_location = new_location;
        current_facing = new_facing;

        if !visited.insert((new_location, new_facing)) {
          return true;
        }
      },
      None => break,
    }
  }

  false
}

impl AOCDay for Day6 {
  fn name(&self) -> String {
    "day6".to_string()
  }
  
  fn test_answer_part1(&self) -> String {
    "41".to_string()
  }
  
  fn test_answer_part2(&self) -> String {
    "6".to_string()
  }
  
  fn solve_part1(&self, input: &[String]) -> String {
    let map = parse_input(input);
    let starting_location = find_starting_location(&map);
    
    let visited = run_simulation(&map, starting_location, Facing::Up);
    visited.len().to_string()
  }
  
  fn solve_part2(&self, input: &[String]) -> String {
    let map = parse_input(input);
    let starting_location = find_starting_location(&map);
    
    let visited_ids = run_simulation(&map, starting_location, Facing::Up);

    let count = visited_ids.iter().filter(|location| {
      let mut inner_map = map.clone();
      inner_map[location.row][location.col] = Tile::Obstacle;
      contains_loop(&inner_map, starting_location, Facing::Up)
    }).count();

    count.to_string()
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_file;

    #[test]
  fn test_part_1() {
    let day = Day6 {};
    assert_eq!(
      "4663",
      day.solve_part1(&read_file("input/day6/part1.txt"))
    );
  }

  #[test]
  fn test_part_2() {
    let day = Day6 {};
    assert_eq!(
      "1530",
      day.solve_part2(&read_file("input/day6/part1.txt"))
    );
  }
}