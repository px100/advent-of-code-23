const DIGITS: &[&[u8]] = &[b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine"];

fn sum(w: &[u8], part_two: bool) -> usize {
  let mut digits = (0..w.len())
    .filter_map(|i| match w[i] {
      b'0'..=b'9' => Some((w[i] - b'0') as usize),
      _ if part_two => DIGITS.iter()
        .enumerate()
        .find_map(|(di, d)| w[i..].starts_with(d).then_some(di + 1)),
      _ => None
    });
  let x = digits.next().unwrap();
  let y = digits.last().unwrap_or(x);
  x * 10 + y
}

fn main(input: &str) -> (usize, usize) {
  let (p1, p2) = input.lines()
    .map(str::as_bytes)
    .collect::<Vec<_>>()
    .iter()
    .map(|line| (sum(line, false), sum(line, true)))
    .fold((0, 0), |(sum1, sum2), (d1, d2)| (sum1 + d1, sum2 + d2));
  (p1, p2)
}
