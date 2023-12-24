use hashbrown::HashMap;
use itertools::Itertools;

type RulesMap<'a> = HashMap<&'a str, (Vec<(char, bool, usize, &'a str)>, &'a str)>;

fn main(input: &str) -> (usize, usize) {
  let (rules_str, parts) = input
    .split_once("\n\n")
    .unwrap();
  let rules = rules_str.lines().map(|line| {
    let (name, rest) = line
      .split_once('{')
      .unwrap();
    let (rest, label) = rest[0..rest.len() - 1]
      .split_at(rest.rfind(',').unwrap());
    let conditions = rest.split(',').map(|condition| {
      let (rest, label) = condition
        .split_once(':')
        .unwrap();
      let lt = rest.contains('<');
      let (name, n) = rest
        .split_once(if lt { '<' } else { '>' })
        .unwrap();
      (name.as_bytes()[0] as char, lt, n.parse::<usize>().unwrap(), label)
    }).collect::<Vec<_>>();
    (name, (conditions, &label[1..]))
  }).collect::<RulesMap<>>();
  let p1 = parts
    .lines()
    .map(|line| line.split(|c: char| !c.is_ascii_digit())
      .filter(|s| !s.is_empty())
      .map(|w| w.parse::<usize>().unwrap())
      .collect_tuple()
      .unwrap())
    .filter(|&(x, m, a, s)| filter(&rules, [x, m, a, s]))
    .map(|(x, m, a, s)| x + m + a + s)
    .sum();
  let p2 = count(&rules, "in", std::array::from_fn(|_| (1..=4000).collect::<Vec<_>>()));
  (p1, p2)
}

fn filter(rules: &RulesMap<'_>, values: [usize; 4]) -> bool {
  let mut current = "in";
  while current != "A" && current != "R" {
    let (rules_list, fallback_label) = &rules[current];
    current = rules_list
      .iter()
      .find_map(|&(p, lt, n, label)| {
        let index = "xmas".find(p).unwrap();
        if (lt && values[index] < n) || (!lt && values[index] > n) {
          Some(label)
        } else {
          None
        }
      }).unwrap_or_else(|| fallback_label);
  }
  current == "A"
}

fn count(rules: &RulesMap<'_>, current: &str, mut ranges: [Vec<usize>; 4]) -> usize {
  if current == "R" {
    return 0;
  }
  if current == "A" {
    return ranges
      .iter()
      .map(|v| v.len())
      .product();
  }
  let (rules_list, fallback_label) = &rules[current];
  let result = rules_list
    .iter()
    .map(|&(p, lt, n, label)| {
      let index = "xmas"
        .chars()
        .position(|c| c == p)
        .unwrap();
      let mut new_ranges = ranges.clone();
      (new_ranges[index], ranges[index]) = ranges[index]
        .iter()
        .partition(|&&val| if lt { val < n } else { val > n });
      count(rules, label, new_ranges)
    }).sum::<usize>();
  result + count(rules, fallback_label, ranges)
}
