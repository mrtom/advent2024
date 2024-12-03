use regex::Regex;

use crate::AOCDay;

pub struct Day3 {}

fn parse_line(line: &str) -> i32 {
  let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
  let result: i32 = re.captures_iter(line)
    .map(|caps| {
        let first = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let second = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        first * second
    })
    .sum();

  return result;
}

fn parse_line_p2(line: &str) -> i32 {
  let re = Regex::new(r"mul\((\d+),(\d+)\)|(don)'t\(\)|(do)\(\)").unwrap();

  let mut result = 0;
  let mut should_multiply = true;

  for caps in re.captures_iter(line) {
      let first = caps.get(1).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
      let second = caps.get(2).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
      let is_dont = caps.get(3).map_or(false, |m| m.as_str().len() > 0);
      let is_do = caps.get(4).map_or(false, |m| m.as_str().len() > 0);

      if is_dont {
        should_multiply = false;
      } else if is_do {
        should_multiply = true;
      } else if should_multiply {
          result += first * second;
      }
  }

  return result;
}

impl AOCDay for Day3 {
  fn name(&self) -> String {
    "day3".to_string()
  }
  
  fn test_answer_part1(&self) -> String {
    return "161".to_string();
  }
  
  fn test_answer_part2(&self) -> String {
    return "48".to_string();
  }
  
  fn solve_part1(&self, input: &[String]) -> String {
    return input.iter()
      .map(|line| parse_line(line))
      .sum::<i32>()
      .to_string();
  }
  
  fn solve_part2(&self, input: &[String]) -> String {
    let input_single_line = input.join("");
    return parse_line_p2(&input_single_line).to_string();
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
          161, 
          parse_line("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
        );
    }

    #[test]
    fn test_parse_line_p2() {
      assert_eq!(
        48,
        parse_line_p2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))")
      )
    }
}