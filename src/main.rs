mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

use std::path::Path;
use utils::read_file;

// use day1::Day1;
// use day2::Day2;
// use day3::Day3;
// use day4::Day4;
// use day5::Day5;
// use day6::Day6;
// use day7::Day7;
// use day8::Day8;
use day9::Day9;

pub trait AOCDay {
  fn name(&self) -> String;
  fn test_answer_part1(&self) -> String;
  fn test_answer_part2(&self) -> String;

  fn solve_part1(&self, input: &[String]) -> String;
  fn solve_part2(&self, input: &[String]) -> String;
}

fn run_day(day: &impl AOCDay) {
  println!("Running {}", day.name());

  let test_data_1_name = &format!("input/{}/test1.txt", day.name());
  let input_1_name = &format!("input/{}/part1.txt", day.name());
  let test_data_2_name = &format!("input/{}/test2.txt", day.name());
  let input_2_name = &format!("input/{}/part2.txt", day.name());

  let test_data1 = read_file(test_data_1_name);
  let input_data1 = read_file(input_1_name);

  let test1 = day.solve_part1(&test_data1);
  if test1 == day.test_answer_part1() {
    println!("Part 1 Test Passed, attemtping to solve");
    let start_part_1 = std::time::Instant::now();
    let answer1 = day.solve_part1(&input_data1);
    println!("Part 1: {answer1}");
    println!("{:?}", start_part_1.elapsed());

    let test_data_name = if Path::new(test_data_2_name).exists() {
      test_data_2_name
    } else {
      test_data_1_name
    };
    let input_data_name = if Path::new(input_2_name).exists() {
      input_2_name
    } else {
      input_1_name
    };

    let test_data2 = read_file(test_data_name);
    let input_data2 = read_file(input_data_name);

    let test2 = day.solve_part2(&test_data2);
    if test2 == day.test_answer_part2() {
      println!("Part 2 Test Passed, attemtping to solve");
      let start_part_2 = std::time::Instant::now();
      let answer2 = day.solve_part2(&input_data2);
      println!("Part 2: {answer2}");
      println!("{:?}", start_part_2.elapsed());
    } else {
      println!("Part 2 Test Failed");
    }
  } else {
    println!("Part 1 Test Failed");
  }
}

fn main() {
  let start = std::time::Instant::now();

  // let day1 = Day1 {};
  // let day2 = Day2 {};
  // let day3 = Day3 {};
  // let day4 = Day4 {};
  // let day5 = Day5 {};
  // let day6 = Day6 {};
  // let day7 = Day7 {};
  // let day8 = Day8 {};
  let day9 = Day9 {};

  // run_day(&day1);
  // run_day(&day2);
  // run_day(&day3);
  // run_day(&day4);
  // run_day(&day5);
  // run_day(&day6);
  // run_day(&day7);
  // run_day(&day8);
  run_day(&day9);

  println!("{:?}", start.elapsed());
}
