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

#[derive(Debug, Clone)]
struct Cell {
  tile: Tile,
  visited_dirns: Vec<Facing>,
}

type Map = Vec<Vec<Cell>>;

#[derive(Debug, Clone, Copy)]
struct Location {
  row: usize,
  col: usize,
}

#[allow(clippy::manual_find)]
fn find_duplicate<T: Eq + std::hash::Hash>(vec: &[T]) -> Option<&T> {
  let mut seen = HashSet::new();
  for item in vec {
    if !seen.insert(item) {
      return Some(item);
    }
  }
  None
}

#[allow(dead_code)]
fn print_map(map: &Map) {
  for row in map {
    for cell in row {
      match cell.tile {
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
        .map(|char| Cell { tile: tile_from_string(&char), visited_dirns: Vec::new() })
        .collect::<Vec<Cell>>()
      );
    }
  }
  
  result
}

fn get_cell(map: &Map, row: usize, col: usize) -> &Cell {
  match map.get(row) {
    Some(row) => row.get(col).expect("Invalid location"),
    None => panic!("Invalid location"),
  }
}

fn update_map(map: &mut Map, location: Location, facing: Facing) {
  let cell = &mut map[location.row][location.col];
  cell.tile = Tile::Visited;
  cell.visited_dirns.push(facing);
}

fn next_location(map: &mut Map, location: Location, facing: Facing) -> Option<(Location, Facing)> {
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

      let new_cell = get_cell(map, new_location.row, new_location.col);
      match new_cell.tile {
        Tile::Obstacle => Some((location, facing.turn_clockwise())),
        Tile::Empty | Tile::Visited | Tile::Starting => {
          update_map(map, new_location, facing);
          Some((new_location, facing))
        },
      }
    },
  }
}

fn find_starting_location(map: &Map) -> Location {
  for (row_index, row) in map.iter().enumerate() {
    for (col_index, cell) in row.iter().enumerate() {
      if cell.tile == Tile::Starting {
        return Location { row: row_index, col: col_index };
      }
    }
  }
  panic!("No starting location found");
}

fn run_simulation(map: &mut Map, location: Location, facing: Facing) -> &Map {
  let mut current_location = location;
  let mut current_facing = facing;

  map[location.row][location.col] = Cell { tile: Tile::Visited, visited_dirns: vec![facing] };

  loop {
    let next = next_location(map, current_location, current_facing);
    match next {
      Some(( new_location, new_facing)) => {
        current_location = new_location;
        current_facing = new_facing;
      },
      None => break,
    }
  }

  map
}

fn contains_loop(map: &mut Map, location: Location, facing: Facing) -> bool {
  let mut current_location = location;
  let mut current_facing = facing;

  map[location.row][location.col] = Cell { tile: Tile::Visited, visited_dirns: vec![facing] };

  loop {
    let next = next_location(map, current_location, current_facing);
    match next {
      Some(( new_location, new_facing)) => {
        current_location = new_location;
        current_facing = new_facing;

        let visited_dirns = &get_cell(map, new_location.row, new_location.col).visited_dirns;
        if let Some(_location) = find_duplicate(visited_dirns) {
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
    let mut map = parse_input(input);
    let starting_location = find_starting_location(&map);
    
    run_simulation(&mut map, starting_location, Facing::Up);

    let visited = map.iter().flatten().filter(|cell| cell.tile == Tile::Visited).count();
    visited.to_string()
  }
  
  fn solve_part2(&self, input: &[String]) -> String {
    let starting_map = parse_input(input);
    let starting_location = find_starting_location(&starting_map);
    
    let mut map = starting_map.clone();
    run_simulation(&mut map, starting_location, Facing::Up);

    let visited_idxs = map
    .iter()
    .enumerate()
    .flat_map(|(row_idx, row)|{
      row
      .iter()
      .enumerate()
      .filter_map(move |(col_idx, cell)| {
        if cell.tile == Tile::Visited {
          Some(Location { row: row_idx, col: col_idx })
        } else {
          None
        }
      })
    });

    let count = visited_idxs.filter(|location| {
      let mut inner_map = starting_map.clone();
      inner_map[location.row][location.col].tile = Tile::Obstacle;
      contains_loop(&mut inner_map, starting_location, Facing::Up)
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