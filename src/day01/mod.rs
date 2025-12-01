fn parse_input(input: &str) -> impl Iterator<Item = isize> {
  input.lines().map(|line| {
    let (dir, len) = line.split_at(1);
    let len: isize = len.parse().unwrap();
    match dir {
      "L" => -len,
      "R" => len,
      _ => panic!("invalid direction"),
    }
  })
}

pub fn part1(input: &str) -> usize {
  let mut val = 50;
  let mut count = 0;

  for step in parse_input(input) {
    val += step;
    while val < 0 {
      val += 100;
    }
    val %= 100;
    if val == 0 {
      count += 1;
    }
  }

  count
}

pub fn part2(input: &str) -> usize {
  let mut val = 50;
  let mut count = 0;

  for step in parse_input(input) {
    let sig = step.signum();
    let len = step.abs();

    for _ in 0..len {
      val += sig;
      match val {
        -1 => val = 99,
        100 => val = 0,
        _ => {}
      }
      if val == 0 {
        count += 1;
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
    assert_eq!(part1(INPUT_TEST), 3);
    assert_eq!(part1(INPUT), 980);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 6);
    assert_eq!(part2(INPUT), 5961);
  }
}
