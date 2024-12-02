use crate::AOCDay;

pub struct Day2 {}

fn string_to_ints(s: &str) -> Vec<i32> {
  return s.split(' ').map(|t| t.parse::<i32>().unwrap()).collect();
}

fn is_small_gaps(v: &Vec<i32>) -> bool {
  for i in 0..v.len() - 1 {
    let diff = (v[i + 1] - v[i]).abs();
    if diff > 3 || diff == 0 {
      return false;
    }
  }

  return true;
}

fn is_single_direction(v: &Vec<i32>) -> bool {
  let mut asc = v.clone();
  asc.sort();

  let mut desc = v.clone();
  desc.sort_by(|a, b| b.cmp(a));

  *v == asc || *v == desc
}

fn does_pass(v: &Vec<i32>) -> bool {
  return is_small_gaps(v) && is_single_direction(v);
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

  
  return false;
}

impl AOCDay for Day2 {
  fn name(&self) -> String {
    "day2".to_string()
  }
  
  fn test_answer_part1(&self) -> String {
    return "2".to_string();
  }
  
  fn test_answer_part2(&self) -> String {
    return "4".to_string();
  }
  
  fn solve_part1(&self, input: &[String]) -> String {
    let decoded = input.iter()
      .map(|s| string_to_ints(s)).collect::<Vec<Vec<i32>>>();
    let passing_count = decoded.iter()
      .filter(|v| does_pass(v)).count();
    return passing_count.to_string();
  }
  
  fn solve_part2(&self, input: &[String]) -> String {
    let decoded = input.iter()
      .map(|s| string_to_ints(s)).collect::<Vec<Vec<i32>>>();
    let passing_count = decoded.iter()
      .filter(|v| does_pass_part_2(v)).count();
    return passing_count.to_string();
  }
}
