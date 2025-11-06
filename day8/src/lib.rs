use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    numbers: Vec<u64>,
}
impl Solution {
    fn add_number(&mut self, num: u64) {
        self.numbers.push(num);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            for w in line.split(' ').map(|v| v.trim()).filter(|v| !v.is_empty()) {
                let num: u64 = w.parse().unwrap();
                solution.add_number(num);
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut pos = 0;
        fn parse_node(numbers: &Vec<u64>, pos: &mut usize) -> u64 {
            let child_count = numbers[*pos] as usize;
            *pos += 1;
            let metadata_count = numbers[*pos] as usize;
            *pos += 1;
            let mut value = 0;
            for _ in 0..child_count {
                value += parse_node(numbers, pos);
            }
            for _ in 0..metadata_count {
                let metadata = numbers[*pos];
                debug!("Metadata: {}", metadata);
                *pos += 1;
                value += metadata;
            }
            value
        }
        let root_value = parse_node(&self.numbers, &mut pos);
        // Implement for problem
        Ok(root_value)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut pos = 0;
        fn parse_node(numbers: &Vec<u64>, pos: &mut usize) -> u64 {
            let child_count = numbers[*pos] as usize;
            *pos += 1;
            let metadata_count = numbers[*pos] as usize;
            *pos += 1;
            let mut value = 0;
            let mut child_values = Vec::new();
            for _ in 0..child_count {
                let child_value = parse_node(numbers, pos);
                child_values.push(child_value);
            }
            for _ in 0..metadata_count {
                let metadata = numbers[*pos];
                debug!("Metadata: {}", metadata);
                *pos += 1;
                if child_count == 0 {
                    value += metadata;
                } else if metadata >= 1 && (metadata as usize) <= child_count {
                    value += child_values[(metadata - 1) as usize];
                }
            }
            value
        }
        let root_value = parse_node(&self.numbers, &mut pos);
        // Implement for problem
        Ok(root_value)
    }
}
