use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    // Implement for problem
    polymers: Vec<String>,
}
impl Solution {
    pub fn add_polymer(&mut self, line: &str) {
        // Implement for problem
        self.polymers.push(line.to_string())
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            let line = line.trim();
            solution.add_polymer(line);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let r = self
            .polymers
            .iter()
            .map(|polymer| length(polymer) as u64)
            .sum();
        // Implement for problem
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let r = self
            .polymers
            .iter()
            .map(|polymer| {
                let mut min_length = polymer.len() as u64;
                for unit in b'a'..=b'z' {
                    let filtered: String = polymer
                        .chars()
                        .filter(|&c| c as u8 != unit && c as u8 != unit - 32)
                        .collect();
                    info!(
                        "Filtered polymer length without {}: {} / {}",
                        unit as char,
                        filtered.len(),
                        polymer.len()
                    );
                    let reacted_length = length(&filtered) as u64;
                    if reacted_length < min_length {
                        min_length = reacted_length;
                    }
                }
                min_length
            })
            .sum();
        // Implement for problem
        Ok(r)
    }
}

fn length(polymer: &str) -> usize {
    let mut polymer: String = polymer.to_string();
    loop {
        let original_length = polymer.len();
        let mut i = 0;
        while i + 1 < polymer.len() {
            let a = polymer.as_bytes()[i];
            let b = polymer.as_bytes()[i + 1];
            if a != b && a.eq_ignore_ascii_case(&b) {
                polymer.remove(i);
                polymer.remove(i);
            } else {
                i += 1;
            }
        }
        if polymer.len() == original_length {
            break;
        }
    }

    polymer.len()
}
