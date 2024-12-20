use std::string::ToString;

pub fn read_file(file: &str) -> Vec<String> {
  let contents = std::fs::read_to_string(file).expect("Could not read file");
  let lines: Vec<String> = contents.lines().map(ToString::to_string).collect();
  lines
}

pub fn i32_to_usize(value: i32) -> Option<usize> {
  value.try_into().ok()
}
pub fn i32_to_usize_x(value: i32) -> usize {
  value.try_into().expect("Failed to convert i32 to usize")
}

pub fn i32_to_u32_x(value: i32) -> u32 {
  value.try_into().expect("Failed to convert i32 to u32")
}

pub fn usize_to_i32(value: usize) -> Option<i32> {
  value.try_into().ok()
}
pub fn usize_to_i32_x(value: usize) -> i32 {
 value.try_into().expect("Failed to convert usize to i32")
}

pub fn u32_to_i32(value: u32) -> Option<i32> {
  value.try_into().ok()
}

pub fn isize_to_usize_x(value: isize) -> usize {
  value.try_into().expect("Failed to convert isize to usize")
}