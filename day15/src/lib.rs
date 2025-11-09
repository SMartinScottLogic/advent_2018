use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};
use utils::grid::Matrix;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    pub grid: Matrix<char>,
    pub units: Vec<(usize, usize, char)>, // (x, y, type)
}
impl Solution {
    pub fn set_grid(&mut self, x: usize, y: usize, c: char) {
        self.grid.set(x as isize, y as isize, c);
    }
    pub fn add_unit(&mut self, x: usize, y: usize, c: char) {
        self.units.push((x, y, c));
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (y, line) in reader.lines().map_while(Result::ok).enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let ch = match ch {
                    'G' => {
                        solution.add_unit(x, y, 'G');
                        '.'
                    }
                    'E' => {
                        solution.add_unit(x, y, 'E');
                        '.'
                    }
                    other => other,
                };
                solution.set_grid(x, y, ch);
            }
            // Implement for problem
        }
        Ok(solution)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Unit {
    x: isize,
    y: isize,
    unit_type: char,
    hit_points: isize,
    attack_power: usize,
}
impl Unit {
    fn new(x: isize, y: isize, unit_type: char, hit_points: isize, attack_power: usize) -> Self {
        Self { x, y, unit_type, hit_points, attack_power }
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut units = self
            .units
            .iter()
            .cloned()
            .map(|(x, y, t)| Unit {
                x: x as isize,
                y: y as isize,
                unit_type: t,
                hit_points: 200,
                attack_power: 3,
            })
            .collect::<Vec<_>>();

        let mut round = 0;
        let mut done;
        let mut hits = 0;
        let mut total_hits = 0;
        let r = loop {
            round += 1;
            (units, done, hits) = Self::perform_round(round, &units, &self.grid);
            total_hits += hits;
            debug!("End of round {} - {}", round, total_hits);
            if done {
                break (round - 1)
                    * units
                        .iter()
                        .filter(|unit| unit.hit_points > 0)
                        .map(|unit| unit.hit_points as u64)
                        .sum::<u64>();
            }
        };
        // Other: 95 x 2613 = 248235 (1134 hits)
        /*
unit: Unit { kind: Goblin, hit_points: 200, attack_power: 3 } @ Pos(22, 11)
unit: Unit { kind: Goblin, hit_points: 200, attack_power: 3 } @ Pos(11, 13)
unit: Unit { kind: Goblin, hit_points: 134, attack_power: 3 } @ Pos(21, 13)
unit: Unit { kind: Goblin, hit_points: 200, attack_power: 3 } @ Pos(22, 14)
unit: Unit { kind: Goblin, hit_points: 200, attack_power: 3 } @ Pos(23, 14)
unit: Unit { kind: Goblin, hit_points: 200, attack_power: 3 } @ Pos(11, 15)
unit: Unit { kind: Goblin, hit_points: 17, attack_power: 3 } @ Pos(21, 15)
unit: Unit { kind: Goblin, hit_points: 179, attack_power: 3 } @ Pos(21, 17)
unit: Unit { kind: Goblin, hit_points: 200, attack_power: 3 } @ Pos(9, 18)
unit: Unit { kind: Goblin, hit_points: 134, attack_power: 3 } @ Pos(10, 20)
unit: Unit { kind: Goblin, hit_points: 200, attack_power: 3 } @ Pos(14, 21)
unit: Unit { kind: Goblin, hit_points: 200, attack_power: 3 } @ Pos(16, 21)
unit: Unit { kind: Goblin, hit_points: 200, attack_power: 3 } @ Pos(11, 22)
unit: Unit { kind: Goblin, hit_points: 200, attack_power: 3 } @ Pos(12, 22)
unit: Unit { kind: Goblin, hit_points: 149, attack_power: 3 } @ Pos(15, 22)
         */
        // Wrong: 95 * 2658 = 252510
        /*
unit: Unit { x: 22, y: 11, unit_type: 'G', hit_points: 200, attack_power: 3 }
unit: Unit { x: 11, y: 13, unit_type: 'G', hit_points: 200, attack_power: 3 }
unit: Unit { x: 21, y: 13, unit_type: 'G', hit_points: 134, attack_power: 3 }
unit: Unit { x: 22, y: 14, unit_type: 'G', hit_points: 200, attack_power: 3 }
unit: Unit { x: 23, y: 14, unit_type: 'G', hit_points: 200, attack_power: 3 }
unit: Unit { x: 11, y: 15, unit_type: 'G', hit_points: 200, attack_power: 3 }
unit: Unit { x: 21, y: 15, unit_type: 'G', hit_points: 62, attack_power: 3 } X
unit: Unit { x: 21, y: 17, unit_type: 'G', hit_points: 179, attack_power: 3 }
unit: Unit { x: 9, y: 18, unit_type: 'G', hit_points: 200, attack_power: 3 }
unit: Unit { x: 10, y: 20, unit_type: 'G', hit_points: 134, attack_power: 3 }
unit: Unit { x: 14, y: 21, unit_type: 'G', hit_points: 200, attack_power: 3 }
unit: Unit { x: 16, y: 21, unit_type: 'G', hit_points: 200, attack_power: 3 }
unit: Unit { x: 11, y: 22, unit_type: 'G', hit_points: 200, attack_power: 3 }
unit: Unit { x: 12, y: 22, unit_type: 'G', hit_points: 200, attack_power: 3 }
unit: Unit { x: 15, y: 22, unit_type: 'G', hit_points: 149, attack_power: 3 }
 */
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut elf_power = 0;
        let r = 'outer: loop {
            elf_power += 1;
            if elf_power > 50 {
                panic!();
            }
        let mut units = self
            .units
            .iter()
            .cloned()
            .map(|(x, y, t)| Unit {
                x: x as isize,
                y: y as isize,
                unit_type: t,
                hit_points: 200,
                attack_power: if t == 'E' { elf_power } else { 3 },
            })
            .collect::<Vec<_>>();

        let mut round = 0;
        let mut done;
        let mut hits = 0;
        let mut total_hits = 0;
        loop {
            round += 1;
            (units, done, hits) = Self::perform_round(round, &units, &self.grid);
            total_hits += hits;
            debug!("End of round {} - {}", round, total_hits);
            if done {
                let dead_elf_count = units.iter()
                .filter(|unit| unit.unit_type == 'E')
                .filter(|elf| elf.hit_points <= 0)
                .count();
                info!("Elf power {} {}: {} elfs died: {:?}", elf_power, round-1, dead_elf_count, units);
                if dead_elf_count == 0 {
                    let sum = units
                        .iter()
                        .filter(|unit| unit.hit_points > 0)
                        .map(|unit| unit.hit_points as u64)
                        .sum::<u64>();
                    let r = (round - 1) * sum; 
                    info!("Outcome: {} * {} = {}", round - 1, sum, r);

                    break 'outer r;
                }
                break;
            }
        };
        };
        // Other: 30 x 1538 (274 hits) = 46140 (power = 34)
        /*
unit: 8: Unit { kind: Elf, hit_points: 125, attack_power: 34 } @ Pos(16, 9)
unit: 16: Unit { kind: Elf, hit_points: 158, attack_power: 34 } @ Pos(13, 11)
unit: 20: Unit { kind: Elf, hit_points: 170, attack_power: 34 } @ Pos(22, 14)
unit: 17: Unit { kind: Elf, hit_points: 200, attack_power: 34 } @ Pos(21, 15)
unit: 28: Unit { kind: Elf, hit_points: 200, attack_power: 34 } @ Pos(19, 20)
unit: 27: Unit { kind: Elf, hit_points: 200, attack_power: 34 } @ Pos(21, 20)
unit: 22: Unit { kind: Elf, hit_points: 89, attack_power: 34 } @ Pos(8, 22)
unit: 29: Unit { kind: Elf, hit_points: 200, attack_power: 34 } @ Pos(15, 21)
unit: 23: Unit { kind: Elf, hit_points: 164, attack_power: 34 } @ Pos(7, 24)
unit: 26: Unit { kind: Elf, hit_points: 32, attack_power: 34 } @ Pos(6, 25)
        */
        // Wrong: 47678 (power = 34)
        /*
Unit { x: 16, y: 9, unit_type: 'E', hit_points: 125, attack_power: 34 },
Unit { x: 13, y: 11, unit_type: 'E', hit_points: 158, attack_power: 34 },
Unit { x: 22, y: 14, unit_type: 'E', hit_points: 170, attack_power: 34 },
Unit { x: 21, y: 15, unit_type: 'E', hit_points: 200, attack_power: 34 },
Unit { x: 19, y: 20, unit_type: 'E', hit_points: 200, attack_power: 34 },
Unit { x: 21, y: 20, unit_type: 'E', hit_points: 200, attack_power: 34 },
Unit { x: 15, y: 21, unit_type: 'E', hit_points: 200, attack_power: 34 },
Unit { x: 8, y: 22, unit_type: 'E', hit_points: 89, attack_power: 34 },
Unit { x: 7, y: 24, unit_type: 'E', hit_points: 164, attack_power: 34 },
Unit { x: 6, y: 25, unit_type: 'E', hit_points: 32, attack_power: 34 },
*/
        Ok(r)
    }
}

