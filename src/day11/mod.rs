use std::{array, collections::HashMap};

type Node<'a> = &'a str;

fn parse_input(
  input: &str,
) -> impl Iterator<Item = (Node<'_>, Box<[Node<'_>]>)> {
  input.lines().map(|line| {
    let (parent, children) = line.split_once(':').unwrap();
    (parent, children.split_whitespace().collect())
  })
}

fn count_paths<'a>(
  cache: &mut HashMap<[Node<'a>; 2], u64>,
  nodes: &HashMap<Node<'a>, Box<[Node<'a>]>>,
  node_pair: [Node<'a>; 2],
) -> u64 {
  let [start_node, end_node] = node_pair;

  if start_node == end_node {
    return 1;
  }

  if let Some(cached_count) = cache.get(&node_pair) {
    return *cached_count;
  }

  let mut count = 0;

  if let Some(next_start_nodes) = nodes.get(start_node) {
    for next_start_node in next_start_nodes {
      count += count_paths(cache, nodes, [next_start_node, end_node]);
    }
  }

  cache.insert(node_pair, count);

  count
}

fn count_paths_through<'a>(
  cache: &mut HashMap<[Node<'a>; 2], u64>,
  nodes: &HashMap<Node<'a>, Box<[Node<'a>]>>,
  routes: &[Node<'a>],
) -> u64 {
  routes
    .windows(2)
    .map(|pair| count_paths(cache, nodes, array::from_fn(|i| pair[i])))
    .product()
}

pub fn part1(input: &str) -> u64 {
  let nodes = parse_input(input).collect();

  let mut cache = HashMap::new();

  count_paths(&mut cache, &nodes, ["you", "out"])
}

pub fn part2(input: &str) -> u64 {
  let nodes = parse_input(input).collect();

  let mut cache = HashMap::new();

  count_paths_through(&mut cache, &nodes, &["svr", "fft", "dac", "out"])
    + count_paths_through(&mut cache, &nodes, &["svr", "dac", "fft", "out"])
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT_TEST_0: &str = include_str!("input_test_0");
  const INPUT_TEST_1: &str = include_str!("input_test_1");
  const INPUT: &str = include_str!("input");

  #[test]
  fn part1_works() {
    assert_eq!(part1(INPUT_TEST_0), 5);
    assert_eq!(part1(INPUT), 428);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST_1), 2);
    assert_eq!(part2(INPUT), 331_468_292_364_745);
  }
}
