use regex::Regex;

use crate::AOCDay;

pub struct DayN {}

impl AOCDay for DayN {
  fn name(&self) -> String {
    "dayN".to_string()
  }
  
  fn test_answer_part1(&self) -> String {
    panic!("Not implemented");
    "FAIL".to_string()
  }
  
  fn test_answer_part2(&self) -> String {
    panic!("Not implemented");
    "FAIL".to_string()
  }
  
  fn solve_part1(&self, input: &[String]) -> String {
    panic!("Not implemented");
    "".to_string()
  }
  
  fn solve_part2(&self, input: &[String]) -> String {
    panic!("Not implemented");
    "".to_string()
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_file;

    #[test]
    fn test_something() {
        let result = "";
        assert_eq!(result, "");
    }

    #[test]
  fn test_part_1() {
    let day = DayN {};
    assert_eq!(
      "TODO",
      day.solve_part1(&read_file("input/dayN/part1.txt"))
    );
  }

  #[test]
  fn test_part_2() {
    let day = DayN {};
    assert_eq!(
      "TODO",
      day.solve_part2(&read_file("input/dayN/part1.txt"))
    );
  }
}