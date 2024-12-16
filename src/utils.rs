use std::string::ToString;

pub mod a_star;

pub fn read_file(file: &str) -> Vec<String> {
  let contents = std::fs::read_to_string(file).expect("Could not read file");
  let lines: Vec<String> = contents.lines().map(ToString::to_string).collect();
  lines
}

pub fn i32_to_usize(value: i32) -> Option<usize> {
  value.try_into().ok()
}

pub fn usize_to_i32(value: usize) -> Option<i32> {
  value.try_into().ok()
}