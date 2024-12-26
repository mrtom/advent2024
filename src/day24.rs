use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::string::ToString;

use crate::AOCDay;

const PART_1_EXAMPLE: &str = "2024";
const PART_2_EXAMPLE: &str = "SKIP";

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum GateType {
  Input,
  And,
  Or,
  Xor,
}

impl GateType {
  fn from_str(s: &str) -> Self {
    match s {
      "AND" => GateType::And,
      "OR" => GateType::Or,
      "XOR" => GateType::Xor,
      _ => panic!("Invalid gate type"),
    }
  }
}

#[allow(clippy::struct_field_names)]
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Gate {
  name: String,
  gate_type: GateType,
  input_1: Option<String>,
  input_2: Option<String>,
  output: Option<bool>,
}

fn get_or_create_gate<'a>(
  name: &str,
  gate_type: GateType,
  gates: &'a mut HashMap<String, Gate>,
) -> &'a mut Gate {
  let gate = gates.entry(name.to_string()).or_insert_with(|| Gate {
    name: name.to_string(),
    gate_type,
    input_1: None,
    input_2: None,
    output: None,
  });

  gate
}

fn parse_input(input: &[String]) -> (HashMap<String, Gate>, VecDeque<String>) {
  let mut gates = HashMap::<String, Gate>::new();
  let mut queue = VecDeque::<String>::new();

  let empty_line_idx = input
    .iter()
    .enumerate()
    .find(|(_, line)| line.is_empty())
    .expect("Expected an empty line splitting the two parts of the input")
    .0;

  let (inputs, edges_) = input.split_at(empty_line_idx);
  let edges = edges_.split_first().unwrap().1;

  let input_regex = Regex::new(r"^([a-z,0-9]{3}): (\d)$").expect("Invalid input regex");
  for input in inputs {
    let Some(captures) = input_regex.captures(input) else {
      panic!("Failed to parse input");
    };
    let gate_name = captures.get(1).expect("Failed to parse name").as_str();
    let gate_output = captures
      .get(2)
      .expect("Failed to parse output")
      .as_str()
      .parse::<usize>()
      .expect("Failed to parse output to integer")
      == 1;

    gates.entry(gate_name.to_string()).or_insert(Gate {
      name: gate_name.to_string(),
      gate_type: GateType::Input,
      input_1: None,
      input_2: None,
      output: Some(gate_output),
    });
  }

  let edge_regex = Regex::new(r"^([a-z,0-9]{3}) (AND|OR|XOR) ([a-z,0-9]{3}) -> ([a-z,0-9]{3})$")
    .expect("Invalid edge regex");
  for edge in edges {
    let Some(captures) = edge_regex.captures(edge) else {
      panic!("Failed to parse edge");
    };
    let first = captures
      .get(1)
      .expect("Failed to parse first gate")
      .as_str();
    let second = captures
      .get(3)
      .expect("Failed to parse second gate")
      .as_str();
    let third = captures
      .get(4)
      .expect("Failed to parse third gate")
      .as_str();

    let gate_type =
      GateType::from_str(captures.get(2).expect("Failed to parse gate type").as_str());

    let third_gate = get_or_create_gate(third, gate_type, &mut gates);
    third_gate.input_1 = Some(first.to_string());
    third_gate.input_2 = Some(second.to_string());
    queue.push_back(third.to_string());
  }

  (gates, queue)
}

fn run_device(mut gates: HashMap<String, Gate>, mut queue: VecDeque<String>) -> (u64, u64, u64) {
  while !queue.is_empty() {
    let immut_gates = gates.clone();
    if let Some(cur_gate_name) = queue.pop_front() {
      let cur_gate = gates.get_mut(&cur_gate_name).expect("Failed to find gate");

      if let (Some(input_1_name), Some(input_2_name)) =
        (cur_gate.input_1.clone(), cur_gate.input_2.clone())
      {
        let input_1 = immut_gates
          .get(&input_1_name)
          .expect("Failed to find input 1");
        let input_2 = immut_gates
          .get(&input_2_name)
          .expect("Failed to find input 2");

        if let (Some(output_1), Some(output_2)) = (input_1.output, input_2.output) {
          let output = match cur_gate.gate_type {
            GateType::And => output_1 & output_2,
            GateType::Or => output_1 | output_2,
            GateType::Xor => output_1 ^ output_2,
            GateType::Input => panic!("Cannot have input gate in queue"),
          };
          cur_gate.output = Some(output);
        } else {
          // Inputs not ready yet, push back to the end of the queue
          queue.push_back(cur_gate_name);
        }
      } else {
        panic!("Invalid gate - all non-INPUT gates should have inputs");
      }
    }
  }

  let mut x_result = 0_u64;
  let xxx_regex = Regex::new(r"^x[0-9]{2}$").expect("Failed to create XXX regex");
  gates
    .iter()
    .filter(|(gate_name, _)| xxx_regex.is_match(gate_name))
    .sorted_by(|(a, _), (b, _)| b.cmp(a))
    .for_each(|(_, gate)| {
      x_result = x_result * 2 + u64::from(gate.output.unwrap());
    });

  let mut y_result = 0_u64;
  let yxx_regex = Regex::new(r"^y[0-9]{2}$").expect("Failed to create YXX regex");
  gates
    .iter()
    .filter(|(gate_name, _)| yxx_regex.is_match(gate_name))
    .sorted_by(|(a, _), (b, _)| b.cmp(a))
    .for_each(|(_, gate)| {
      y_result = y_result * 2 + u64::from(gate.output.unwrap());
    });

  let mut z_result = 0_u64;
  let zxx_regex = Regex::new(r"^z[0-9]{2}$").expect("Failed to create ZXX regex");
  gates
    .iter()
    .filter(|(gate_name, _)| zxx_regex.is_match(gate_name))
    .sorted_by(|(a, _), (b, _)| b.cmp(a))
    .for_each(|(_, gate)| {
      z_result = z_result * 2 + u64::from(gate.output.unwrap());
    });
  (x_result, y_result, z_result)
}

