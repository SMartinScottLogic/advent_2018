use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    pub serial_number: i32,
}
impl Solution {
    pub fn serial_number(&mut self, serial_number: i32) {
        self.serial_number = serial_number;
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            solution.serial_number(line.trim().parse().unwrap());
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut levels = vec![vec![0i32; 300]; 300];
        for x in 0..300 {
            for y in 0..300 {
                let rack_id = x + 10;
                let mut power_level = rack_id * y;
                power_level += self.serial_number;
                power_level *= rack_id;
                power_level = (power_level / 100) % 10;
                power_level -= 5;
                levels[x as usize][y as usize] = power_level;
            }
        }
        let mut grid_level = vec![vec![0i32; 298]; 298];
        for x in 0..298 {
            for y in 0..298 {
                let mut total_power = 0;
                for dx in 0..3 {
                    for dy in 0..3 {
                        total_power += levels[x + dx][y + dy];
                    }
                }
                grid_level[x][y] = total_power;
            }
        }
        let max_pos = grid_level
            .into_iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row.into_iter()
                    .enumerate()
                    .map(move |(y, power)| (x, y, power))
            })
            .max_by_key(|&(_x, _y, power)| power)
            .unwrap();
        // Implement for problem
        info!("Part 1 answer: {},{}", max_pos.0, max_pos.1);
        Ok(0)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut levels = vec![vec![0i32; 300]; 300];
        for x in 0..300 {
            for y in 0..300 {
                let rack_id = x + 10;
                let mut power_level = rack_id * y;
                power_level += self.serial_number;
                power_level *= rack_id;
                power_level = (power_level / 100) % 10;
                power_level -= 5;
                levels[x as usize][y as usize] = power_level;
            }
        }
        let mut best = None;
        for size in 1..=30 {
            let mut current_level = vec![vec![0i32; 301 - size]; 301 - size];
            for x in 0..=(300 - size) {
                for y in 0..=(300 - size) {
                    let mut total_power = 0;
                    for dx in 0..size {
                        for dy in 0..size {
                            total_power += levels[x + dx][y + dy];
                        }
                    }
                    current_level[x][y] = total_power;
                }
            }
            let current_max = current_level
                .into_iter()
                .enumerate()
                .flat_map(|(x, row)| {
                    row.into_iter()
                        .enumerate()
                        .map(move |(y, power)| (x, y, power))
                })
                .max_by_key(|&(_x, _y, power)| power)
                .unwrap();

            best = match best {
                None => Some((size, current_max.0, current_max.1, current_max.2)),
                Some(b) if b.3 < current_max.2 => {
                    Some((size, current_max.0, current_max.1, current_max.2))
                }
                Some(b) => Some(b),
            };
            debug!("Checked size {}, {:?}", size, best);
        }
        let best = best.unwrap();
        // Implement for problem
        info!("Part 2 answer: {},{},{}", best.1, best.2, best.0);
        Ok(0)
    }
}
