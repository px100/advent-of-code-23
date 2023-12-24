fn main(input: &str) -> (usize, usize) {
  let grid = input
    .split('\n')
    .map(str::as_bytes)
    .collect::<Vec<_>>();
  let part2_result = (0..grid.len())
    .flat_map(|row| [(row, 0, 1), (row, grid[0].len() - 1, 3)])
    .chain((0..grid[0].len())
      .flat_map(|col| [(0, col, 2), (grid.len() - 1, col, 0)]))
    .map(|start| count_tiles(&grid, start))
    .max()
    .unwrap();
  (count_tiles(&grid, (0, 0, 1)), part2_result)
}

fn count_tiles(grid: &[&[u8]], start: (usize, usize, usize)) -> usize {
  let mut is_tile_seen = vec![vec![[false; 4]; grid[0].len()]; grid.len()];
  let mut cur_beams = vec![start];
  while !cur_beams.is_empty() {
    let mut new_beams = Vec::new();
    for (cur_row, cur_col, cur_dir) in cur_beams {
      if cur_row >= grid.len() || cur_col >= grid[0].len() || is_tile_seen[cur_row][cur_col][cur_dir] {
        continue;
      }
      is_tile_seen[cur_row][cur_col][cur_dir] = true;
      match (grid[cur_row][cur_col], cur_dir) {
        (b'/', _) => new_beams.push(step(cur_row, cur_col, [1, 0, 3, 2][cur_dir])),
        (b'\\', _) => new_beams.push(step(cur_row, cur_col, [3, 2, 1, 0][cur_dir])),
        (b'|', 1 | 3) => new_beams.extend([step(cur_row, cur_col, 0), step(cur_row, cur_col, 2)]),
        (b'-', 0 | 2) => new_beams.extend([step(cur_row, cur_col, 1), step(cur_row, cur_col, 3)]),
        _ => new_beams.push(step(cur_row, cur_col, cur_dir)),
      }
    }
    cur_beams = new_beams;
  }
  is_tile_seen
    .iter()
    .flat_map(|row| row)
    .filter(|tile| tile.iter().any(|&is_seen| is_seen))
    .count()
}

fn step(row: usize, col: usize, direction: usize) -> (usize, usize, usize) {
  let deltas = [(-1isize, 0isize), (0, 1), (1, 0), (0, -1)];
  let (delta_row, delta_col) = deltas[direction];
  (
    (row as isize + delta_row) as usize,
    (col as isize + delta_col) as usize,
    direction,
  )
}
