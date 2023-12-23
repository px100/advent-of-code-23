use itertools::Itertools;

fn main(input: &str) -> (usize, usize) {
  let (seeds, rest) = input
    .split_once("\n\n")
    .unwrap();
  let seeds = seeds
    .split_whitespace()
    .skip(1)
    .map(|s| s.parse().unwrap())
    .collect::<Vec<_>>();
  let layers = rest.split("\n\n").map(|s|
    s.split('\n').skip(1).map(|l|
      l.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_tuple()
        .unwrap())
      .collect::<Vec<_>>())
    .collect::<Vec<_>>();
  (p1(seeds.clone(), &layers), p2(seeds, &layers))
}

fn p1(seeds: Vec<usize>, layers: &[Vec<(usize, usize, usize)>]) -> usize {
  let result = layers
    .iter()
    .fold(seeds, |current_seeds, mappings| {
      current_seeds.into_iter().map(|current_seed| {
        mappings
          .iter()
          .find(|&&(_, src, range)| (src..src + range).contains(&current_seed))
          .map(|(dst, src, _)| dst + current_seed - src)
          .unwrap_or(current_seed)
      }).collect()
    });
  *result.iter().min().unwrap()
}

fn p2(seeds: Vec<usize>, layers: &[Vec<(usize, usize, usize)>]) -> usize {
  let seed_ranges = seeds
    .into_iter()
    .tuples()
    .map(|(start, len)| (start, start + len))
    .collect::<Vec<_>>();
  let result = layers
    .iter()
    .fold(seed_ranges, |current_seeds, mappings| {
      current_seeds.iter().flat_map(|&(start, end)| {
        let mut mapped_ranges = Vec::new();
        let mut unmapped_ranges = vec![(start, end)];
        for &(dst, src, len) in mappings {
          let mut new_unmapped_ranges = Vec::new();
          for (range_start, range_end) in unmapped_ranges {
            let mapped_range = (range_start, range_end.min(src));
            let intersection_range = (
              range_start.max(src),
              (src + len).min(range_end),
            );
            let remaining_range = ((src + len).max(range_start), range_end);
            if mapped_range.0 < mapped_range.1 {
              new_unmapped_ranges.push(mapped_range);
            }
            if intersection_range.0 < intersection_range.1 {
              mapped_ranges.push((
                intersection_range.0 - src + dst,
                intersection_range.1 - src + dst,
              ));
            }
            if remaining_range.0 < remaining_range.1 {
              new_unmapped_ranges.push(remaining_range);
            }
          }
          unmapped_ranges = new_unmapped_ranges;
        }
        mapped_ranges.extend(unmapped_ranges);
        mapped_ranges
      }).collect()
    });
  result
    .iter()
    .map(|&(mapped_start, _)| mapped_start)
    .min()
    .unwrap()
}
