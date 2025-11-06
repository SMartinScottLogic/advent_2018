use itertools::Itertools;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    // Implement for problem
    points: Vec<(u32, i32, i32)>,
}
impl Solution {
    // Implement for problem
    fn add_point(&mut self, id: u32, x: i32, y: i32) {
        self.points.push((id, x, y));
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            let (x, y) = line.split_once(',').unwrap();
            let x: i32 = x.trim().parse().unwrap();
            let y: i32 = y.trim().parse().unwrap();
            solution.add_point(id as u32, x, y);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut seen = HashMap::new();
        let mut queue = std::collections::VecDeque::new();
        self.points.iter().for_each(|&(id, x, y)| {
            seen.insert((x, y), (id, 0));
            queue.push_back((id, x, y, 0));
        });
        while let Some((id, x, y, dist)) = queue.pop_front() {
            for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                if let Some(&(other_id, other_dist)) = seen.get(&(nx, ny)) {
                    if other_dist == dist + 1 && other_id != id {
                        seen.insert((nx, ny), (u32::MAX, other_dist));
                    }
                } else if (-400..=400).contains(&nx) && (-400..=400).contains(&ny) {
                    seen.insert((nx, ny), (id, dist + 1));
                    queue.push_back((id, nx, ny, dist + 1));
                    debug!(
                        "Visiting ({},{}) from id {} at dist {}",
                        nx,
                        ny,
                        id,
                        dist + 1
                    );
                }
            }
        }
        let mut area_count = HashMap::new();
        let mut infinite_ids = std::collections::HashSet::new();
        for (&(x, y), &(id, _)) in &seen {
            if id != u32::MAX {
                *area_count.entry(id).or_insert(0) += 1;
                if x == -400 || y == -400 || x == 400 || y == 400 {
                    infinite_ids.insert(id);
                }
            }
        }
        let max_area = area_count
            .into_iter()
            .filter(|(id, _)| !infinite_ids.contains(id))
            .map(|(_, area)| area)
            .max()
            .unwrap_or(0);
        Ok(max_area)
    }

    fn answer_part2(&self, is_full: bool) -> Self::Result {
        let max_distance = if is_full { 10_000 } else { 32 };
        let (min_x, max_x) = match self.points.iter().minmax_by_key(|(_id, x, _y)| x) {
            itertools::MinMaxResult::MinMax(min, max) => (min.1, max.1),
            itertools::MinMaxResult::OneElement(e) => (e.1, e.1),
            itertools::MinMaxResult::NoElements => panic!("No points"),
        };
        let (min_y, max_y) = match self.points.iter().minmax_by_key(|(_id, _x, y)| y) {
            itertools::MinMaxResult::MinMax(min, max) => (min.2, max.2),
            itertools::MinMaxResult::OneElement(e) => (e.2, e.2),
            itertools::MinMaxResult::NoElements => panic!("No points"),
        };
        let mut region_size = 0;
        let mut queue = std::collections::VecDeque::new();
        let mut seen = std::collections::HashSet::new();
        queue.push_back(((min_x + max_x) / 2, (min_y + max_y) / 2));
        seen.insert(((min_x + max_x) / 2, (min_y + max_y) / 2));
        while let Some(p) = queue.pop_back() {
            let total_distance: i32 = self
                .points
                .iter()
                .map(|&(_id, x, y)| (x - p.0).abs() + (y - p.1).abs())
                .sum();
            if total_distance < max_distance {
                debug!(
                    "Point {:?} has total distance {} / {}",
                    p, total_distance, max_distance
                );
                region_size += 1;
                for np in [
                    (p.0 - 1, p.1),
                    (p.0 + 1, p.1),
                    (p.0, p.1 - 1),
                    (p.0, p.1 + 1),
                ] {
                    if !seen.contains(&np) {
                        seen.insert(np);
                        queue.push_back(np);
                    }
                }
            }
        }
        // Implement for problem
        Ok(region_size)
    }
}
