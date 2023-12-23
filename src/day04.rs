fn main(input: &str) -> (usize, usize) {
  let mut part1_score = 0;
  let mut copies = vec![1; input.lines().count()];
  for (i, line) in input.lines().enumerate() {
    let (wanted, got) = parse_input_line(line);
    let won = calculate_score(&wanted, &got);
    part1_score += if won != 0 { 1 << (won - 1) } else { 0 };
    for j in 0..won {
      copies[i + j + 1] += copies[i];
    }
  }
  let part2_score = copies.iter().sum();
  (part1_score, part2_score)
}

pub(crate) fn parse_input_line(line: &str) -> (Vec<usize>, Vec<usize>) {
  let (_, rest) = line.split_once(": ").unwrap();
  let (wanted, got) = rest.split_once(" | ").unwrap();
  let wanted = wanted
    .split_whitespace()
    .map(|w| w.parse::<usize>().unwrap())
    .collect::<Vec<_>>();
  let got = got
    .split_whitespace()
    .map(|w| w.parse::<usize>().unwrap())
    .collect::<Vec<_>>();
  (wanted, got)
}

fn calculate_score(wanted: &[usize], got: &[usize]) -> usize {
  got.iter().filter(|c| wanted.contains(c)).count()
}
