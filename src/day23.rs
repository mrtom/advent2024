use petgraph::{graph::NodeIndex, graph::UnGraph, Graph};
use regex::Regex;
use std::collections::HashSet;

use crate::AOCDay;

const PART_1_EXAMPLE: &str = "7";
const PART_2_EXAMPLE: &str = "co,de,ka,ta";

fn parse_input(input: &[String]) -> Graph<&str, (), petgraph::Undirected> {
  let regex = Regex::new(r"^([a-z]{2})-([a-z]{2})$").unwrap();
  let mut computers = UnGraph::<&str, ()>::with_capacity(input.len() * 2, 4);

  for line in input {
    let Some(captures) = regex.captures(line) else {
      panic!("Failed to parse line");
    };
    let first_name = captures.get(1).expect("Failed to parse first").as_str();
    let second_name = captures.get(2).expect("Failed to parse second").as_str();

    let first = match computers
      .node_indices()
      .find(|i| computers[*i] == first_name)
    {
      Some(i) => i,
      None => computers.add_node(first_name),
    };

    let second = match computers
      .node_indices()
      .find(|i| computers[*i] == second_name)
    {
      Some(i) => i,
      None => computers.add_node(second_name),
    };

    computers.add_edge(first, second, ());
  }

  computers
}

fn group_key(group: &[NodeIndex], graph: &Graph<&str, (), petgraph::Undirected>) -> String {
  let mut keys: Vec<&str> = group.iter().map(|i| graph[*i]).collect::<Vec<&str>>();
  keys.sort_unstable();
  keys.join(",")
}

fn intersect<'a>(a: &'a [NodeIndex], b: &'a [NodeIndex]) -> Vec<NodeIndex> {
  a.iter().filter(|&x| b.contains(x)).copied().collect()
}

fn bron_kerbosch(
  graph: &Graph<&str, (), petgraph::Undirected>,
  largest_subgraph_key: String,
  current: &[NodeIndex],
  candidates: &mut Vec<NodeIndex>,
  processed: &mut Vec<NodeIndex>,
) -> String {
  let mut existing_key = largest_subgraph_key;

  if candidates.is_empty() && processed.is_empty() {
    let key = group_key(current, graph);
    if key.len() > existing_key.len() {
      return key;
    }
    return existing_key;
  }

  while !candidates.is_empty() {
    let node = candidates[0];
    let mut new_current_set: HashSet<NodeIndex> = current.iter().copied().collect();
    new_current_set.insert(node);
    let new_current = new_current_set.into_iter().collect::<Vec<NodeIndex>>();

    let neighbours = graph.neighbors(node).collect::<Vec<_>>();
    let mut new_candidates = intersect(candidates, &neighbours);
    let mut new_processed = intersect(processed, &neighbours);

    let candidate_key = bron_kerbosch(
      graph,
      existing_key.clone(),
      &new_current,
      &mut new_candidates,
      &mut new_processed,
    );

    if candidate_key.len() > existing_key.len() {
      existing_key = candidate_key;
    }

    candidates.remove(0);
    processed.push(node);
  }

  existing_key
}

pub struct Day23 {}

impl AOCDay for Day23 {
  fn name(&self) -> String {
    "day23".to_string()
  }

  fn test_answer_part1(&self) -> String {
    PART_1_EXAMPLE.to_string()
  }

  fn test_answer_part2(&self) -> String {
    PART_2_EXAMPLE.to_string()
  }

  fn solve_part1(&self, input: &[String]) -> String {
    let computers = parse_input(input);
    let mut triplets = HashSet::new();

    computers.node_indices().for_each(|node| {
      let neighbours = computers.neighbors(node).collect::<Vec<_>>();
      for neighbour in neighbours {
        let second_neighbours = computers.neighbors(neighbour).collect::<Vec<_>>();
        for second_neighbour in second_neighbours {
          if second_neighbour != node && computers.contains_edge(node, second_neighbour) {
            triplets.insert(group_key(&[node, neighbour, second_neighbour], &computers));
          }
        }
      }
    });

    let contains_t = Regex::new(r"^t|(,t)").expect("Failed to create regex");
    let with_t = triplets
      .iter()
      .filter(|triplet| contains_t.is_match(triplet))
      .collect::<Vec<&String>>();

    with_t.len().to_string()
  }

  fn solve_part2(&self, input: &[String]) -> String {
    let computers = parse_input(input);
    let mut all = computers.node_indices().collect::<Vec<NodeIndex>>();
    bron_kerbosch(&computers, String::new(), &[], &mut all, &mut vec![])
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::read_file;

  #[test]
  fn test_part_1_example() {
    let day = Day23 {};
    assert_eq!(
      PART_1_EXAMPLE,
      day.solve_part1(&read_file("input/day23/test1.txt"))
    );
  }

  #[test]
  fn test_part_1() {
    let day = Day23 {};
    assert_eq!("1512", day.solve_part1(&read_file("input/day23/part1.txt")));
  }

  #[test]
  fn test_part_2_example() {
    let day = Day23 {};
    assert_eq!(
      PART_2_EXAMPLE,
      day.solve_part2(&read_file("input/day23/test1.txt"))
    );
  }

  #[test]
  fn test_part_2() {
    let day = Day23 {};
    assert_eq!(
      "ac,ed,fh,kd,lf,mb,om,pe,qt,uo,uy,vr,wg",
      day.solve_part2(&read_file("input/day23/part1.txt"))
    );
  }
}
