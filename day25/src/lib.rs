use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    points: Vec<(isize, isize, isize, isize)>,
}
impl Solution {
    fn add_point(&mut self, x: isize, y: isize, z: isize, t: isize) {
        self.points.push((x, y, z, t));
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        let r = regex_lite::Regex::new(r"(-?\d+),(-?\d+),(-?\d+),(-?\d+)$").unwrap();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            let line = line.trim();
            if let Some(caps) = r.captures(line) {
                let x = caps[1].parse().unwrap();
                let y = caps[2].parse().unwrap();
                let z = caps[3].parse().unwrap();
                let t = caps[4].parse().unwrap();
                solution.add_point(x, y, z, t);
            } else {
                panic!("Failed to parse: '{}'", line);
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut cluster = (0..self.points.len()).collect::<Vec<_>>();

        let mut changed;
        loop {
            changed = false;
            for i in 0..self.points.len() {
                let a = &self.points[i];
                for j in 0..i {
                    let b = &self.points[j];
                    let distance = (a.0 - b.0).abs()
                        + (a.1 - b.1).abs()
                        + (a.2 - b.2).abs()
                        + (a.3 - b.3).abs();
                    if distance <= 3 && cluster[i] != cluster[j] {
                        debug!("{} {} = {}", i, j, distance);
                        // merge clusters
                        let source = std::cmp::max(cluster[i], cluster[j]);
                        let target = std::cmp::min(cluster[i], cluster[j]);
                        for c in cluster.iter_mut() {
                            if *c == source {
                                *c = target;
                            }
                        }
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }
        debug!("clusters = {:?}", cluster);
        let cluster =
            cluster
                .iter()
                .enumerate()
                .fold(HashMap::<usize, Vec<_>>::new(), |mut acc, v| {
                    acc.entry(*v.1).or_default().push(v.0);
                    acc
                });
        debug!("clusters = {:?}", cluster);
        // Implement for problem
        Ok(cluster.len() as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        Ok(0)
    }
}