fn is_input_wire(wire: Option<String>) -> bool {
  match wire {
    Some(wire) => wire.starts_with('x') || wire.starts_with('y'),
    None => false,
  }
}

fn are_inputs_first_bit(gate: &Gate) -> bool {
  match (gate.input_1.as_ref(), gate.input_2.as_ref()) {
    (Some(input_1), Some(input_2)) => input_1.ends_with("00") && input_2.ends_with("00"),
    _ => false,
  }
}

pub struct Day24 {}

impl AOCDay for Day24 {
  fn name(&self) -> String {
    "day24".to_string()
  }

  fn test_answer_part1(&self) -> String {
    PART_1_EXAMPLE.to_string()
  }

  fn test_answer_part2(&self) -> String {
    PART_2_EXAMPLE.to_string()
  }

  fn solve_part1(&self, input: &[String]) -> String {
    let (gates, queue) = parse_input(input);
    let (_, _, z_result) = run_device(gates, queue);
    z_result.to_string()
  }

  fn solve_part2(&self, input: &[String]) -> String {
    let (gates, _) = parse_input(input);
    let is_test = input.len() == 19;
    if is_test {
      // The test for this part demonstrates an circuit that performs a bitwise AND operation
      // Whereas the problem wants us to find an adder. So it's not very useful.
      return "SKIP".to_string();
    }

    let mut faulty_gates = HashSet::new();
    let final_z_gate_name = &gates
      .values()
      .filter(|gate| gate.name.starts_with('z'))
      .sorted_by(|a, b| b.name.cmp(&a.name))
      .next()
      .unwrap()
      .name;

    for (gate_name, gate) in &gates {
      if gate.name.starts_with('z') && gate.name != *final_z_gate_name {
        if gate.gate_type != GateType::Xor {
          faulty_gates.insert(gate_name);
        }
      } else if !gate.name.starts_with('z')
        && !is_input_wire(gate.input_1.clone())
        && !is_input_wire(gate.input_2.clone())
      {
        if gate.gate_type == GateType::Xor {
          faulty_gates.insert(gate_name);
        }
      } else if is_input_wire(gate.input_1.clone())
        && is_input_wire(gate.input_2.clone())
        && !are_inputs_first_bit(gate)
      {
        let name = &gate.name;
        let expected_next_type = if gate.gate_type == GateType::Xor {
          GateType::Xor
        } else {
          GateType::Or
        };

        let feeds_into_expected_gate = &gates.values().any(|other_gate| {
          other_gate.name != *name
            && (other_gate.input_1 == Some(name.clone())
              || other_gate.input_2 == Some(name.clone()))
            && other_gate.gate_type == expected_next_type
        });

        if !feeds_into_expected_gate {
          faulty_gates.insert(gate_name);
        }
      }
    }

    faulty_gates.iter().sorted().join(",")
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::read_file;

  #[test]
  fn test_part_1_example() {
    let day = Day24 {};
    assert_eq!(
      PART_1_EXAMPLE,
      day.solve_part1(&read_file("input/day24/test1.txt"))
    );
  }

  #[test]
  fn test_part_1() {
    let day = Day24 {};
    assert_eq!(
      "55920211035878",
      day.solve_part1(&read_file("input/day24/part1.txt"))
    );
  }

  #[test]
  fn test_part_2() {
    let day = Day24 {};
    assert_eq!(
      "btb,cmv,mwp,rdg,rmj,z17,z23,z30",
      day.solve_part2(&read_file("input/day24/part1.txt"))
    );
  }
}
