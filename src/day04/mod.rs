fn parse_input(input: &str) -> (Box<[u8]>, usize, usize) {
  let mut lines = input.lines().peekable();
  let width = lines.peek().unwrap().len();
  let bytes = lines
    .flat_map(|line| line.bytes())
    .collect::<Vec<_>>()
    .into_boxed_slice();
  let height = bytes.len() / width;
  (bytes, width, height)
}

fn neighbors(
  index: usize,
  width: usize,
  height: usize,
) -> impl Iterator<Item = usize> {
  const NEIGHBORS: [[isize; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
  ];

  let x = (index % width).cast_signed();
  let y = (index / width).cast_signed();

  NEIGHBORS.into_iter().filter_map(move |[dx, dy]| {
    let next_x = x + dx;
    if 0 <= next_x {
      let next_y = y + dy;
      if 0 <= next_y {
        let next_x = next_x.cast_unsigned();
        if next_x < width {
          let next_y = next_y.cast_unsigned();
          if next_y < height {
            let next_index = width * next_y + next_x;
            return Some(next_index);
          }
        }
      }
    }

    None
  })
}

const PAPER: u8 = b'@';

pub fn part1(input: &str) -> usize {
  let (bytes, width, height) = parse_input(input);

  let mut accessible_papers_count = 0;

  for (index, &byte) in bytes.iter().enumerate() {
    if byte == PAPER {
      let mut paper_neighbor_count = 0;

      for next_index in neighbors(index, width, height) {
        let next_byte = bytes[next_index];
        if next_byte == PAPER {
          paper_neighbor_count += 1;
        }
      }

      if paper_neighbor_count < 4 {
        accessible_papers_count += 1;
      }
    }
  }

  accessible_papers_count
}

pub fn part2(input: &str) -> usize {
  let (mut bytes, width, height) = parse_input(input);

  let mut total_removed_papers_count = 0;

  let mut papers_to_remove = Vec::with_capacity(bytes.len());

  loop {
    for (index, &byte) in bytes.iter().enumerate() {
      if byte == PAPER {
        let mut paper_neighbors_count = 0;

        for next_index in neighbors(index, width, height) {
          let next_byte = bytes[next_index];
          if next_byte == PAPER {
            paper_neighbors_count += 1;
          }
        }

        if paper_neighbors_count < 4 {
          papers_to_remove.push(index);
        }
      }
    }

    if papers_to_remove.is_empty() {
      break;
    }

    while let Some(next_index) = papers_to_remove.pop() {
      total_removed_papers_count += 1;
      bytes[next_index] = b'.';
    }
  }

  total_removed_papers_count
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT_TEST: &str = include_str!("input_test");
  const INPUT: &str = include_str!("input");

  #[test]
  fn part1_works() {
    assert_eq!(part1(INPUT_TEST), 13);
    assert_eq!(part1(INPUT), 1460);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 43);
    assert_eq!(part2(INPUT), 9243);
  }
}
