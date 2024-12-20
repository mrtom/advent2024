use std::fmt;
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
  let v = value.try_into();

  if v.ok().is_none() {
    println!("Failed to convert i32 to usize: {value}");
  }

  v.expect("Failed to convert i32 to usize")
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
pub fn u32_to_i32_x(value: u32) -> i32 {
  value.try_into().expect("Failed to convert u32 to i32")
}

pub fn isize_to_usize_x(value: isize) -> usize {
  value.try_into().expect("Failed to convert isize to usize")
}

// Mark - Points

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

impl fmt::Display for Point {
  // This trait requires `fmt` with this exact signature.
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // Write strictly the first element into the supplied output
    // stream: `f`. Returns `fmt::Result` which indicates whether the
    // operation succeeded or failed. Note that `write!` uses syntax which
    // is very similar to `println!`.
    write!(f, "{},{}", self.x, self.y)
  }
}

pub fn get_manhattan_distance(a: Point, b: Point) -> i32 {
  u32_to_i32_x(a.x.abs_diff(b.x) + a.y.abs_diff(b.y))
}