impl Solution {
    fn reading_order(units: &[Unit]) -> Vec<Unit> {
        let mut units = units.to_vec();
        units.sort_by(|a, b| match a.y.cmp(&b.y) {
            std::cmp::Ordering::Equal => a.x.cmp(&b.x),
            other => other,
        });
        units
    }

    fn perform_round(round: u64, units: &[Unit], grid: &Matrix<char>) -> (Vec<Unit>, bool, u64) {
        // Implement round logic
        let mut hits = 0;
        let mut units_in_order = Self::reading_order(units);
        for unit_id in 0..units_in_order.len() {
            let mut unit = units_in_order[unit_id];
            if unit.hit_points <= 0 {
                continue;
            }
            // Process unit turn
            debug!("Round {}, Unit {}: {:?}", round, unit_id, unit);
            // Identify targets
            let targets = units_in_order
                .iter()
                .filter(|target| target.hit_points > 0 && target.unit_type != unit.unit_type)
                .cloned()
                .collect::<Vec<_>>();
            if targets.is_empty() {
                debug!("  No targets remaining, ending combat");
                let sum = units_in_order
                    .iter()
                    .filter(|u| u.hit_points > 0)
                    .map(|u| u.hit_points as u64)
                    .sum::<u64>();
                debug!("Combat ends after {} full rounds", round - 1);
                let r = (round - 1) * sum;
                debug!("Outcome: {} * {} = {}", round - 1, sum, r);
                // for unit in units_in_order.iter()
                //     .filter(|u| u.hit_points > 0)
                //      {
                //         println!("unit: {:?}", unit);
                //     }

                return (units_in_order, true, hits);
            }
            debug!("targets = {:?}", targets);
            // Should we move?
            let mut in_range = false;
            for target in &targets {
                let dist = ((unit.x - target.x).abs() + (unit.y - target.y).abs()) as usize;
                if dist == 1 {
                    in_range = true;
                    debug!("in_range: {:?}", target);
                    //break;
                }
            }
            if !in_range {
                // Get adjacent squares to targets
                let target_squares = targets
                    .iter()
                    .flat_map(|target| {
                        vec![
                            (target.x, target.y - 1),
                            (target.x - 1, target.y),
                            (target.x + 1, target.y),
                            (target.x, target.y + 1),
                        ]
                    })
                    .filter(|(ax, ay)| grid.get(*ax, *ay) == Some(&'.'))
                    .filter(|(ax, ay)| {
                        // Ensure no unit is currently on that square
                        for other in &units_in_order {
                            if other.hit_points > 0 && (other.x, other.y) == (*ax, *ay) {
                                return false;
                            }
                        }
                        true
                    })
                    .collect::<HashSet<_>>();
                debug!("  Target squares: {:?}", target_squares);
                // 1. Find shortest path to any target square
                if let Some((next_x, next_y)) = {
                    // Implement BFS or Dijkstra to find shortest path
                    Self::find_next_move(&unit, target_squares, grid, &units_in_order)
                } {
                    debug!("  Moving to ({},{})", next_x, next_y);
                    // Update unit position
                    units_in_order[unit_id].x = next_x;
                    units_in_order[unit_id].y = next_y;
                    unit.x = next_x;
                    unit.y = next_y;
                } else {
                    debug!("  No reachable target squares");
                }
            }

            // 3. Attack if in range
            debug!("targets = {:?}", targets);
            debug!("units = {:?}", units_in_order);
            debug!("uio = {:?}", units_in_order
                .iter()
                .filter(|target| target.hit_points > 0 && target.unit_type != unit.unit_type)
                .cloned()
                .collect::<Vec<_>>());
            let target = targets
                .iter()
                .filter(|target| {
                    let dist = ((unit.x - target.x).abs() + (unit.y - target.y).abs()) as usize;
                    dist == 1
                })
                .min_by_key(|target| target.hit_points);
            let mut attack = None;
            for t in &targets {
                let dist = ((unit.x - t.x).abs() + (unit.y - t.y).abs()) as usize;
                if dist != 1 {
                    continue;
                }
                attack = match attack {
                    Some((_, hit_points)) if hit_points > t.hit_points => {
                        Some((t, t.hit_points))
                    }
                    None => Some((t, t.hit_points)),
                    other => other,
                };

            }
            let candidates = targets.iter()
            .filter(|target| {
                    let dist = ((unit.x - target.x).abs() + (unit.y - target.y).abs()) as usize;
                    dist == 1
                })
                .collect::<Vec<_>>();
            assert_eq!(attack.map(|(a, b)| a), target);
            if let Some(target) = target {
                debug!("  Attacking target {:?}", target);
                // Implement attack logic
                let mut hit = false;
                for other in units_in_order.iter_mut() {
                    if other.hit_points > 0 && other.x == target.x && other.y == target.y {
                        debug!("unit={:?} target={:?} other={:?} hp={} ap={}", unit, target, other, other.hit_points, unit.attack_power);
                        other.hit_points -= unit.attack_power as isize;
                        debug!("    Target now has {} hit points", other.hit_points);
                        hit = true;
                        hits += 1;
                        //assert!(other.hit_points > -3);
                        break;
                    }
                }
                if !hit {
                    panic!();
                }
            }
        }
        (units_in_order, false, hits)
    }

