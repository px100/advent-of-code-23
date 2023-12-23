use hashbrown::HashMap;

fn main(input: &str) -> (usize, usize) {
  let (path, rest) = input.split_once("\n\n").unwrap();
  let graph: HashMap<&[u8], (&[u8], &[u8])> = rest
    .lines()
    .map(|line| {
      let bytes = line.as_bytes();
      (
        &bytes[0..3],
        (&bytes[7..10], &bytes[12..15]),
      )
    })
    .collect();
  let part1_result = count_steps(path.as_bytes(), &graph, b"AAA", b"ZZZ");
  let part2_result = graph.keys()
    .filter(|key| key.ends_with(b"A"))
    .map(|node| count_steps(path.as_bytes(), &graph, node, b"Z"))
    .fold(1, |acc, steps| acc * steps / gcd(steps, acc));
  (part1_result, part2_result)
}

fn count_steps<'a>(path: &[u8], graph: &HashMap<&[u8], (&'a [u8], &'a [u8])>, mut current_node: &'a [u8], goal: &[u8]) -> usize {
  path.iter().cycle().position(|&direction| {
    current_node = match direction {
      b'L' => graph[current_node].0,
      _ => graph[current_node].1,
    };
    current_node.ends_with(goal)
  }).map_or(0, |pos| pos + 1)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
  while b != 0 {
    let temp = b;
    b = a % b;
    a = temp;
  }
  a
}
