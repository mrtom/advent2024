use crate::AOCDay;
pub struct Day2 {}

fn string_to_ints(s: &str) -> Vec<i32> {
  s.split(' ').map(|t| t.parse::<i32>().unwrap()).collect()
}

fn is_small_gaps(v: &[i32]) -> bool {
  for i in 0..v.len() - 1 {
    let diff = (v[i + 1] - v[i]).abs();
    if diff > 3 || diff == 0 {
      return false;
    }
  }
  
  true
}

fn is_single_direction(v: &Vec<i32>) -> bool {
  let mut asc = v.clone();
  asc.sort_unstable();
  
  let mut desc = v.clone();
  desc.sort_by(|a, b| b.cmp(a));
  
  *v == asc || *v == desc
}

fn does_pass(v: &Vec<i32>) -> bool {
  is_small_gaps(v) && is_single_direction(v)
}

fn does_pass_part_2(v: &Vec<i32>) -> bool {
  // First, check the full array
  let does_pass = is_small_gaps(v) && is_single_direction(v);
  
  if does_pass { return true; }
  
  // Now we need to explode the input by removing element one at a time.
  // We pass if any one of them passes.
  for i in 0..v.len() {
    let mut copy = v.clone();
    copy.remove(i);
    if is_small_gaps(&copy) && is_single_direction(&copy) {
      return true;
    }
  }
  
  
  false
}

impl AOCDay for Day2 {
  fn name(&self) -> String {
    "day2".to_string()
  }
  
  fn test_answer_part1(&self) -> String {
    "2".to_string()
  }
  
  fn test_answer_part2(&self) -> String {
    "4".to_string()
  }
  
  fn solve_part1(&self, input: &[String]) -> String {
    let decoded = input.iter()
    .map(|s| string_to_ints(s)).collect::<Vec<Vec<i32>>>();
    let passing_count = decoded.iter()
    .filter(|v| does_pass(v)).count();
    passing_count.to_string()
  }
  
  fn solve_part2(&self, input: &[String]) -> String {
    let decoded = input.iter()
    .map(|s| string_to_ints(s)).collect::<Vec<Vec<i32>>>();
    let passing_count = decoded.iter()
    .filter(|v| does_pass_part_2(v)).count();
    passing_count.to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::read_file;
  
  #[test]
  fn test_string_to_ints() {
    let result = string_to_ints("7 6 4 2 1");
    assert_eq!(result, vec![7, 6, 4, 2, 1]);
  }
  
  #[test]
  fn test_is_small_gaps() {
    let result = is_small_gaps(&[1, 2, 3, 4, 5]);
    assert!(result);
  }
  
  #[test]
  fn test_is_large_gap() {
    let result = is_small_gaps(&[1, 2, 6, 7]);
    assert!(!result);
  }
  
  #[test]
  fn test_is_no_gap() {
    let result = is_small_gaps(&[1, 2, 3, 3]);
    assert!(!result);
  }
  
  #[test]
  fn test_part_1() {
    let day = Day2 {};
    assert_eq!(
      "502",
      day.solve_part1(&read_file("input/day2/part1.txt"))
    );
  }
  
  #[test]
  fn test_part_2() {
    let day = Day2 {};
    assert_eq!(
      "544",
      day.solve_part2(&read_file("input/day2/part2.txt"))
    );
  }
}