use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Node {
  Empty,
  Beam,
  Splitter,
}

fn parse_input(input: &str) -> Box<[Box<[Node]>]> {
  input
    .lines()
    .map(|line| {
      line
        .chars()
        .map(|c| match c {
          '.' => Node::Empty,
          'S' => Node::Beam,
          '^' => Node::Splitter,
          _ => unreachable!("Invalid byte"),
        })
        .collect::<Vec<_>>()
        .into_boxed_slice()
    })
    .collect::<Vec<_>>()
    .into_boxed_slice()
}

pub fn part1(input: &str) -> usize {
  let mut map = parse_input(input);

  let mut split_count = 0;

  for y in 1..map.len() {
    let (before, after) = map.split_at_mut(y);

    let prev = before.last().unwrap();
    let next = after.first_mut().unwrap();

    for x in 0..prev.len() {
      if matches!(prev[x], Node::Beam) {
        match next[x] {
          Node::Splitter => {
            split_count += 1;
            if 0 < x {
              next[x - 1] = Node::Beam;
            }
            if x + 1 < next.len() {
              next[x + 1] = Node::Beam;
            }
          }
          _ => {
            next[x] = Node::Beam;
          }
        }
      }
    }
  }

  split_count
}

pub fn part2(input: &str) -> u64 {
  type Pos = (usize, usize);

  fn timelines(
    cache: &mut HashMap<Pos, u64>,
    map: &[Box<[Node]>],
    pos: Pos,
  ) -> u64 {
    if let Some(&count) = cache.get(&pos) {
      return count;
    }

    let mut count = 0;

    let (x, y) = pos;

    for y in y..map.len() {
      let row = &map[y];
      if matches!(row[x], Node::Splitter) {
        count += 1;

        if 0 < x {
          count += timelines(cache, map, (x - 1, y));
        }

        if x + 1 < row.len() {
          count += timelines(cache, map, (x + 1, y));
        }

        break;
      }
    }

    cache.insert(pos, count);

    count
  }

  let map = parse_input(input);

  let start_pos = map
    .iter()
    .enumerate()
    .find_map(|(y, row)| {
      row
        .iter()
        .enumerate()
        .find_map(|(x, node)| matches!(node, Node::Beam).then_some((x, y)))
    })
    .unwrap();

  let mut cache = HashMap::new();

  timelines(&mut cache, &map, start_pos) + 1
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT_TEST: &str = include_str!("input_test");
  const INPUT: &str = include_str!("input");

  #[test]
  fn part1_works() {
    assert_eq!(part1(INPUT_TEST), 21);
    assert_eq!(part1(INPUT), 1537);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 40);
    assert_eq!(part2(INPUT), 18_818_811_755_665);
  }
}
