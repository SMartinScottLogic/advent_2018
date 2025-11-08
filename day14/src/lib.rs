use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = String;

#[derive(Debug, Default)]
pub struct Solution {
    pub value: String,
}
impl Solution {
    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_owned();
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            solution.set_value(line.trim());
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
        let target = self.value.parse::<usize>().unwrap();
        loop {
            let sum = scores[first] + scores[second];
            if sum >= 10 {
                scores.push(sum / 10);
            }
            scores.push(sum % 10);
            first = (first + 1 + scores[first] as usize) % scores.len();
            second = (second + 1 + scores[second] as usize) % scores.len();

            if scores.len() >= (target + 10) {
                let result: String = scores
                    .iter()
                    .skip(target)
                    .take(10)
                    .map(|d| char::from_digit(*d as u32, 10).unwrap())
                    .collect();
                return Ok(result);
            }
        }
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut scores = String::new();
        scores.push('3');
        scores.push('7');
        let mut first = 0;
        let mut second = 1;
        loop {
            let sum = scores.as_bytes()[first] - b'0' + scores.as_bytes()[second] - b'0';
            if sum >= 10 {
                scores.push(char::from_digit((sum / 10) as u32, 10).unwrap());
                if scores.ends_with(&self.value) {
                    break;
                }
            }
            scores.push(char::from_digit((sum % 10) as u32, 10).unwrap());
            if scores.ends_with(&self.value) {
                break;
            }
            first = (first + 1 + (scores.as_bytes()[first] - b'0') as usize) % scores.len();
            second = (second + 1 + (scores.as_bytes()[second] - b'0') as usize) % scores.len();
        }
        // Implement for problem
        Ok(format!("{}", scores.len() - self.value.len()))
    }
}
