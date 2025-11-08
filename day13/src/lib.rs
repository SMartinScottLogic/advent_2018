use std::{
    cmp::Ordering,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};
use utils::{grid::Matrix, point::Point};

pub type ResultType = String;

#[derive(Debug, Default, Clone)]
enum IntersectionMode {
    #[default]
    TurnLeft,
    GoStraight,
    TurnRight,
}

#[derive(Debug, Default)]
pub struct Solution {
    grid: Matrix<char>,
    carts: Vec<(Point<i32>, char, IntersectionMode)>,
}
impl Solution {
    pub fn set_char(&mut self, x: usize, y: usize, c: char) {
        self.grid.set(x as isize, y as isize, c);
    }

    pub fn add_initial_cart(&mut self, point: Point<i32>, c: char) {
        self.carts.push((point, c, IntersectionMode::default()));
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (y, line) in reader.lines().map_while(Result::ok).enumerate() {
            for (x, c) in line.chars().enumerate() {
                let gc = match c {
                    '^' | 'v' => {
                        let point = Point::new(x as i32, y as i32);
                        solution.add_initial_cart(point, c);
                        '|'
                    }
                    '<' | '>' => {
                        let point = Point::new(x as i32, y as i32);
                        solution.add_initial_cart(point, c);
                        '-'
                    }
                    other => other,
                };
                solution.set_char(x, y, gc);
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut carts = self.carts.clone();
        loop {
            carts.sort_by(|a, b| match a.0.y().cmp(&b.0.y()) {
                Ordering::Equal => a.0.x().cmp(&b.0.x()),
                other => other,
            });
            debug!("Carts: {:?}", carts);

            for cart_id in 0..carts.len() {
                let cart = &carts[cart_id];
                let next_cart = match cart.1 {
                    '^' => cart.0.north(),
                    'v' => cart.0.south(),
                    '<' => cart.0.west(),
                    '>' => cart.0.east(),
                    _ => panic!("Invalid cart direction"),
                };
                let mut intersection_mode = cart.2.clone();
                // Check for cart rotations
                let direction = match self
                    .grid
                    .get(next_cart.x() as isize, next_cart.y() as isize)
                    .unwrap()
                {
                    '/' => match cart.1 {
                        '^' => '>',
                        'v' => '<',
                        '<' => 'v',
                        '>' => '^',
                        _ => panic!("Invalid cart direction"),
                    },
                    '\\' => match cart.1 {
                        '^' => '<',
                        'v' => '>',
                        '<' => '^',
                        '>' => 'v',
                        _ => panic!("Invalid cart direction"),
                    },
                    '+' => {
                        // Implement intersection logic
                        match intersection_mode {
                            IntersectionMode::TurnLeft => {
                                intersection_mode = IntersectionMode::GoStraight;
                                match cart.1 {
                                    '^' => '<',
                                    'v' => '>',
                                    '<' => 'v',
                                    '>' => '^',
                                    _ => panic!("Invalid cart direction"),
                                }
                            }
                            IntersectionMode::GoStraight => {
                                intersection_mode = IntersectionMode::TurnRight;
                                cart.1
                            }
                            IntersectionMode::TurnRight => {
                                intersection_mode = IntersectionMode::TurnLeft;
                                match cart.1 {
                                    '^' => '>',
                                    'v' => '<',
                                    '<' => '^',
                                    '>' => 'v',
                                    _ => panic!("Invalid cart direction"),
                                }
                            }
                        }
                    }
                    '|' => cart.1,
                    '-' => cart.1,
                    other => panic!("Invalid track piece: {}", other),
                };
                debug!("Moved cart {} to {:?}", cart_id, next_cart);
                // Check for collisions
                for (other_id, other_cart) in carts.iter().enumerate() {
                    if other_id != cart_id && other_cart.0 == next_cart {
                        debug!("Collision at {:?}", next_cart);
                        return Ok(format!("{},{}", next_cart.x(), next_cart.y()));
                    }
                }
                // For now, just move the cart
                carts[cart_id] = (next_cart, direction, intersection_mode);
            }
        }
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut carts = self
            .carts
            .iter()
            .cloned()
            .map(|(pos, dir, intersection_mode)| (pos, dir, intersection_mode, false))
            .collect::<Vec<_>>();
        loop {
            carts.sort_by(|a, b| match a.0.y().cmp(&b.0.y()) {
                Ordering::Equal => a.0.x().cmp(&b.0.x()),
                other => other,
            });
            debug!("Carts: {:?}", carts);

            for cart_id in 0..carts.len() {
                let cart = &carts[cart_id];
                if cart.3 {
                    // Cart is destroyed
                    continue;
                }
                let next_cart = match cart.1 {
                    '^' => cart.0.north(),
                    'v' => cart.0.south(),
                    '<' => cart.0.west(),
                    '>' => cart.0.east(),
                    _ => panic!("Invalid cart direction"),
                };
                let mut intersection_mode = cart.2.clone();
                // Check for cart rotations
                let direction = match self
                    .grid
                    .get(next_cart.x() as isize, next_cart.y() as isize)
                    .unwrap()
                {
                    '/' => match cart.1 {
                        '^' => '>',
                        'v' => '<',
                        '<' => 'v',
                        '>' => '^',
                        _ => panic!("Invalid cart direction"),
                    },
                    '\\' => match cart.1 {
                        '^' => '<',
                        'v' => '>',
                        '<' => '^',
                        '>' => 'v',
                        _ => panic!("Invalid cart direction"),
                    },
                    '+' => {
                        // Implement intersection logic
                        match intersection_mode {
                            IntersectionMode::TurnLeft => {
                                intersection_mode = IntersectionMode::GoStraight;
                                match cart.1 {
                                    '^' => '<',
                                    'v' => '>',
                                    '<' => 'v',
                                    '>' => '^',
                                    _ => panic!("Invalid cart direction"),
                                }
                            }
                            IntersectionMode::GoStraight => {
                                intersection_mode = IntersectionMode::TurnRight;
                                cart.1
                            }
                            IntersectionMode::TurnRight => {
                                intersection_mode = IntersectionMode::TurnLeft;
                                match cart.1 {
                                    '^' => '>',
                                    'v' => '<',
                                    '<' => '^',
                                    '>' => 'v',
                                    _ => panic!("Invalid cart direction"),
                                }
                            }
                        }
                    }
                    '|' => cart.1,
                    '-' => cart.1,
                    other => panic!("Invalid track piece: {}", other),
                };
                debug!("Moved cart {} to {:?}", cart_id, next_cart);
                // Check for collisions
                carts[cart_id] = (next_cart, direction, intersection_mode, false);
                for other_id in 0..carts.len() {
                    if other_id != cart_id && carts[other_id].0 == next_cart && !carts[other_id].3 {
                        debug!("Collision at {:?}", next_cart);
                        // Mark both carts as destroyed
                        carts[cart_id].3 = true;
                        carts[other_id].3 = true;
                    }
                }
                // Count remaining carts
                let remaining_carts: Vec<_> = carts.iter().filter(|c| !c.3).cloned().collect();
                if remaining_carts.len() == 1 {
                    // Only way we can have one cart left is if we just had a collision, so move the last cart
                    let last_cart = &remaining_carts[0];
                    let last_cart = match last_cart.1 {
                        '^' => last_cart.0.north(),
                        'v' => last_cart.0.south(),
                        '<' => last_cart.0.west(),
                        '>' => last_cart.0.east(),
                        _ => panic!("Invalid cart direction"),
                    };
                    return Ok(format!("{},{}", last_cart.x(), last_cart.y()));
                }

                if remaining_carts.is_empty() {
                    panic!("All carts destroyed");
                }
            }
        }
    }
}
