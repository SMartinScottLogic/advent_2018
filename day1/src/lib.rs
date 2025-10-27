use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = i64;

#[derive(Debug, Default)]
pub struct Solution {
    delta: Vec<ResultType>,
}
impl Solution {
    fn add_delta(&mut self, delta: ResultType) {
        self.delta.push(delta);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            for delta in line.split(", ") {
                info!("Line {}: delta '{}'", id + 1, delta);
                let delta = delta.parse::<ResultType>().unwrap();
                solution.add_delta(delta);
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let r = self.delta.iter().sum::<ResultType>();
        // Implement for problem
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut seen = std::collections::HashSet::new();
        let mut current = 0;
        seen.insert(current);
        for delta in self.delta.iter().cycle() {
            current += delta;
            if seen.contains(&current) {
                return Ok(current);
            }
            seen.insert(current);
        }
        // Implement for problem
        Ok(0)
    }
}
