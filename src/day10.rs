use hashbrown::HashSet;

fn main(input: &str) -> (usize, usize) {
  let mut starting_position = (0, 0);
  let mut grid = input
    .split('\n')
    .enumerate()
    .map(|(row, line)| {
      line.bytes()
        .enumerate()
        .map(|(col, tile)| {
          if tile == b'S' {
            starting_position = (row, col);
          }
          find_connection(tile)
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();
  let pipe_loop = "J|-L7F"
    .bytes()
    .find_map(|start_tile| {
      grid[starting_position.0][starting_position.1] = find_connection(start_tile);
      find_loop(&grid, starting_position)
    })
    .unwrap();
  let mut p2 = 0;
  for row in 0..grid.len() {
    let mut inside = false;
    for col in 0..grid[0].len() {
      if !pipe_loop.contains(&(row, col)) {
        p2 += inside as usize;
      } else if grid[row][col][0] {
        inside = !inside;
      }
    }
  }
  (pipe_loop.len() / 2, p2)
}

fn find_loop(graph: &[Vec<[bool; 4]>], start: (usize, usize)) -> Option<HashSet<(usize, usize)>> {
  let (mut row, mut col) = start;
  let mut direction = graph[row][col]
    .iter()
    .position(|&d| d)
    .unwrap();
  let mut seen = HashSet::new();
  loop {
    if !seen.insert((row, col)) {
      break Some(seen);
    }
    let came_from = match direction {
      0 => {
        row -= 1;
        2
      }
      1 => {
        col += 1;
        3
      }
      2 => {
        row += 1;
        0
      }
      3 => {
        col -= 1;
        1
      }
      _ => unreachable!(),
    };
    if graph[row][col][came_from] {
      direction = (0..4).find(|&i| i != came_from && graph[row][col][i]).unwrap();
    }
  }
}

fn find_connection(tile: u8) -> [bool; 4] {
  if let b'|' = tile {
    [true, false, true, false]
  } else if let b'-' = tile {
    [false, true, false, true]
  } else if let b'L' = tile {
    [true, true, false, false]
  } else if let b'J' = tile {
    [true, false, false, true]
  } else if let b'7' = tile {
    [false, false, true, true]
  } else if let b'F' = tile {
    [false, true, true, false]
  } else {
    [false, false, false, false]
  }
}
