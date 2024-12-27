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

use std::path::Path;

// use day1::Day1;
// use day2::Day2;
// use day3::Day3;
// use day4::Day4;
// use day5::Day5;
// use day6::Day6;
// use day7::Day7;
// use day8::Day8;
// use day9::Day9;
// use day10::Day10;
// use day11::Day11;
// use day12::Day12;
// use day13::Day13;
// use day14::Day14;
// use day15::Day15;
// use day16::Day16;
// use day17::Day17;
// use day18::Day18;
// use day19::Day19;
// use day20::Day20;
use day21::Day21;
// use day22::Day22;
// use day23::Day23;
// use day24::Day24;
// use day25::Day25;

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

  let test_data1 = utils::read_file(test_data_1_name);
  let input_data1 = utils::read_file(input_1_name);

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

    let test_data2 = utils::read_file(test_data_name);
    let input_data2 = utils::read_file(input_data_name);

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
  // let day9 = Day9 {};
  // let day10 = Day10 {};
  // let day11 = Day11 {};
  // let day12 = Day12 {};
  // let day13 = Day13 {};
  // let day14 = Day14 {};
  // let day15 = Day15 {};
  // let day16 = Day16 {};
  // let day17 = Day17 {};
  // let day18 = Day18 {};
  // let day19 = Day19 {};
  // let day20 = Day20 {};
  let day21 = Day21 {};
  // let day22 = Day22 {};
  // let day23 = Day23 {};
  // let day24 = Day24 {};
  // let day25 = Day25 {};

  // run_day(&day1);
  // run_day(&day2);
  // run_day(&day3);
  // run_day(&day4);
  // run_day(&day5);
  // run_day(&day6);
  // run_day(&day7);
  // run_day(&day8);
  // run_day(&day9);
  // run_day(&day10);
  // run_day(&day11);
  // run_day(&day12);
  // run_day(&day13);
  // run_day(&day14);
  // run_day(&day15);
  // run_day(&day16);
  // run_day(&day17);
  // run_day(&day18);
  // run_day(&day19);
  // run_day(&day20);
  run_day(&day21);
  // run_day(&day22);
  // run_day(&day23);
  // run_day(&day24);
  // run_day(&day25);

  println!("{:?}", start.elapsed());
}
