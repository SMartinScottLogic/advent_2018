use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};
use utils::grid::Picture;

pub type ResultType = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub vx: i32,
    pub vy: i32,
}

#[derive(Debug, Default)]
pub struct Solution {
    pub points: Vec<Point>,
}
impl Solution {
    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        let re = regex_lite::Regex::new(
            r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>",
        )
        .unwrap();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            if let Some(caps) = re.captures(&line) {
                let x = caps[1].parse().unwrap();
                let y = caps[2].parse().unwrap();
                let vx = caps[3].parse().unwrap();
                let vy = caps[4].parse().unwrap();
                solution.add_point(Point { x, y, vx, vy });
            } else {
                // Optionally handle parse error
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut points = self.points.clone();
        loop {
            let mut cur_positions: HashMap<(i32, i32), usize> = HashMap::new();
            for point in &mut points {
                point.x += point.vx;
                point.y += point.vy;
                *cur_positions.entry((point.x, point.y)).or_default() += 1;
            }
            // Add logic to check for message formation and break when found
            if points.iter().all(|point| {
                [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ]
                .iter()
                .any(|(dx, dy)| {
                    let neighbor_pos = (point.x + dx, point.y + dy);
                    if let Some(&count) = cur_positions.get(&neighbor_pos) {
                        count > 0
                    } else {
                        false
                    }
                })
            }) {
                break;
            }
        }
        Picture::from({
            let max_x = points.iter().map(|p| p.x).max().unwrap() as usize + 1;
            let max_y = points.iter().map(|p| p.y).max().unwrap() as usize + 1;
            let mut grid = utils::grid::FixedGrid::new(max_x, max_y);
            for point in &points {
                grid.set_checked(point.x as isize, point.y as isize, '#')
                    .unwrap();
            }
            grid
        })
        .display_with_mapping(|v| if *v == '#' { "#" } else { " " });
        Ok(0)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut points = self.points.clone();
        let mut time = 0;
        loop {
            let mut cur_positions: HashMap<(i32, i32), usize> = HashMap::new();
            for point in &mut points {
                point.x += point.vx;
                point.y += point.vy;
                *cur_positions.entry((point.x, point.y)).or_default() += 1;
            }
            time += 1;
            // Add logic to check for message formation and break when found
            if points.iter().all(|point| {
                [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ]
                .iter()
                .any(|(dx, dy)| {
                    let neighbor_pos = (point.x + dx, point.y + dy);
                    if let Some(&count) = cur_positions.get(&neighbor_pos) {
                        count > 0
                    } else {
                        false
                    }
                })
            }) {
                break;
            }
        }
        Ok(time)
    }
}
