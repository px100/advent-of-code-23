use hashbrown::HashMap;

fn main(input: &str) -> (usize, usize) {
  let mut map = input
    .lines()
    .map(|l| l.as_bytes().to_vec())
    .collect::<Vec<_>>();
  let mut seen = HashMap::new();
  for i in 1..1000000000 {
    for _ in 0..4 {
      roll_map(&mut map);
      map = (0..map[0].len())
        .map(|c| (0..map.len())
          .map(|r| map[r][c]).rev()
          .collect())
        .collect();
    }
    match seen.insert(map.clone(), i) {
      Some(x) => {
        if (1000000000 - i) % (i - x) == 0 {
          break;
        }
      }
      None => {}
    }
  }
  (solve_p1(&mut map), load_map(&map))
}

fn solve_p1(map: &mut Vec<Vec<u8>>) -> usize {
  load_map(&{
    let mut map_clone = map.clone();
    roll_map(&mut map_clone);
    map_clone
  })
}

fn load_map(map: &Vec<Vec<u8>>) -> usize {
  (0..map.len())
    .map(|r| (0..map[0].len())
      .filter(|&c| map[r][c] == b'O')
      .map(|_| map.len() - r)
      .sum::<usize>())
    .sum()
}

fn roll_map(map: &mut Vec<Vec<u8>>) {
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
