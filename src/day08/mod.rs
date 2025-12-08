use std::cmp::Reverse;

use bit_set::BitSet;

type Point = [u64; 3];

fn parse_input(input: &str) -> impl Iterator<Item = Point> {
  input.lines().map(|line| {
    let mut it = line.split(',');
    std::array::from_fn(move |_| it.next().unwrap().parse().unwrap())
  })
}

fn dist_squared([x_a, y_a, z_a]: Point, [x_b, y_b, z_b]: Point) -> u64 {
  x_a.abs_diff(x_b).pow(2) + y_a.abs_diff(y_b).pow(2) + z_a.abs_diff(z_b).pow(2)
}

pub fn part1(input: &str, truncate_count: usize) -> usize {
  let points: Vec<Point> = parse_input(input).collect();

  let mut pairs = (0..points.len())
    .flat_map(|i| (i + 1..points.len()).map(move |j| (i, j)))
    .collect::<Vec<_>>();

  pairs.select_nth_unstable_by_key(truncate_count, |(a, b)| {
    dist_squared(points[*a], points[*b])
  });
  pairs.truncate(truncate_count);

  let mut points_to_groups: Vec<_> = (0..points.len()).collect();
  let mut groups_to_points: Vec<_> = (0..points.len())
    .map(|n| {
      let mut set = BitSet::new();
      set.insert(n);
      set
    })
    .collect();

  for (point_id_a, point_id_b) in pairs {
    let group_id_a = points_to_groups[point_id_a];
    let group_id_b = points_to_groups[point_id_b];

    if group_id_a != group_id_b {
      let group_b = std::mem::take(&mut groups_to_points[group_id_b]);

      for point_id in &group_b {
        points_to_groups[point_id] = group_id_a;
      }

      groups_to_points[group_id_a].union_with(&group_b);
    }
  }

  let mut groups = groups_to_points
    .into_iter()
    .map(|group| group.len())
    .collect::<Vec<_>>();

  groups.select_nth_unstable_by_key(3, |key| Reverse(*key));

  groups.into_iter().take(3).product()
}

pub fn part2(input: &str) -> u64 {
  let points: Vec<Point> = parse_input(input).collect();

  let mut pairs = (0..points.len())
    .flat_map(|i| (i + 1..points.len()).map(move |j| (i, j)))
    .collect::<Vec<_>>();

  pairs.sort_unstable_by_key(|(a, b)| dist_squared(points[*a], points[*b]));

  let mut points_to_groups: Vec<_> = (0..points.len()).collect();
  let mut groups_to_points: Vec<_> = (0..points.len())
    .map(|n| {
      let mut set = BitSet::new();
      set.insert(n);
      set
    })
    .collect();

  let mut group_len = points.len();

  for (point_id_a, point_id_b) in pairs {
    let group_id_a = points_to_groups[point_id_a];
    let group_id_b = points_to_groups[point_id_b];

    if group_id_a != group_id_b {
      let group_b = std::mem::take(&mut groups_to_points[group_id_b]);

      for point_id in &group_b {
        points_to_groups[point_id] = group_id_a;
      }

      groups_to_points[group_id_a].union_with(&group_b);

      group_len -= 1;
    }

    if group_len == 1 {
      let x_a = points[point_id_a][0];
      let x_b = points[point_id_b][0];
      return x_a * x_b;
    }
  }

  unreachable!();
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT_TEST: &str = include_str!("input_test");
  const INPUT: &str = include_str!("input");

  #[test]
  fn part1_works() {
    assert_eq!(part1(INPUT_TEST, 10), 40);
    assert_eq!(part1(INPUT, 1000), 96_672);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 25_272);
    assert_eq!(part2(INPUT), 22_517_595);
  }
}
