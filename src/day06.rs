fn main(input: &str) -> (usize, usize) {
  let (line1, line2) = input.split_once('\n').unwrap();
  let times = line1
    .split_whitespace()
    .skip(1)
    .map(|word| word.parse().unwrap())
    .collect::<Vec<_>>();
  let distances = line2
    .split_whitespace()
    .skip(1)
    .map(|word| word.parse().unwrap())
    .collect::<Vec<_>>();
  let time2 = line1
    .split_whitespace()
    .skip(1)
    .collect::<String>()
    .parse()
    .unwrap();
  let dist2 = line2
    .split_whitespace()
    .skip(1)
    .collect::<String>()
    .parse()
    .unwrap();
  let part1_result = times
    .iter()
    .zip(distances)
    .map(|(&t, d)| calculate_span(t, d))
    .product();
  let part2_result = calculate_span(time2, dist2);
  (part1_result, part2_result)
}

pub(crate) fn calculate_span(t: usize, d: usize) -> usize {
  let discriminant = ((t * t - 4 * d) as f64).sqrt();
  let lower_bound = (t as f64 - discriminant) / 2.0;
  let upper_bound = (t as f64 + discriminant) / 2.0;
  (upper_bound.floor() - lower_bound.ceil()) as usize + 1
}
