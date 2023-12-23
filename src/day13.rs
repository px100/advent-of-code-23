fn main(input: &str) -> (usize, usize) {
  let grids = input
    .split("\n\n")
    .map(|s| s.split('\n')
      .map(|line| line.as_bytes())
      .collect::<Vec<_>>())
    .collect::<Vec<_>>();
  (solve(&grids, 0), solve(&grids, 1))
}

fn solve(grids: &[Vec<&[u8]>], limit: usize) -> usize {
  grids.iter().map(|grid|
    find_row(grid, limit).map(|r| (r + 1) * 100)
      .or_else(|| find_col(grid, limit).map(|c| c + 1))
      .unwrap()
  ).sum()
}

fn find_row(grid: &[&[u8]], limit: usize) -> Option<usize> {
  (0..grid.len() - 1).find(|&row| {
    (0..=row.min(grid.len() - row - 2))
      .map(|dr| grid[row - dr]
        .iter()
        .zip(grid[row + 1 + dr].iter())
        .filter(|&(c1, c2)| c1 != c2)
        .count())
      .sum::<usize>() == limit
  })
}

fn find_col(grid: &[&[u8]], limit: usize) -> Option<usize> {
  (0..grid[0].len() - 1).find(|&col| {
    (0..=col.min(grid[0].len() - col - 2))
      .map(|dc| grid
        .iter()
        .filter(|&&row| row[col - dc] != row[col + 1 + dc])
        .count())
      .sum::<usize>() == limit
  })
}
