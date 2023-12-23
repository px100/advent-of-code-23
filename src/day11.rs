use itertools::Itertools;

fn main(input: &str) -> (usize, usize) {
  let universe = input
    .split('\n')
    .map(|line| line.as_bytes())
    .collect::<Vec<_>>();
  let galaxies = (0..universe.len())
    .cartesian_product(0..universe[0].len())
    .filter(|&(r, c)| universe[r][c] == b'#')
    .collect::<Vec<_>>();
  (solve(&universe, galaxies.clone(), 2), solve(&universe, galaxies, 1000000))
}

fn solve(universe: &[&[u8]], mut galaxies: Vec<(usize, usize)>, size: usize) -> usize {
  let (rows, cols) = (universe.len(), universe[0].len());
  let empty_rows = (0..rows).filter(|&r| universe[r].iter().all(|&cell| cell == b'.'));
  let empty_cols = (0..cols).filter(|&c| (0..rows).all(|r| universe[r][c] == b'.'));
  for empty_row in empty_rows.rev() {
    for galaxy in &mut galaxies {
      if galaxy.0 > empty_row {
        galaxy.0 += size - 1;
      }
    }
  }
  for empty_col in empty_cols.rev() {
    for galaxy in &mut galaxies {
      if galaxy.1 > empty_col {
        galaxy.1 += size - 1;
      }
    }
  }
  let mut total_distance = 0;
  for (i, &(r1, c1)) in galaxies.iter().enumerate() {
    for &(r2, c2) in &galaxies[i + 1..] {
      total_distance += r1.abs_diff(r2) + c1.abs_diff(c2);
    }
  }
  total_distance
}
