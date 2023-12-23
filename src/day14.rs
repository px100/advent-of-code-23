use hashbrown::HashMap;

fn main(input: &str) -> (usize, usize) {
  let mut current_map = input
    .lines()
    .map(|l| l.as_bytes().to_vec())
    .collect::<Vec<_>>();
  let p1 = {
    let mut map = current_map.clone();
    roll_north(&mut map);
    load_map(&map)
  };
  let mut seen = HashMap::new();
  for i in 1..1000000000 {
    for _ in 0..4 {
      roll_north(&mut current_map);
      current_map = (0..current_map[0].len())
        .map(|c| (0..current_map.len())
          .map(|r| current_map[r][c]).rev()
          .collect())
        .collect();
    }
    match seen.insert(current_map.clone(), i) {
      Some(seen_at) => {
        if (1000000000 - i) % (i - seen_at) == 0 {
          break;
        }
      }
      None => {}
    }
  }
  (p1, load_map(&current_map))
}

fn load_map(map: &Vec<Vec<u8>>) -> usize {
  (0..map.len())
    .map(|r| (0..map[0].len())
      .filter(|&c| map[r][c] == b'O')
      .map(|_| map.len() - r)
      .sum::<usize>())
    .sum()
}

fn roll_north(map: &mut Vec<Vec<u8>>) {
  let mut done;
  while {
    done = true;
    for r in 0..map.len() - 1 {
      for c in 0..map[0].len() {
        if map[r + 1][c] == b'O' && map[r][c] == b'.' {
          map[r][c] = b'O';
          map[r + 1][c] = b'.';
          done = false;
        }
      }
    }
    !done
  } {}
}
