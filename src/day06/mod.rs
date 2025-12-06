use std::ops::Range;

#[derive(Debug)]
enum Operator {
  Add,
  Multiply,
}

pub fn part1(input: &str) -> u64 {
  let mut lines = input.lines();

  let operators: Vec<_> = lines
    .next_back()
    .unwrap()
    .split_whitespace()
    .map(|s| match s {
      "+" => Operator::Add,
      "*" => Operator::Multiply,
      _ => panic!("Unknown operator"),
    })
    .collect();

  let rows: Vec<_> = lines
    .flat_map(|line| line.split_whitespace().map(|s| s.parse().unwrap()))
    .collect();

  operators
    .iter()
    .enumerate()
    .map(|(i, op)| {
      let col_values = rows.iter().skip(i).step_by(operators.len());

      match op {
        Operator::Add => col_values.sum::<u64>(),
        Operator::Multiply => col_values.product::<u64>(),
      }
    })
    .sum()
}

pub fn part2(input: &str) -> u64 {
  let mut lines = input.lines();

  let operator_ranges = {
    let operators = lines.next_back().unwrap().as_bytes();

    let mut operator_it = operators.iter().enumerate();
    let mut current_group: Option<(Operator, usize)> = None;

    std::iter::from_fn::<(Operator, Range<usize>), _>(move || {
      for (i, c) in operator_it.by_ref() {
        let next_op = match c {
          b'+' => Operator::Add,
          b'*' => Operator::Multiply,
          _ => continue,
        };

        if let Some((op, i_start)) = current_group.replace((next_op, i)) {
          return Some((op, i_start..i - 1));
        }
      }

      if let Some((op, i_start)) = current_group.take() {
        return Some((op, i_start..operators.len()));
      }

      None
    })
  };

  let rows: Vec<Vec<_>> = lines
    .map(|line| line.chars().map(|c| c.to_digit(10)).collect())
    .collect();

  operator_ranges
    .map(|(op, range)| {
      let values = range.map(|i_group| {
        rows
          .iter()
          .filter_map(|row| row[i_group])
          .fold(0u64, |acc, val| 10 * acc + u64::from(val))
      });

      match op {
        Operator::Add => values.sum::<u64>(),
        Operator::Multiply => values.product::<u64>(),
      }
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT_TEST: &str = include_str!("input_test");
  const INPUT: &str = include_str!("input");

  #[test]
  fn part1_works() {
    assert_eq!(part1(INPUT_TEST), 4_277_556);
    assert_eq!(part1(INPUT), 6_371_789_547_734);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 3_263_827);
    assert_eq!(part2(INPUT), 11_419_862_653_216);
  }
}
