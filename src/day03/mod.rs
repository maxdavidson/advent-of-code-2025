use std::collections::VecDeque;

fn parse_input(input: &str) -> impl Iterator<Item = Box<[u64]>> {
  input.lines().map(|line| {
    line
      .chars()
      .map(|char| char.to_digit(10).unwrap().into())
      .collect::<Vec<_>>()
      .into_boxed_slice()
  })
}

fn find_best_value(digits: &[u64], initial_depth: u32) -> u64 {
  struct Entry {
    value: u64,
    index: usize,
    depth: u32,
  }

  let mut best_value = 0u64;

  let mut queue = VecDeque::<Entry>::new();

  queue.push_back(Entry {
    value: 0,
    index: 0,
    depth: initial_depth - 1,
  });

  while let Some(Entry {
    value,
    index,
    depth,
  }) = queue.pop_front()
  {
    let level = 10u64.pow(depth);

    for next_index in index..(digits.len() - depth as usize) {
      let next_value = value + level * digits[next_index];

      if next_value / level < best_value / level {
        continue;
      }

      if best_value < next_value {
        best_value = next_value;
      }

      if depth != 0 {
        queue.push_back(Entry {
          value: next_value,
          index: next_index + 1,
          depth: depth - 1,
        });
      }
    }
  }

  best_value
}

pub fn part1(input: &str) -> u64 {
  parse_input(input)
    .map(|digits| find_best_value(&digits, 2))
    .sum()
}

pub fn part2(input: &str) -> u64 {
  parse_input(input)
    .map(|digits| find_best_value(&digits, 12))
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT_TEST: &str = include_str!("input_test");
  const INPUT: &str = include_str!("input");

  #[test]
  fn part1_works() {
    assert_eq!(part1(INPUT_TEST), 357);
    assert_eq!(part1(INPUT), 16_812);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 3_121_910_778_619);
    assert_eq!(part2(INPUT), 166_345_822_896_410);
  }
}
