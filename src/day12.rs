use crate::AOCDay;

// 140
// 772
// 1930
const PART_1_EXAMPLE: &str = "1930";

// 80
// 436
// 236
// 368
// 1206
const PART_2_EXAMPLE: &str = "1206";

type Map = Vec<Vec<Cell>>;

struct Cell {
  crop: char,
  island_id: Option<usize>,
  row: usize,
  col: usize,
}

fn parse_input(input: &[String]) -> Vec<Vec<Cell>> {
  input
    .iter()
    .enumerate()
    .map(|(row_idx, line)| {
      line
        .chars()
        .enumerate()
        .map(|(col_idx, char)| Cell {
          crop: char,
          island_id: None,
          row: row_idx,
          col: col_idx,
        })
        .collect()
    })
    .collect()
}

fn find_valid_neighbours(map: &Map, row: usize, col: usize) -> Vec<(usize, usize)> {
  let mut neighbors = Vec::new();

  if row > 0 {
    neighbors.push((row - 1, col)); // Up
  }
  if row < map.len() - 1 {
    neighbors.push((row + 1, col)); // Down
  }
  if col > 0 {
    neighbors.push((row, col - 1)); // Left
  }
  if col < map[0].len() - 1 {
    neighbors.push((row, col + 1)); // Right
  }

  neighbors
}

fn find_detached_neighbours(map: &Map, row: usize, col: usize) -> Vec<(usize, usize)> {
  find_valid_neighbours(map, row, col)
    .iter()
    .filter(|(r, c)| map[*r][*c].island_id.is_none())
    .map(|(r, c)| (*r, *c))
    .collect()
}

#[allow(dead_code)]
fn print_map(map: &Map) {
  for row in map {
    for cell in row {
      let id: String = match cell.island_id {
        Some(id) => id.to_string(),
        None => "#".to_string(),
      };
      print!("{}{}", cell.crop, id);
    }
    println!();
  }
  println!("-----------------\n");
}

fn grow_island(map: &mut Map, row: usize, col: usize, id: usize) {
  let current_crop = map[row][col].crop;
  map[row][col].island_id = Some(id);
  for neighbour in find_detached_neighbours(map, row, col) {
    if map[neighbour.0][neighbour.1].crop == current_crop {
      grow_island(map, neighbour.0, neighbour.1, id);
    }
  }
}

fn find_islands(map: &mut Map) -> usize {
  let mut max_id = 0;
  let row_max = map.len();
  let col_max = map[0].len();

  for row_idx in 0..row_max {
    for col_idx in 0..col_max {
      if Option::is_none(&map[row_idx][col_idx].island_id) {
        max_id += 1;
        grow_island(map, row_idx, col_idx, max_id);
      }
    }
  }

  max_id
}

fn calculate_perimeter_length(map: &Map, row: usize, col: usize) -> usize {
  let mut perimeter = 4;
  let current_crop = map[row][col].crop;
  for neighbour in find_valid_neighbours(map, row, col) {
    if map[neighbour.0][neighbour.1].crop == current_crop {
      perimeter -= 1;
    }
  }
  perimeter
}

fn calculate_island_perimeter(map: &Map, island_id: usize) -> usize {
  map
    .iter()
    .flat_map(|row| row.iter())
    .filter(|cell| cell.island_id == Some(island_id))
    .map(|cell| calculate_perimeter_length(map, cell.row, cell.col))
    .sum()
}

