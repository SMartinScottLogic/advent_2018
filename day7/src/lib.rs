use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = String;

#[derive(Debug, Default)]
pub struct Solution {
    edges: Vec<(char, char)>,
}
impl Solution {
    fn add_edge(&mut self, lhs: &str, rhs: &str) {
        self.edges
            .push((lhs.chars().next().unwrap(), rhs.chars().next().unwrap()));
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            let line = line.split(' ').collect::<Vec<_>>();
            let lhs = line[1];
            let rhs = line[7];
            solution.add_edge(lhs, rhs);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let requirements =
            self.edges
                .iter()
                .fold(HashMap::<char, Vec<char>>::new(), |mut acc, (lhs, rhs)| {
                    acc.entry(*rhs).or_default().push(*lhs);
                    acc
                });
        let mut remaining_nodes: HashSet<char> = self
            .edges
            .iter()
            .flat_map(|(lhs, rhs)| vec![*lhs, *rhs])
            .collect();
        debug!("All nodes: {:?}", remaining_nodes);
        debug!("Requirements: {:?}", requirements);
        let mut r = String::new();
        loop {
            let mut available_nodes: Vec<char> = remaining_nodes
                .iter()
                .copied()
                .filter(|node| match requirements.get(node) {
                    Some(reqs) => reqs.iter().all(|r| !remaining_nodes.contains(r)),
                    None => true,
                })
                .collect();
            available_nodes.sort_unstable();
            if available_nodes.is_empty() {
                break;
            }
            let next_node = available_nodes[0];
            debug!("Next node: {}", next_node);
            r.push(next_node);
            remaining_nodes.remove(&next_node);
        }
        // Implement for problem
        Ok(r)
    }

    fn answer_part2(&self, is_full: bool) -> Self::Result {
        let requirements =
            self.edges
                .iter()
                .fold(HashMap::<char, Vec<char>>::new(), |mut acc, (lhs, rhs)| {
                    acc.entry(*rhs).or_default().push(*lhs);
                    acc
                });
        let mut remaining_nodes: HashSet<char> = self
            .edges
            .iter()
            .flat_map(|(lhs, rhs)| vec![*lhs, *rhs])
            .collect();
        let mut uncompleted_nodes = remaining_nodes.clone();
        debug!("All nodes: {:?}", remaining_nodes);
        debug!("Requirements: {:?}", requirements);
        let mut r = String::new();
        let mut workers: Vec<(Option<char>, usize)> = if is_full {
            vec![(None, 0); 5]
        } else {
            vec![(None, 0); 2]
        };
        let fixed_time = if is_full { 60 } else { 0 };
        let mut time = 0;
        loop {
            // Assign workers
            for worker in workers.iter_mut() {
                if worker.1 == 0 {
                    let mut available_nodes: Vec<char> = remaining_nodes
                        .iter()
                        .copied()
                        .filter(|node| match requirements.get(node) {
                            Some(reqs) => reqs.iter().all(|r| !uncompleted_nodes.contains(r)),
                            None => true,
                        })
                        .collect();
                    available_nodes.sort_unstable();
                    if available_nodes.is_empty() {
                        continue;
                    }
                    let next_node = available_nodes[0];
                    debug!("Assigning node {} to worker at {}", next_node, time);
                    worker.0 = Some(next_node);
                    worker.1 = fixed_time + (next_node as u8 - b'A' + 1) as usize;
                    remaining_nodes.remove(&next_node);
                }
            }
            // Advance time
            let time_advance = workers
                .iter()
                .filter_map(|(_, t)| if *t > 0 { Some(*t) } else { None })
                .min();
            if let Some(t) = time_advance {
                time += t;
                for worker in workers.iter_mut() {
                    if worker.1 > 0 {
                        worker.1 -= t;
                        if worker.1 == 0 {
                            let finished_node = worker.0.take().unwrap();
                            debug!("Worker finished node {} at {}", finished_node, time);
                            r.push(finished_node);
                            uncompleted_nodes.remove(&finished_node);
                        }
                    }
                }
            } else {
                break;
            }
        }
        // Implement for problem
        Ok(time.to_string())
    }
}
