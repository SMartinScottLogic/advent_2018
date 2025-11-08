use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = String;

#[derive(Debug, Default)]
pub struct Solution {
    pub value: u64,
}
impl Solution {
    pub fn set_value(&mut self, value: u64) {
        self.value = value;
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            solution.set_value(line.parse().unwrap());
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut scores = vec![];
        scores.push(3);
        scores.push(7);
        let mut first = 0;
        let mut second = 1;
        loop {
            let sum = scores[first] + scores[second];
            if sum >= 10 {
                scores.push(sum / 10);
            }
            scores.push(sum % 10);
            first = (first + 1 + scores[first] as usize) % scores.len();
            second = (second + 1 + scores[second] as usize) % scores.len();

            if scores.len() >= (self.value as usize + 10) {
                let result: String = scores
                    .iter()
                    .skip(self.value as usize)
                    .take(10)
                    .map(|d| char::from_digit(*d as u32, 10).unwrap())
                    .collect();
                return Ok(result);
            }
        }
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        Ok("".to_string())
    }
}
