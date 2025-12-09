use std::{array, cmp::Reverse};

fn parse_input(input: &str) -> impl Iterator<Item = Point> {
  input.lines().map(|line| {
    let mut it = line.split(',');
    array::from_fn(move |_| it.next().unwrap().parse().unwrap())
  })
}

fn area(&[x_a, y_a]: &Point, &[x_b, y_b]: &Point) -> u64 {
  (x_a.abs_diff(x_b) + 1) * (y_a.abs_diff(y_b) + 1)
}

type Point = [u64; 2];
type Line<'a> = [&'a Point; 2];

trait CircularWindows<T> {
  fn circular_windows<'a, const N: usize>(
    &'a self,
  ) -> impl Iterator<Item = [&'a T; N]>
  where
    T: 'a;
}

impl<T> CircularWindows<T> for [T] {
  fn circular_windows<'a, const N: usize>(
    &'a self,
  ) -> impl Iterator<Item = [&'a T; N]>
  where
    T: 'a,
  {
    (0..self.len()).map(|i| array::from_fn(|n| &self[(i + n) % self.len()]))
  }
}

pub fn part1(input: &str) -> u64 {
  let points: Vec<Point> = parse_input(input).collect();

  points
    .iter()
    .enumerate()
    .flat_map(|(i, a)| points[i + 1..].iter().map(move |b| (a, b)))
    .map(|(a, b)| area(a, b))
    .max()
    .unwrap()
}

pub fn part2(input: &str) -> u64 {
  let points: Vec<Point> = parse_input(input).collect();

  let lines: Vec<Line> = points.circular_windows().collect();

  let mut pairs: Vec<_> = points
    .iter()
    .enumerate()
    .flat_map(|(i, a)| points[i + 1..].iter().map(move |b| (a, b, area(a, b))))
    .collect();

  pairs.sort_unstable_by_key(|(_, _, a)| Reverse(*a));

  pairs
    .into_iter()
    .find(|(p1, p2, _)| {
      lines.iter().all(|[l1, l2]| {
        p1[0].max(p2[0]) <= l1[0].min(l2[0])
          || p1[0].min(p2[0]) >= l1[0].max(l2[0])
          || p1[1].max(p2[1]) <= l1[1].min(l2[1])
          || p1[1].min(p2[1]) >= l1[1].max(l2[1])
      })
    })
    .map(|(_, _, a)| a)
    .unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT_TEST: &str = include_str!("input_test");
  const INPUT: &str = include_str!("input");

  #[test]
  fn part1_works() {
    assert_eq!(part1(INPUT_TEST), 50);
    assert_eq!(part1(INPUT), 4_777_824_480);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 24);
    assert_eq!(part2(INPUT), 1_542_119_040);
  }
}
