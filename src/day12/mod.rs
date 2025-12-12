use std::array;

const PRESENT_COUNT: usize = 6;

/** A 3x3 area */
struct Present([u8; 3]);

impl Present {
  fn area(&self) -> u16 {
    self.0.iter().map(|val| val.count_ones() as u16).sum()
  }
}

#[derive(Debug)]
struct Region {
  dimensions: [u8; 2],
  counts: [u8; PRESENT_COUNT],
}

impl Region {
  fn area(&self) -> u16 {
    self.dimensions.iter().copied().map(u16::from).product()
  }
}

type Presents = [Present; PRESENT_COUNT];
type Regions = Box<[Region]>;

fn parse_input(input: &str) -> (Presents, Regions) {
  let mut lines = input.lines();

  let presents = array::from_fn(|_| {
    lines.next().unwrap();
    let bytes = array::from_fn(|_| {
      lines
        .next()
        .unwrap()
        .chars()
        .fold(0, |acc, c| acc << 1 | u8::from(c == '#'))
    });
    lines.next().unwrap();
    Present(bytes)
  });

  let regions = lines
    .map(|line| {
      let (size, line) = line.split_once(':').unwrap();

      let mut dimensions_iter = size.split('x').map(|x| x.parse().unwrap());
      let dimensions = array::from_fn(|_| dimensions_iter.next().unwrap());

      let mut counts_iter = line.split_whitespace().map(|x| x.parse().unwrap());
      let counts = array::from_fn(|_| counts_iter.next().unwrap());

      Region { dimensions, counts }
    })
    .collect();

  (presents, regions)
}

pub fn part1(input: &str) -> usize {
  let (presents, regions) = parse_input(input);

  regions
    .into_iter()
    .filter(|region| {
      let total_present_area: u16 = (0..PRESENT_COUNT)
        .map(|i| u16::from(region.counts[i]) * presents[i].area())
        .sum();

      // this is just a feasibility check, but it works!
      total_present_area <= region.area()
    })
    .count()
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT: &str = include_str!("input");

  #[test]
  fn part1_works() {
    assert_eq!(part1(INPUT), 460);
  }
}
