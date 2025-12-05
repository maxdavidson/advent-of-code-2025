use std::ops::RangeInclusive;

fn parse_input(input: &str) -> (Box<[RangeInclusive<u64>]>, Box<[u64]>) {
  let mut ranges = Vec::<RangeInclusive<_>>::new();
  let mut ids = Vec::<_>::new();

  for line in input.lines() {
    if let Some((left, right)) = line.split_once('-') {
      ranges.push(left.parse().unwrap()..=right.parse().unwrap());
    } else if !line.is_empty() {
      ids.push(line.parse().unwrap());
    }
  }

  (ranges.into(), ids.into())
}

pub fn part1(input: &str) -> usize {
  let (ranges, ids) = parse_input(input);

  ids
    .into_iter()
    .filter(|id| ranges.iter().any(|range| range.contains(id)))
    .count()
}

pub fn part2(input: &str) -> u64 {
  let (mut ranges, _) = parse_input(input);

  ranges.sort_by_key(|range| *range.start());

  let mut merged_ranges = Vec::<RangeInclusive<u64>>::new();

  for range in ranges {
    if let Some(last) = merged_ranges.last_mut()
      && *last.end() >= *range.start() - 1
    {
      *last = *last.start()..=(*last.end()).max(*range.end());
    } else {
      merged_ranges.push(range);
    }
  }

  merged_ranges
    .into_iter()
    .map(|range| range.end() - range.start() + 1)
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT_TEST: &str = include_str!("input_test");
  const INPUT: &str = include_str!("input");

  #[test]
  fn part1_works() {
    assert_eq!(part1(INPUT_TEST), 3);
    assert_eq!(part1(INPUT), 770);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 14);
    assert_eq!(part2(INPUT), 357_674_099_117_260);
  }
}
