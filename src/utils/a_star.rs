use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use crate::utils;

type Map = Vec<Vec<char>>;
type Path = Vec<(i32, i32)>;

#[derive(Debug, Eq, PartialEq)]
struct Node {
  position: (i32, i32),
  cost: i32,
  estimated_cost: i32,
}

impl Ord for Node {
  fn cmp(&self, other: &Self) -> Ordering {
    other.estimated_cost.cmp(&self.estimated_cost)
  }
}

impl PartialOrd for Node {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

fn calc_direction_cost(
  prev: Option<(i32, i32)>, 
  current: (i32, i32), 
  next: (i32, i32), 
  change_direction_penalty: i32
) -> i32 {
  if let Some(prev_pos) = prev {
    let prev_direction = (current.0 - prev_pos.0, current.1 - prev_pos.1);
    let next_direction = (next.0 - current.0, next.1 - current.1);

    if prev_direction != next_direction {
      if prev_direction == (0, 1) && next_direction == (0, -1) || 
        prev_direction == (0, -1) && next_direction == (0, 1) ||
        prev_direction == (1, 0) && next_direction == (-1, 0) ||
        prev_direction == (-1, 0) && next_direction == (1, 0)
      {
        // 180 degree turn, so add double cost
        return change_direction_penalty * 2;
      }

      // Otherwise, if directions are different add single cost
      return change_direction_penalty;
    }
  }
  0 // No penalty for continuing in the same direction
}

fn heuristic(pos: (i32, i32), goal: (i32, i32)) -> i32 {
  // Manhattan distance
  (pos.0 - goal.0).abs() + (pos.1 - goal.1).abs() 
}

fn neighbours(pos: (i32, i32), map: &Map) -> Vec<(i32, i32)> {
  let potential_neighbours = vec![
    (pos.0 - 1, pos.1),
    (pos.0 + 1, pos.1),
    (pos.0, pos.1 - 1),
    (pos.0, pos.1 + 1),
  ];
  
  potential_neighbours
    .into_iter()
    .filter(|&(x_, y_)| {
      let x = utils::i32_to_usize(x_).unwrap();
      let y = utils::i32_to_usize(y_).unwrap();

      // Must be larger than 0 due to conversion to usize
      // x >= 0 && 
      // y >= 0 && 
      x < map[0].len() && 
      y < map.len() && 
      map[y][x] != '#'
    })
    .collect::<Vec<(i32, i32)>>()
}

fn calculate_path_cost(path: &Path, movement_cost: i32, direction_change_cost: i32) -> i32 {
  // The windowing function below drops the first move, so we need to add the cost of the first move
  // Note: We assume the first move never carries any direction cost, i.e. you always start facing the
  // right direction initially
  movement_cost + 
  // The cost of all subsequent moves
  path.windows(3).fold(0, |acc, window| {
    acc + movement_cost + calc_direction_cost(Some(window[0]), window[1], window[2], direction_change_cost)
  })
}

pub fn with_direction_cost(
  map: &Map, 
  start: (i32, i32), 
  end: (i32, i32),
  movement_cost: i32,
  direction_change_cost: i32,
) -> Option<(Vec<(i32, i32)>, i32)> {
  let mut open_set = BinaryHeap::new();
  let mut g_score = HashMap::new();
  let mut came_from = HashMap::new();
  
  g_score.insert(start, 0);
  open_set.push(Node {
    position: start,
    cost: 0,
    estimated_cost: heuristic(start, end),
  });
  
  while let Some(current_node) = open_set.pop() {
    let mut current_pos = current_node.position;
    
    if current_pos == end {
      // Reconstruct the path
      let mut path = vec![current_pos];
      while let Some(&prev) = came_from.get(&current_pos) {
        path.push(prev);
        current_pos = prev;
      }
      
      path.reverse();
      
      let cost = calculate_path_cost(&path, movement_cost, direction_change_cost);
      
      return Some((path, cost));
    }
    
    for neighbour in neighbours(current_pos, map) {
      let tentative_g_score = g_score.get(&current_pos).unwrap_or(&i32::MAX)
      + movement_cost
      + calc_direction_cost(came_from.get(&current_pos).copied(), current_pos, neighbour, direction_change_cost);
      
      if tentative_g_score < *g_score.get(&neighbour).unwrap_or(&i32::MAX) {
        came_from.insert(neighbour, current_pos);
        g_score.insert(neighbour, tentative_g_score);
        open_set.push(Node {
          position: neighbour,
          cost: tentative_g_score,
          estimated_cost: tentative_g_score + heuristic(neighbour, end),
        });
      }
    }
  }
  
  // No path found
  None
}