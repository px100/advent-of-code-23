use hashbrown::HashMap;
use itertools::Itertools;

fn main(input: &str) -> (usize, usize) {
  let mut cache = HashMap::new();
  let (p1, p2) = input
    .lines()
    .map(|line| {
      let (vents, nums_str) = line
        .split_once(' ')
        .unwrap();
      let nums = nums_str
        .split(',')
        .map(|w| w.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
      let new_vents = (0..5)
        .map(|_| vents)
        .join("?");
      let new_nums = (0..5)
        .flat_map(|_| &nums)
        .copied()
        .collect::<Vec<_>>();
      cache.clear();
      let ways_p1 = ways(&mut cache, vents.as_bytes(), None, &nums);
      cache.clear();
      let ways_p2 = ways(&mut cache, new_vents.as_bytes(), None, &new_nums);
      (ways_p1, ways_p2)
    })
    .fold((0, 0), |(acc_p1, acc_p2), (current_p1, current_p2)| (acc_p1 + current_p1, acc_p2 + current_p2));
  (p1, p2)
}

fn ways(cache: &mut HashMap<(usize, usize, usize), usize>, s: &[u8], within: Option<usize>, remaining: &[usize]) -> usize {
  if !(s.is_empty()) {
    if !within.is_some() || !remaining.is_empty() {
      let key = (s.len(), within.unwrap_or(0), remaining.len());
      match cache.get(&key) {
        Some(&x) => {
          return x;
        }
        None => {}
      }
      let ways = get_ways(cache, &s, within, &remaining);
      cache.insert(key, ways);
      return ways;
    }
    return 0;
  }
  match (within, remaining.len()) {
    (None, 0) => 1,
    (Some(x), 1) if x == remaining[0] => 1,
    _ => 0
  }
}

fn get_ways(cache: &mut HashMap<(usize, usize, usize), usize>, s: &&[u8], within: Option<usize>, remaining: &&[usize]) -> usize {
  match (s[0], within) {
    (b'.', Some(x)) if x != remaining[0] => 0,
    (b'.', Some(_)) => ways(cache, &s[1..], None, &remaining[1..]),
    (b'.', None) => ways(cache, &s[1..], None, remaining),
    (b'#', Some(_)) => ways(cache, &s[1..], within.map(|x| x + 1), remaining),
    (b'#', None) => ways(cache, &s[1..], Some(1), remaining),
    (b'?', Some(x)) => {
      let mut ans = ways(cache, &s[1..], within.map(|x| x + 1), remaining);
      if x == remaining[0] {
        ans += ways(cache, &s[1..], None, &remaining[1..])
      }
      ans
    }
    (b'?', None) => ways(cache, &s[1..], Some(1), remaining) + ways(cache, &s[1..], None, remaining),
    _ => unreachable!(),
  }
}
