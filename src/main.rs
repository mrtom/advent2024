mod day1;
mod day2;

use day1::Day1;
use day2::Day2;

pub trait AOCDay {
    fn name(&self) -> String;
    fn test_answer_part1(&self) -> String;
    fn test_answer_part2(&self) -> String;

    fn solve_part1(&self, input: &[String]) -> String;
    fn solve_part2(&self, input: &[String]) -> String;
}

fn read_file(file: &str) -> Vec<String> {
    let contents = std::fs::read_to_string(file).expect("Could not read file");
    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    lines
}

fn run_day(day: &impl AOCDay) {
    println!("Running {}", day.name());

    let test_data1 = read_file(&format!("input/{}/test1.txt", day.name()));
    let input_data1 = read_file(&format!("input/{}/part1.txt", day.name()));

    let test1 = day.solve_part1(&test_data1);
    if test1 == day.test_answer_part1() {
        println!("Part 1 Test Passed, attemtping to solve");
        let answer1 = day.solve_part1(&input_data1);
        println!("Part 1: {}", answer1);

        let test_data2 = read_file(&format!("input/{}/test2.txt", day.name()));
        let input_data2 = read_file(&format!("input/{}/part2.txt", day.name()));

        let test2 = day.solve_part2(&test_data2);
        if test2 == day.test_answer_part2() {
            println!("Part 2 Test Passed, attemtping to solve");
            let answer2 = day.solve_part2(&input_data2);
            println!("Part 2: {}", answer2);
        } else {
            println!("Part 2 Test Failed");
        }
    } else {
        println!("Part 1 Test Failed");
    }
}

fn main() {
    let day1 = Day1 {};
    let day2 = Day2 {};

    // run_day(&day1);
    run_day(&day2);
}
