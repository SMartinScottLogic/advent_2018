use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    ids: Vec<String>,
}
impl Solution {
    fn add_id(&mut self, id: &str) {
        self.ids.push(id.to_string());
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            let id = line.split(' ').next().unwrap();
            solution.add_id(id);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut count_twos = 0;
        let mut count_threes = 0;
        for id in &self.ids {
            let mut counts = std::collections::HashMap::new();
            for c in id.chars() {
                *counts.entry(c).or_insert(0) += 1;
            }
            let has_two = counts.values().any(|&v| v == 2);
            let has_three = counts.values().any(|&v| v == 3);
            info!("ID '{}' has_two={} has_three={}", id, has_two, has_three);
            if has_two {
                count_twos += 1;
            }
            if has_three {
                count_threes += 1;
            }
        }

        // Implement for problem
        Ok(count_twos * count_threes)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        for (i, id1) in self.ids.iter().enumerate() {
            for id2 in self.ids.iter().skip(i + 1) {
                let distance = id1
                    .chars()
                    .zip(id2.chars())
                    .filter(|(c1, c2)| c1 != c2)
                    .count();
                if distance == 1 {
                    info!(
                        result = id1
                            .chars()
                            .zip(id2.chars())
                            .filter(|(c1, c2)| c1 == c2)
                            .map(|(c, _)| c)
                            .collect::<String>()
                    );
                }
            }
        }
        // Implement for problem
        Ok(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    use tracing_test::traced_test;
    use utils::Solution;

    #[test]
    #[traced_test]
    fn read() {
        let input = "replace for problem";
        let r = BufReader::new(input.as_bytes());
        let s = crate::Solution::try_from(r).unwrap();
        assert_eq!(0 as ResultType, s.answer_part1(false).unwrap());
    }
}
