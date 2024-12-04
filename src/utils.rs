use std::string::ToString;

pub fn read_file(file: &str) -> Vec<String> {
  let contents = std::fs::read_to_string(file).expect("Could not read file");
  let lines: Vec<String> = contents.lines().map(ToString::to_string).collect();
  lines
}
