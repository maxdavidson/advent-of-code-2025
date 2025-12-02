use std::ops::RangeInclusive;

fn parse_input(input: &str) -> impl Iterator<Item = RangeInclusive<u64>> {
  input.split(',').map(|range| {
    let (start, end) = range.trim().split_once('-').unwrap();
    let start = start.parse().unwrap();
    let end = end.parse().unwrap();
    start..=end
  })
}

fn is_invalid_id(mut id: u64, reps: u32) -> bool {
  let digits = 1 + id.ilog10();
  if !digits.is_multiple_of(reps) {
    return false;
  }

  let quot = 10u64.pow(digits / reps);

  for _ in 0..reps - 1 {
    let lo = id % quot;
    id /= quot;
    let hi = id % quot;
    if hi != lo {
      return false;
    }
  }

  true
}

pub fn part1(input: &str) -> u64 {
  let mut count = 0;

  for range in parse_input(input) {
    for id in range {
      if is_invalid_id(id, 2) {
        count += id;
      }
    }
  }

  count
}

pub fn part2(input: &str) -> u64 {
  let mut count = 0;

  for range in parse_input(input) {
    for id in range {
      if (2..8).any(|reps| is_invalid_id(id, reps)) {
        count += id;
      }
    }
  }

  count
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT_TEST: &str = include_str!("input_test");
  const INPUT: &str = include_str!("input");

  #[test]
  fn part1_works() {
    assert_eq!(part1(INPUT_TEST), 1_227_775_554);
    assert_eq!(part1(INPUT), 41_294_979_841);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 4_174_379_265);
    assert_eq!(part2(INPUT), 66_500_947_346);
  }
}
