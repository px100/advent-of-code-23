fn main(input: &str) -> (isize, isize) {
  let p1 = input
    .split('\n')
    .map(|line| {
      let (num, _) = line[2..].split_once(' ').unwrap();
      (line.as_bytes()[0], num.parse::<isize>().unwrap())
    });
  let p2 = input
    .split('\n')
    .map(|line| {
      let (_, color) = line.split_once(" (#").unwrap();
      let direction = match color.as_bytes()[color.len() - 2] {
        b'0' => b'R',
        b'1' => b'D',
        b'2' => b'L',
        b'3' => b'U',
        _ => unreachable!(),
      };
      (direction, isize::from_str_radix(&color[0..color.len() - 2], 16).unwrap())
    });
  (calculate_area(p1), calculate_area(p2))
}

fn calculate_area(mut instructions: impl Iterator<Item=(u8, isize)>) -> isize {
  let (mut area, mut prev_row, mut prev_col) = (0, 0, 0);
  while let Some((direction, steps)) = instructions.next() {
    let (prev_row_copy, prev_col_copy) = (prev_row, prev_col);
    match direction {
      b'U' => prev_row -= steps,
      b'R' => prev_col += steps,
      b'D' => prev_row += steps,
      b'L' => prev_col -= steps,
      _ => unreachable!(),
    };
    area += (prev_col + prev_col_copy) * (prev_row - prev_row_copy) + steps;
  }
  area / 2 + 1
}
