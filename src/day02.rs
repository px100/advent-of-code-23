fn run_game(line: &str) -> Option<(usize, usize)> {
  let (id, game) = line.trim_start_matches("Game ").split_once(':')?;
  let (mut red_max, mut green_max, mut blue_max, mut is_possible) = (0, 0, 0, true);
  for action in game.split([';', ',']) {
    let (n, color) = action.trim().split_once(' ')?;
    let n = n.parse().ok()?;
    match color.as_bytes()[0] {
      b'r' => {
        is_possible &= n <= 12;
        red_max = red_max.max(n)
      }
      b'g' => {
        is_possible &= n <= 13;
        green_max = green_max.max(n)
      }
      b'b' => {
        is_possible &= n <= 14;
        blue_max = blue_max.max(n)
      }
      _ => unreachable!(),
    }
  }
  Some((if is_possible { id.parse().ok()? } else { 0 }, red_max * green_max * blue_max))
}

fn main(input: &str) -> (usize, usize) {
  input.split('\n').fold((0, 0), |(p1, p2), line| {
    let (parsed_id, parsed_value) = run_game(line).unwrap();
    (p1 + parsed_id, p2 + parsed_value)
  })
}
