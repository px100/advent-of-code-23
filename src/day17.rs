use std::collections::{BinaryHeap, HashMap};

fn main(input: &str) -> (i64, i64) {
  let grid = input
    .split('\n')
    .map(str::as_bytes)
    .collect::<Vec<_>>();
  (dijkstra(&grid, 1, 3), dijkstra(&grid, 4, 10))
}

fn dijkstra(grid: &[&[u8]], min_step: isize, max_step: isize) -> i64 {
  let mut distances = HashMap::new();
  let mut queue = BinaryHeap::from_iter([(0, (0, 0, (0, 0)))]);
  while let Some((cost, (row, col, direction))) = queue.pop() {
    if (row, col) == (grid.len() - 1, grid[0].len() - 1) {
      return -cost;
    }
    if !(distances.get(&(row, col, direction)).is_some_and(|&c| -cost > c)) {
      let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];
      for (dr, dc) in dir {
        if direction != (dr, dc) && direction != (-dr, -dc) {
          let mut next_cost = -cost;
          for dist in 1..=max_step {
            let next_row = (row as isize + dr * dist) as usize;
            let next_col = (col as isize + dc * dist) as usize;
            if next_row < grid.len() && next_col < grid[0].len() {
              next_cost += (grid[next_row][next_col] - b'0') as i64;
              if dist >= min_step {
                let key = (next_row, next_col, (dr, dc));
                if next_cost < *distances.get(&key).unwrap_or(&i64::MAX) {
                  distances.insert(key, next_cost);
                  queue.push((-next_cost, key));
                }
              }
            }
          }
        }
      }
    }
  }
  unreachable!()
}
