use std::collections::{HashMap, VecDeque};

use microlp::{ComparisonOp, OptimizationDirection, Problem, Variable};

#[derive(Debug)]
struct Machine {
  lights: u16,
  buttons: Box<[u16]>,
  joltages: Box<[u16]>,
}

fn parse_input(input: &str) -> impl Iterator<Item = Machine> {
  input.lines().map(|line| {
    let line = line.strip_prefix('[').expect("Must start with ]");

    let (lights, line) = line.split_once(']').expect("Must contain ]");

    let lights = lights
      .chars()
      .enumerate()
      .filter_map(|(i, c)| (c == '#').then_some(i))
      .fold(0, |acc, i| acc | 1 << i);

    let (buttons, line) = line.split_once('{').expect("Must contain {");

    let buttons = buttons
      .split_whitespace()
      .map(|button| {
        button
          .strip_prefix('(')
          .expect("Must start with (")
          .strip_suffix(')')
          .expect("Must end with )")
          .split(',')
          .map(|s| s.parse::<usize>().expect("Not an integer!"))
          .fold(0, |acc, i| acc | 1 << i)
      })
      .collect();

    let joltages = line
      .strip_suffix('}')
      .expect("Must end with }")
      .split(',')
      .map(|s| s.parse().expect("Not an integer!"))
      .collect();

    Machine {
      lights,
      buttons,
      joltages,
    }
  })
}

pub fn part1(input: &str) -> u16 {
  let machines = parse_input(input);

  machines
    .map(|machine| {
      let mut best = HashMap::new();
      let mut queue = VecDeque::new();

      queue.push_back(machine.lights);
      best.insert(machine.lights, 0);

      while let Some(lights) = queue.pop_front() {
        let clicks = *best.get(&lights).unwrap();

        if lights == 0 {
          return clicks;
        }

        let next_clicks = clicks + 1;

        for button in &machine.buttons {
          let next_lights = lights ^ button;

          match best.entry(next_lights) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
              if *entry.get() < next_clicks {
                continue;
              }
              entry.insert(next_clicks);
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
              entry.insert(next_clicks);
            }
          }

          queue.push_back(next_lights);
        }
      }

      unreachable!()
    })
    .sum()
}

pub fn part2(input: &str) -> u16 {
  let machines = parse_input(input);

  machines
    .map(|machine| {
      let mut problem = Problem::new(OptimizationDirection::Minimize);

      let vars: Box<[Variable]> = (0..machine.buttons.len())
        .map(|_| problem.add_integer_var(1.0, (0, i32::MAX)))
        .collect();

      for (i, joltage) in machine.joltages.iter().enumerate() {
        problem.add_constraint(
          vars.iter().copied().zip(
            machine
              .buttons
              .iter()
              .map(|button| f64::from((button >> i) & 1)),
          ),
          ComparisonOp::Eq,
          f64::from(*joltage),
        );
      }

      let solution = problem.solve().unwrap();

      vars
        .into_iter()
        .map(|var| solution.var_value_rounded(var) as u16)
        .sum::<u16>()
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
    assert_eq!(part1(INPUT_TEST), 7);
    assert_eq!(part1(INPUT), 447);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 33);
    assert_eq!(part2(INPUT), 18_960);
  }
}
