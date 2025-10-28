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
    claims: Vec<(u32, u32, u32, u32, u32)>, // (id, sx, sy, dx, dy)
}
impl Solution {
    fn add_claim(&mut self, id: u32, sx: u32, sy: u32, dx: u32, dy: u32) {
        // Implement for problem
        self.claims.push((id, sx, sy, dx, dy));
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            let line = line.trim();
            let regex = regex_lite::Regex::new(
                r"^#(?<id>\d+)\s@\s(?<sx>\d+),(?<sy>\d+):\s(?<dx>\d+)x(?<dy>\d+)$",
            )
            .unwrap();
            let caps = regex.captures(line).unwrap();
            let id: u32 = caps["id"].parse().unwrap();
            let sx: u32 = caps["sx"].parse().unwrap();
            let sy: u32 = caps["sy"].parse().unwrap();
            let dx: u32 = caps["dx"].parse().unwrap();
            let dy: u32 = caps["dy"].parse().unwrap();
            info!("id={} sx={} sy={} dx={} dy={}", id, sx, sy, dx, dy);
            solution.add_claim(id, sx, sy, dx, dy);
            // Implement for problem
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        let mut cells: HashMap<(u32, u32), u32> = std::collections::HashMap::new();
        for claim in &self.claims {
            let (id, sx, sy, dx, dy) = *claim;
            info!(
                "Processing claim id={} sx={} sy={} dx={} dy={}",
                id, sx, sy, dx, dy
            );
            // Implement for problem
            for y in sy..sy + dy {
                for x in sx..sx + dx {
                    *cells.entry((x, y)).or_default() += 1;
                }
            }
        }
        let r = cells.iter().filter(|(_, &count)| count > 1).count();
        Ok(r as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        let mut cells: HashMap<(u32, u32), u32> = std::collections::HashMap::new();
        for claim in &self.claims {
            let (id, sx, sy, dx, dy) = *claim;
            info!(
                "Processing claim id={} sx={} sy={} dx={} dy={}",
                id, sx, sy, dx, dy
            );
            // Implement for problem
            for y in sy..sy + dy {
                for x in sx..sx + dx {
                    *cells.entry((x, y)).or_default() += 1;
                }
            }
        }
        for claim in &self.claims {
            let (id, sx, sy, dx, dy) = *claim;
            let mut intact = true;
            'outer: for y in sy..sy + dy {
                for x in sx..sx + dx {
                    if let Some(&count) = cells.get(&(x, y)) {
                        if count > 1 {
                            intact = false;
                            break 'outer;
                        }
                    }
                }
            }
            if intact {
                return Ok(id as ResultType);
            }
        }
        Ok(0 as ResultType)
    }
}
