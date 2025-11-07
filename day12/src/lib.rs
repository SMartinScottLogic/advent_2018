#![feature(iter_map_windows)]
use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    initial_state: String,
    rules: Vec<(Vec<char>, char)>,
}
impl Solution {
    pub fn add_rule(&mut self, pattern: Vec<char>, result: char) {
        self.rules.push((pattern, result));
    }
    pub fn set_initial_state(&mut self, state: &str) {
        self.initial_state = state.to_owned();
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            let line = line.trim();
            if id == 0 {
                // Parse initial state
                let parts: Vec<&str> = line.split(": ").collect();
                if parts.len() == 2 {
                    let state_str = parts[1];
                    solution.set_initial_state(state_str);
                } else {
                    panic!("Invalid initial state line: {}", line);
                }
            } else if !line.is_empty() {
                // Parse rules
                let parts: Vec<&str> = line.split(" => ").collect();
                if parts.len() == 2 {
                    let pattern: Vec<char> = parts[0].chars().collect();
                    let result: char = parts[1].chars().next().unwrap();
                    solution.add_rule(pattern, result);
                } else {
                    panic!("Invalid rule line: {}", line);
                }
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut current = self
            .initial_state
            .chars()
            .enumerate()
            .filter(|&(_, c)| c == '#')
            .fold(HashMap::new(), |mut acc, (i, _)| {
                acc.insert(i as i64, true);
                acc
            });

        for generation in 1..=20 {
            let mut next = HashMap::new();
            let min_index = *current.keys().min().unwrap() - 2;
            let max_index = *current.keys().max().unwrap() + 2;
            info!(
                "Generation {}: range {} to {}",
                generation, min_index, max_index
            );
            for i in min_index..=max_index {
                let pattern: Vec<char> = (-2..=2)
                    .map(|offset| {
                        if current.contains_key(&(i + offset)) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect();
                let mut found = false;
                for (rule_pattern, result) in &self.rules {
                    if *rule_pattern == pattern {
                        if *result == '#' {
                            next.insert(i, true);
                        }
                        found = true;
                        break;
                    }
                }
                if !found {
                    // Default to '.' if no rule matches
                }
            }
            // Update current to next for the next generation
            // current = next; // Note: current is immutable, need to redefine
            current = next;
        }

        info!("Final state after 20 generations: {:?}", current);
        let sum: i64 = current.keys().sum();
        info!("Sum of pot numbers with plants: {}", sum);

        // Implement for problem
        Ok(0)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        Ok(0)
    }
}
