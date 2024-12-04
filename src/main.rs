mod utils;
mod day1;
mod day2;
mod day3;
mod day4;

use std::path::Path;
use utils::read_file;

// use day1::Day1;
// use day2::Day2;
// use day3::Day3;
use day4::Day4;

pub trait AOCDay {
  fn name(&self) -> String;
  fn test_answer_part1(&self) -> String;
  fn test_answer_part2(&self) -> String;
  
  fn solve_part1(&self, input: &[String]) -> String;
  fn solve_part2(&self, input: &[String]) -> String;
}

fn run_day(day: &impl AOCDay) {
  println!("Running {}", day.name());
  
  let td1_name = &format!("input/{}/test1.txt", day.name());
  let id1_name = &format!("input/{}/part1.txt", day.name());
  let td2_name = &format!("input/{}/test2.txt", day.name());
  let id2_name = &format!("input/{}/part2.txt", day.name());
  
  let test_data1 = read_file(td1_name);
  let input_data1 = read_file(id1_name);
  
  let test1 = day.solve_part1(&test_data1);
  if test1 == day.test_answer_part1() {
    println!("Part 1 Test Passed, attemtping to solve");
    let answer1 = day.solve_part1(&input_data1);
    println!("Part 1: {answer1}");
    
    let test_data_name = if Path::new(td2_name).exists() { td2_name } else { td1_name };
    let input_data_name = if Path::new(id2_name).exists() { id2_name } else { id1_name };
    
    let test_data2 = read_file(test_data_name);
    let input_data2 = read_file(input_data_name);
    
    let test2 = day.solve_part2(&test_data2);
    if test2 == day.test_answer_part2() {
      println!("Part 2 Test Passed, attemtping to solve");
      let answer2 = day.solve_part2(&input_data2);
      println!("Part 2: {answer2}");
    } else {
      println!("Part 2 Test Failed");
    }
  } else {
    println!("Part 1 Test Failed");
  }
}

fn main() {
  // let day1 = Day1 {};
  // let day2 = Day2 {};
  // let day3 = Day3 {};
  let day4 = Day4 {};
  
  // run_day(&day1);
  // run_day(&day2);
  // run_day(&day3);
  run_day(&day4);
}
