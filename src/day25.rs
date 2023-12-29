use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::FromIterator;

fn main(input: &str) -> (usize) {
  let graph: HashMap<_, HashSet<_>> = input
    .lines()
    .flat_map(|line| {
      let (node, rest) = line.split_once(": ").unwrap();
      rest.split(' ').flat_map(move |neighbor| {
        vec![(node, neighbor), (neighbor, node)].into_iter()
      })
    })
    .fold(HashMap::new(), |mut acc, (node, neighbor)| {
      acc.entry(node).or_default().insert(neighbor);
      acc
    });
  graph
    .keys()
    .find_map(|k| solve(&graph, graph.keys().next().unwrap(), k))
    .unwrap()
}

fn solve(graph: &HashMap<&str, HashSet<&str>>, source: &str, target: &str) -> Option<usize> {
  let mut flow = HashMap::new();
  for _i in 0..=3 {
    let mut predecessors = HashMap::new();
    let mut queue = VecDeque::from_iter([source].iter().cloned());
    let mut visited = 0;
    while let Some(cur_vertex) = queue.pop_front() {
      if predecessors.contains_key(target) {
        continue;
      }
      visited += 1;
      let neighbors = graph[cur_vertex]
        .iter()
        .filter(|&&next| next != source)
        .filter(|&&next| !predecessors.contains_key(next))
        .filter(|&&next| *flow.get(&(cur_vertex, next)).unwrap_or(&0) < 1)
        .cloned()
        .collect::<Vec<_>>();
      queue.extend(neighbors);
      for &next_vertex in &graph[cur_vertex] {
        if next_vertex != source {
          if *flow.get(&(cur_vertex, next_vertex)).unwrap_or(&0) < 1 {
            if !predecessors.contains_key(next_vertex) {
              predecessors.insert(next_vertex, cur_vertex);
            }
          }
        }
      }
    }
    if !predecessors.contains_key(target) {
      return Some(visited * (graph.len() - visited)).filter(|_| visited != graph.len());
    }
    let delta_flow = predecessors
      .iter()
      .fold(i64::MAX, |df, (&current, &previous)| {
        df.min(1 - *flow.get(&(previous, current)).unwrap_or(&0))
      });
    let mut cur_vertex = target;
    while let Some(&previous_vertex) = predecessors.get(cur_vertex) {
      *flow.entry((previous_vertex, cur_vertex)).or_default() += delta_flow;
      *flow.entry((cur_vertex, previous_vertex)).or_default() -= delta_flow;
      cur_vertex = previous_vertex;
    }
  }
  None
}
