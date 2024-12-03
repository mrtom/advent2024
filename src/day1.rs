use regex::Regex;

use crate::AOCDay;

pub struct Day1 {}

fn parse_input(input: &[String]) -> (Vec<i32>, Vec<i32>) {
  let mut first: Vec<i32> = Vec::new();
  let mut second: Vec<i32> = Vec::new();
  
  for line in input {
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    let caps = re.captures(line).unwrap();
    
    let left = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let right = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
    
    first.push(left);
    second.push(right);
  }
  
  (first, second)
}

impl AOCDay for Day1 {
  fn name(&self) -> String {
    "day1".to_string()
  }
  
  fn test_answer_part1(&self) -> String {
    "11".to_string()
  }
  
  fn test_answer_part2(&self) -> String {
    "31".to_string()
  }
  
  fn solve_part1(&self, input: &[String]) -> String {
    let (mut first, mut second) = parse_input(input);
    
    first.sort();
    second.sort();
    
    let result: i32 = first
    .iter()
    .enumerate()
    .map(|(i, &x)| (second[i] - x).abs())
    .sum();
    
    result.to_string()
  }
  
  fn solve_part2(&self, input: &[String]) -> String {
    let (first, second): (Vec<i32>, Vec<i32>) = parse_input(input);
    
    let result: i32 = first
    .iter()
    .map(|&x| {
      let count = second.iter().filter(|&y| *y == x).count() as i32;
      x * count
    })
    .sum();
    
    result.to_string()
  }
}
