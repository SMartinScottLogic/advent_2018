use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};
use utils::grid::Matrix;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    grid: Matrix<char>,
    minx: isize,
    maxx: isize,
    miny: isize,
    maxy: isize,
}
impl Solution {
    fn set_cell(&mut self, x: isize, y: isize) {
        if self.grid.is_empty() {
            self.minx = x;
            self.miny = y;
        }
        self.grid.set(x, y, '#');
        self.minx = std::cmp::min(self.minx, x);
        self.maxx = std::cmp::max(self.maxx, x);
        self.miny = std::cmp::min(self.miny, y);
        self.maxy = std::cmp::max(self.maxy, y);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        let vertical = regex_lite::Regex::new(r"^x=(?<x>\d+), y=(?<y1>\d+)\.\.(?<y2>\d+)$").unwrap();
        let horizontal = regex_lite::Regex::new(r"^y=(?<y>\d+), x=(?<x1>\d+)\.\.(?<x2>\d+)$").unwrap();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            let line = line.trim();
            if let Some(capt) = vertical.captures(line) {
                let x: isize = capt.name("x").unwrap().as_str().parse().unwrap();
                let y1: isize = capt.name("y1").unwrap().as_str().parse().unwrap();
                let y2: isize = capt.name("y2").unwrap().as_str().parse().unwrap();
                info!(x, y1, y2, "vertical");
                for y in y1..=y2 {
                    solution.set_cell(x, y);
                }
            } else if let Some(capt) = horizontal.captures(line) {
                let y: isize = capt.name("y").unwrap().as_str().parse().unwrap();
                let x1: isize = capt.name("x1").unwrap().as_str().parse().unwrap();
                let x2: isize = capt.name("x2").unwrap().as_str().parse().unwrap();
                info!(y, x1, x2, "horizontal");
                for x in x1..=x2 {
                    solution.set_cell(x, y);
                }
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
        for y in self.miny..=self.maxy {
            let mut line = String::new();
            for x in self.minx..=self.maxx {
                line.push(match self.grid.get(x, y) {
                    Some(c) => *c,
                    None => '.'
                });
            }
            println!("{:0>4} {}", y, line);
        }
                
        // Implement for problem
        Ok(0)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
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
