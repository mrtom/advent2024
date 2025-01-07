use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

use std::env;
use std::path::Path;

use day1::Day1;
use day10::Day10;
use day11::Day11;
use day12::Day12;
use day13::Day13;
use day14::Day14;
use day15::Day15;
use day16::Day16;
use day17::Day17;
use day18::Day18;
use day19::Day19;
use day2::Day2;
use day20::Day20;
use day21::Day21;
use day22::Day22;
use day23::Day23;
use day24::Day24;
use day25::Day25;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;
use day9::Day9;

pub trait AOCDay: Sync {
  fn name(&self) -> String;
  fn test_answer_part1(&self) -> String;
  fn test_answer_part2(&self) -> String;

  fn solve_part1(&self, input: &[String]) -> String;
  fn solve_part2(&self, input: &[String]) -> String;
}

fn get_filenames(day_name: &str) -> (String, String, String, String) {
  let test_data_1_name = &format!("input/{day_name}/test1.txt");
  let input_1_name = &format!("input/{day_name}/part1.txt");
  let test_data_2_name = &format!("input/{day_name}/test2.txt");
  let input_2_name = &format!("input/{day_name}/part2.txt");

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

  (
    test_data_1_name.to_string(),
    input_1_name.to_string(),
    test_data_name.to_string(),
    input_data_name.to_string(),
  )
}

fn run_day(day: &dyn AOCDay) {
  println!("Running {}", day.name());
  let (_, input_1_name, _, input_2_name) = get_filenames(&day.name());
  let input_data1 = utils::read_file(&input_1_name);
  let input_data2 = utils::read_file(&input_2_name);

  let start_part_1 = std::time::Instant::now();
  let answer1 = day.solve_part1(&input_data1);
  println!("{} Part 1: {answer1}", day.name());
  println!("{:?}", start_part_1.elapsed());

  let start_part_2 = std::time::Instant::now();
  let answer2 = day.solve_part2(&input_data2);
  println!("{} Part 2: {answer2}", day.name());
  println!("{:?}", start_part_2.elapsed());
}

fn check_day(day: &dyn AOCDay) {
  println!("Running {}", day.name());
  let (test_data_1_name, input_1_name, test_data_2_name, input_2_name) = get_filenames(&day.name());
  let test_data1 = utils::read_file(&test_data_1_name);
  let input_data1 = utils::read_file(&input_1_name);
  let test_data2 = utils::read_file(&test_data_2_name);
  let input_data2 = utils::read_file(&input_2_name);

  let test1 = day.solve_part1(&test_data1);
  if test1 == day.test_answer_part1() {
    println!("{} Part 1 Test Passed, attemtping to solve", day.name());
    let start_part_1 = std::time::Instant::now();
    let answer1 = day.solve_part1(&input_data1);
    println!("{} Part 1: {answer1}", day.name());
    println!("{:?}", start_part_1.elapsed());

    let test2 = day.solve_part2(&test_data2);
    if test2 == day.test_answer_part2() {
      println!("{} Part 2 Test Passed, attemtping to solve", day.name());
      let start_part_2 = std::time::Instant::now();
      let answer2 = day.solve_part2(&input_data2);
      println!("{} Part 2: {answer2}", day.name());
      println!("{:?}", start_part_2.elapsed());
    } else {
      println!("{} Part 2 Test Failed", day.name());
    }
  } else {
    println!("{} Part 1 Test Failed", day.name());
  }
}

#[allow(clippy::similar_names)]
fn main() {
  let args: Vec<String> = env::args().collect();
  for arg in &args {
    println!("{arg}");
  }

  let start = std::time::Instant::now();

  let day1 = Day1 {};
  let day2 = Day2 {};
  let day3 = Day3 {};
  let day4 = Day4 {};
  let day5 = Day5 {};
  let day6 = Day6 {};
  let day7 = Day7 {};
  let day8 = Day8 {};
  let day9 = Day9 {};
  let day10 = Day10 {};
  let day11 = Day11 {};
  let day12 = Day12 {};
  let day13 = Day13 {};
  let day14 = Day14 {};
  let day15 = Day15 {};
  let day16 = Day16 {};
  let day17 = Day17 {};
  let day18 = Day18 {};
  let day19 = Day19 {};
  let day20 = Day20 {};
  let day21 = Day21 {};
  let day22 = Day22 {};
  let day23 = Day23 {};
  let day24 = Day24 {};
  let day25 = Day25 {};

  let days: Vec<Box<dyn AOCDay>> = vec![
    // Box::new(day1),
    // Box::new(day2),
    // Box::new(day3),
    // Box::new(day4),
    // Box::new(day5),
    // Box::new(day6),
    // Box::new(day7),
    // Box::new(day8),
    // Box::new(day9),
    // Box::new(day10),
    // Box::new(day11),
    // Box::new(day12),
    // Box::new(day13),
    // Box::new(day14),
    // Box::new(day15),
    // Box::new(day16),
    // Box::new(day17),
    // Box::new(day18),
    // Box::new(day19),
    Box::new(day20),
    // Box::new(day21),
    // Box::new(day22),
    // Box::new(day23),
    // Box::new(day24),
    // Box::new(day25),
  ];

  if args.len() > 1 && args[1] == "run" {
    days.par_iter().for_each(|day| run_day(&**day));
  } else {
    days.par_iter().for_each(|day| check_day(&**day));
  }

  println!("{:?}", start.elapsed());
}