fn calculate_number_of_sides(map: &Map, row: usize, col: usize) -> usize {
  let mut num_sides = calculate_perimeter_length(map, row, col);
  let current_crop = map[row][col].crop;

  // We reduce the top edge by one if the cell to the left is the same crop AND
  // the cell above is not the same crop AND
  // the cell above and to the left is not the same crop (or is out of bounds)
  if (col > 0 && map[row][col - 1].crop == current_crop)
    && (row == 0 || map[row - 1][col].crop != current_crop)
    && (row > 0 && col > 0 && map[row - 1][col - 1].crop != current_crop || row == 0 && col > 0)
  {
    num_sides -= 1;
  }

  // We reduce the bottom edge by one if the cell to the left is the same crop AND
  // the cell below is not the same crop AND
  // the cell below and to the left is not the same crop (or is out of bounds)
  if (col > 0 && map[row][col - 1].crop == current_crop)
    && (row == map.len() - 1 || map[row + 1][col].crop != current_crop)
    && (row < map.len() - 1 && col > 0 && map[row + 1][col - 1].crop != current_crop
      || row == map.len() - 1 && col > 0)
  {
    num_sides -= 1;
  }

  // We reduce the left edge by one if the cell above is the same crop AND
  // the cell to the left is not the same crop AND
  // the cell above and to the left is not the same crop (or is out of bounds)
  if (row > 0 && map[row - 1][col].crop == current_crop)
    && (col == 0 || map[row][col - 1].crop != current_crop)
    && (row > 0 && col > 0 && map[row - 1][col - 1].crop != current_crop || row > 0 && col == 0)
  {
    num_sides -= 1;
  }

  // We reduce the right edge by one if the cell above is the same crop AND
  // the cell to the right is not the same crop AND
  // the cell above and to the right is not the same crop (or is out of bounds)
  if (row > 0 && map[row - 1][col].crop == current_crop)
    && (col == map[0].len() - 1 || map[row][col + 1].crop != current_crop)
    && (row > 0 && col < map[0].len() - 1 && map[row - 1][col + 1].crop != current_crop
      || row > 0 && col == map[0].len() - 1)
  {
    num_sides -= 1;
  }

  num_sides
}

fn calculate_island_num_of_sides(map: &Map, island_id: usize) -> usize {
  let num_sides = map
    .iter()
    .flat_map(|row| row.iter())
    .filter(|cell| cell.island_id == Some(island_id))
    .map(|cell| calculate_number_of_sides(map, cell.row, cell.col))
    .sum();

  num_sides
}

fn calculate_island_area(map: &Map, island_id: usize) -> usize {
  map
    .iter()
    .flat_map(|row| row.iter())
    .filter(|cell| cell.island_id == Some(island_id))
    .count()
}

pub struct Day12 {}

impl AOCDay for Day12 {
  fn name(&self) -> String {
    "day12".to_string()
  }

  fn test_answer_part1(&self) -> String {
    PART_1_EXAMPLE.to_string()
  }

  fn test_answer_part2(&self) -> String {
    PART_2_EXAMPLE.to_string()
  }

  fn solve_part1(&self, input: &[String]) -> String {
    let mut map = parse_input(input);
    let max_id = find_islands(&mut map);

    let cost = (1..=max_id)
      .map(|id| calculate_island_perimeter(&map, id) * calculate_island_area(&map, id))
      .sum::<usize>();

    cost.to_string()
  }

  fn solve_part2(&self, input: &[String]) -> String {
    let mut map = parse_input(input);
    let max_id = find_islands(&mut map);

    let cost = (1..=max_id)
      .map(|id| calculate_island_num_of_sides(&map, id) * calculate_island_area(&map, id))
      .sum::<usize>();

    cost.to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::read_file;

  #[test]
  fn test_part_1_example() {
    let day = Day12 {};
    assert_eq!(
      PART_1_EXAMPLE,
      day.solve_part1(&read_file("input/day12/test1.txt"))
    );
  }

  #[test]
  fn test_part_1() {
    let day = Day12 {};
    assert_eq!(
      "1465112",
      day.solve_part1(&read_file("input/day12/part1.txt"))
    );
  }

  #[test]
  fn test_part_2_example() {
    let day = Day12 {};
    assert_eq!(
      PART_2_EXAMPLE,
      day.solve_part2(&read_file("input/day12/test1.txt"))
    );
  }

  #[test]
  fn test_part_2() {
    let day = Day12 {};
    assert_eq!(
      "893790",
      day.solve_part2(&read_file("input/day12/part1.txt"))
    );
  }
}