    fn find_next_move(
        unit: &Unit,
        target_squares: HashSet<(isize, isize)>,
        grid: &Matrix<char>,
        units_in_order: &[Unit],
    ) -> Option<(isize, isize)> {
        let mut queue = std::collections::VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((unit.x, unit.y, 0, None::<(isize, isize)>)); // (x, y, distance, first_step)
        visited.insert((unit.x, unit.y));
        let mut found = None;
        while let Some((cx, cy, dist, first_step)) = queue.pop_front() {
            if target_squares.contains(&(cx, cy)) {
                found = Some((cx, cy, dist));
                break;
            }
            for (nx, ny) in &[(cx, cy - 1), (cx - 1, cy), (cx + 1, cy), (cx, cy + 1)] {
                if visited.contains(&(*nx, *ny)) {
                    continue;
                }
                if grid.get(*nx, *ny) != Some(&'.') {
                    continue;
                }
                let mut unit_collision = false;
                for other in units_in_order {
                    if other.hit_points > 0 && (other.x, other.y) == (*nx, *ny) {
                        unit_collision = true;
                        break;
                    }
                }
                if unit_collision {
                    continue;
                }
                visited.insert((*nx, *ny));
                queue.push_back((*nx, *ny, dist + 1, first_step.or(Some((*nx, *ny)))));
            }
        }

        let (target_x, target_y, target_distance) = match found {
            Some(found) => found,
            None => return None,
        };

        let mut dist = HashMap::new();

        let mut queue = VecDeque::new();
        queue.push_back((target_x, target_y, 0));

        while let Some((px, py, d)) = queue.pop_front() {
            if px != target_x || py != target_y {
                if grid.get(px, py) != Some(&'.') {
                    continue;
                }
                let mut unit_collision = false;
                for other in units_in_order {
                    if other.hit_points > 0 && (other.x, other.y) == (px, py) {
                        unit_collision = true;
                        break;
                    }
                }
                if unit_collision {
                    continue;
                }
            }

            match dist.entry((px, py)) {
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert(d);
                }
                std::collections::hash_map::Entry::Occupied(mut e) => {
                    if *e.get() <= d {
                        continue;
                    }

                    e.insert(d);
                }
            }


            for (nx, ny) in &[(px, py - 1), (px - 1, py), (px + 1, py), (px, py + 1)] {
                queue.push_back((*nx, *ny, d + 1));
            }
        }

