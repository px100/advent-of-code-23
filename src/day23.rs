use std::cmp::max;

use hashbrown::HashMap;

fn main(input: &str) -> (usize, usize) {
  let grid = input
    .split('\n')
    .map(str::as_bytes)
    .collect::<Vec<_>>();
  (solve(&grid, false), solve(&grid, true))
}

fn solve(grid: &[&[u8]], p2: bool) -> usize {
  let mut graph: HashMap<_, Vec<_>> = grid
    .iter()
    .enumerate()
    .flat_map(|(row, r)| {
      r.iter()
        .enumerate()
        .filter(|&(_, &tile)| tile != b'#')
        .flat_map(move |(col, _)| Some(((row, col), gen_neighbors(row, col, grid, p2))))
    })
    .collect();
  while let Some((&(row, col), _)) = graph.iter().find(|(_, neighbors)| neighbors.len() == 2) {
    let neighbors = graph.remove(&(row, col)).unwrap();
    let (row1, col1, d1) = neighbors[0];
    let (row2, col2, d2) = neighbors[1];
    graph.entry((row1, col1))
      .or_default()
      .retain(|&(next_row, next_col, _)| (next_row, next_col) != (row, col));
    graph.entry((row2, col2))
      .or_default()
      .retain(|&(next_row, next_col, _)| (next_row, next_col) != (row, col));
    graph.entry((row1, col1))
      .or_default()
      .push((row2, col2, d1 + d2));
    graph.entry((row2, col2))
      .or_default()
      .push((row1, col1, d1 + d2));
  }
  let mut step_count = 0;
  dfs(&graph, &mut vec![vec![false; grid[0].len()]; grid.len()], (0, 1), 0, &mut step_count);
  step_count
}

fn dfs(graph: &HashMap<(usize, usize), Vec<(usize, usize, usize)>>, seen: &mut Vec<Vec<bool>>, pos: (usize, usize), dist: usize, max_dist: &mut usize) {
  let (current_row, _current_col) = pos;
  if current_row == seen.len() - 1 {
    *max_dist = max(*max_dist, dist);
    return;
  }
  for &(next_row, next_col, d) in graph.get(&pos).unwrap_or(&vec![]) {
    if seen[next_row][next_col] {
      continue;
    }
    seen[next_row][next_col] = true;
    dfs(graph, seen, (next_row, next_col), dist + d, max_dist);
    seen[next_row][next_col] = false;
  }
}

fn gen_neighbors(row: usize, col: usize, grid: &[&[u8]], part2: bool) -> Vec<(usize, usize, usize)> {
  let directions: &[_] = match grid[row][col] {
    _ if part2 => &[(-1, 0), (1, 0), (0, -1), (0, 1)],
    b'.' => &[(-1, 0), (1, 0), (0, -1), (0, 1)],
    b'^' => &[(-1, 0)],
    b'>' => &[(0, 1)],
    b'v' => &[(1, 0)],
    b'<' => &[(0, -1)],
    _ => unreachable!(),
  };
  directions
    .iter()
    .filter_map(|&(dr, dc)| {
      let (new_row, new_col) = ((row as isize + dr) as usize, (col as isize + dc) as usize);
      grid
        .get(new_row)
        .and_then(|r| r.get(new_col).filter(|&&tile| tile != b'#'))
        .map(|_| (new_row, new_col, 1))
    })
    .collect()
}
