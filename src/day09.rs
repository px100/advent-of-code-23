fn main(input: &str) -> (isize, isize) {
  let mut result = (0, 0);
  for line in input.split('\n') {
    let k: Vec<isize> = line
      .split_whitespace()
      .map(|x| x.parse::<isize>().unwrap())
      .collect();
    let (a, b) = get_sum(k);
    result.0 += a;
    result.1 += b;
  }
  result
}

fn get_sum(initial_values: Vec<isize>) -> (isize, isize) {
  let mut sequences = vec![initial_values];
  while sequences.last().unwrap().iter().any(|&value| value != 0) {
    let differences = sequences
      .last()
      .unwrap()
      .windows(2)
      .map(|window| window[1] - window[0])
      .collect();
    sequences.push(differences);
  }
  let (sum_last, diff_first) = sequences
    .into_iter()
    .rev()
    .fold((0, 0), |(sum, diff), sequence| {
      (sequence.last().unwrap() + sum, sequence[0] - diff)
    });
  (sum_last, diff_first)
}