        let mut candidates = Vec::new();

        for n in [(unit.x, unit.y - 1), (unit.x - 1, unit.y), (unit.x + 1, unit.y), (unit.x, unit.y + 1)] {
            if let Some(d) = dist.get(&n).cloned() {
                if d == target_distance - 1 {
                    candidates.push(n);
                }
            }
        }

        candidates.sort_by(|a, b| {
            match a.1.cmp(&b.1) {
                std::cmp::Ordering::Equal => a.0.cmp(&b.0),
                other => other
            }
        });
        candidates
        .into_iter().next()
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
    fn next_move() {
        let mut grid = Matrix::new();
        let mut units_in_order = Vec::new();
        for (y, line) in ["###########","#.G.#...G.#","###..E#####","###########"].into_iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let ch = match c {
                    'G' => { units_in_order.push( Unit::new(x as isize, y as isize, c, 200, 3)); '.' },
                    'E' => { '.' },
                    other => other,
                };
                grid.set(x as isize, y as isize, ch);
            }
        }
        grid.display();
        let unit = Unit::new(5, 2, 'E', 200, 3);
        let mut target_squares = HashSet::new();
        target_squares.insert((3, 1));
        target_squares.insert((7, 1));
        let next = super::Solution::find_next_move(&unit, target_squares, &grid, &units_in_order);
        assert!(next.is_some());
        assert_eq!(next.unwrap(), (4, 2));
    }
}
