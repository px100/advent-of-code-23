fn main(input: &str) -> (usize, usize) {
  let mut grid = vec![Vec::new(); 256];
  let mut part1_result = 0;
  for word in input.split(',') {
    part1_result += calculate_hash(word);
    let label_end = word
      .bytes()
      .position(|byte| byte == b'-' || byte == b'=')
      .unwrap();
    let label = &word[..label_end];
    let bucket = &mut grid[calculate_hash(label)];
    match (word.as_bytes()[label_end], bucket.iter().position(|&(l, _)| l == label)) {
      (b'=', Some(index)) => bucket[index] = (label, word[label_end + 1..].parse::<usize>().unwrap()),
      (b'=', None) => bucket.push((label, word[label_end + 1..].parse::<usize>().unwrap())),
      (b'-', Some(index)) => { bucket.remove(index); }
      (b'-', None) => {}
      _ => unreachable!(),
    }
  }
  let part2_result = (0..grid.len())
    .flat_map(|b| (0..grid[b].len())
      .map(move |i| (b, i)))
    .map(|(b, i)| (b + 1) * (i + 1) * grid[b][i].1)
    .sum();
  (part1_result, part2_result)
}

fn calculate_hash(word: &str) -> usize {
  word.bytes().fold(0, |acc, byte| (acc + byte) * 17) as usize
}
