use std::string::ToString;

pub fn read_file(file: &str) -> Vec<String> {
  let contents = std::fs::read_to_string(file).expect("Could not read file");
  let lines: Vec<String> = contents.lines().map(ToString::to_string).collect();
  lines
}

#[allow(clippy::cast_sign_loss)]
pub fn isize_to_usize(value: isize) -> Option<usize> {
  if value >= 0 {
      Some(value as usize)
  } else {
      None
  }
}

#[allow(clippy::cast_possible_wrap, clippy::checked_conversions)]
pub fn usize_to_isize(value: usize) -> Option<isize> {
  if value <= isize::MAX as usize {
      Some(value as isize)
  } else {
      None
  }
}