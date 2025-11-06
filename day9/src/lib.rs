use std::{
    collections::VecDeque,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    num_players: usize,
    last_marble: usize,
}
impl Solution {
    pub fn init(&mut self, num_players: usize, last_marble: usize) {
        self.num_players = num_players;
        self.last_marble = last_marble;
        debug!(
            "Initialized with {} players and last marble worth {} points",
            num_players, last_marble
        );
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        let r = regex_lite::Regex::new(
            r"^(?<players>\d+) players; last marble is worth (?<last_marble>\d+) points",
        )
        .unwrap();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            if let Some(caps) = r.captures(&line) {
                let num_players: usize = caps["players"].parse().unwrap();
                let last_marble: usize = caps["last_marble"].parse().unwrap();
                solution.init(num_players, last_marble);
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        let mut score: Vec<u64> = vec![0; self.num_players];
        let mut circle: Vec<usize> = vec![0];
        let mut current_index = 0;
        for marble in 1..=self.last_marble {
            if marble % 23 == 0 {
                let player = (marble - 1) % self.num_players;
                score[player] += marble as u64;
                current_index = (current_index + circle.len() - 7) % circle.len();
                score[player] += circle.remove(current_index) as u64;
            } else {
                current_index = (current_index + 2) % circle.len();
                if current_index == 0 {
                    circle.push(marble);
                    current_index = circle.len() - 1;
                } else {
                    circle.insert(current_index, marble);
                }
            }
        }
        let max_score = score.into_iter().max().unwrap();
        Ok(max_score)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut players_score = vec![0; self.num_players];
        let mut ring = VecDeque::new();
        ring.push_front(0);

        for marble in 1..=self.last_marble * 100 {
            if marble % 23 == 0 {
                // rotate of 7 behind + delete
                (0..7).for_each(|_| {
                    let tmp = ring.pop_back().expect("Rotate problem");
                    ring.push_front(tmp);
                });
                players_score[marble % self.num_players] +=
                    marble + ring.pop_front().expect("No value in the ring");
            } else {
                // rotate of 2 ahead + insert
                (0..2).for_each(|_| {
                    let tmp = ring.pop_front().expect("Rotate problem");
                    ring.push_back(tmp);
                });
                ring.push_front(marble);
            }
        }
        let max_score = *players_score
            .iter()
            .max()
            .expect("No value in the player scores");
        Ok(max_score as ResultType)
    }
}
