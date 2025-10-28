use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    // Implement for problem
    events: Vec<(u32, u32, u32, u32, u32, String)>,
}
impl Solution {
    fn add_event(&mut self, year: u32, month: u32, day: u32, hour: u32, minute: u32, event: &str) {
        // Implement for problem
        self.events
            .push((year, month, day, hour, minute, event.to_string()));
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        let regex = regex_lite::Regex::new(r"^\[(?<year>\d+)-(?<month>\d+)-(?<day>\d+) (?<hour>\d+):(?<minute>\d+)\] (?<event>.+)$").unwrap();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            let line = line.trim();
            let captures = regex.captures(line).unwrap();
            let year: u32 = captures.name("year").unwrap().as_str().parse().unwrap();
            let month: u32 = captures.name("month").unwrap().as_str().parse().unwrap();
            let day: u32 = captures.name("day").unwrap().as_str().parse().unwrap();
            let hour: u32 = captures.name("hour").unwrap().as_str().parse().unwrap();
            let minute: u32 = captures.name("minute").unwrap().as_str().parse().unwrap();
            let event = captures.name("event").unwrap().as_str();
            debug!(
                "Line {}: year={}, month={}, day={}, hour={}, minute={}, event={}",
                id, year, month, day, hour, minute, event
            );
            // Implement for problem
            solution.add_event(year, month, day, hour, minute, event);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {
        self.events.sort();
    }

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut current_guard: Option<String> = None;
        let mut is_asleep = false;
        let mut sleep_start_minute: u32 = 0;
        let mut sleep_start_hour: u32 = 0;
        let mut counter = std::collections::HashMap::<String, u32>::new();
        let mut occurances =
            std::collections::HashMap::<String, std::collections::HashMap<u32, u32>>::new();
        for event in &self.events {
            let (_year, _month, _day, hour, minute, description) = event;
            if description.starts_with("Guard #") {
                // New guard begins shift
                let parts: Vec<&str> = description.split_whitespace().collect();
                current_guard = Some(parts[1].trim_start_matches('#').to_string());
                is_asleep = false;
            } else if description == "falls asleep" {
                // Guard falls asleep
                is_asleep = true;
                sleep_start_minute = *minute;
                sleep_start_hour = *hour;
            } else if description == "wakes up" {
                // Guard wakes up
                if is_asleep {
                    is_asleep = false;
                    let mut sleep_end_minute = *minute;
                    let mut sleep_end_hour = *hour;
                    // Record sleep period for current_guard from sleep_start_minute to sleep_end_minute
                    info!(
                        "Guard {:?} slept from {} to {}",
                        current_guard, sleep_start_minute, sleep_end_minute
                    );
                    *counter.entry(current_guard.clone().unwrap()).or_insert(0) +=
                        sleep_end_minute - sleep_start_minute;
                    loop {
                        *occurances
                            .entry(current_guard.clone().unwrap())
                            .or_default()
                            .entry(sleep_end_minute)
                            .or_insert(0) += 1;
                        if sleep_start_minute == sleep_end_minute
                            && sleep_start_hour == sleep_end_hour
                        {
                            break;
                        }
                        if sleep_end_minute == 0 {
                            sleep_end_minute = 59;
                            sleep_end_hour -= 1;
                        } else {
                            sleep_end_minute -= 1;
                        }
                    }
                } else {
                    panic!("Guard {:?} wakes up but was not asleep", current_guard);
                }
            }
        }
        info!("Sleep counter: {:?}, occurances {:?}", counter, occurances);
        let m = counter.iter().max_by_key(|v1| v1.1).unwrap();
        info!("Sleepiest guard is {:?} with {} minutes", m.0, m.1);
        let n = occurances
            .get(m.0)
            .unwrap()
            .iter()
            .max_by_key(|v| v.1)
            .unwrap();
        info!("Guard {:?} slept most at {}", m.0, n.0); // Implement for problem
        Ok(m.0.parse::<ResultType>().unwrap() * *n.0 as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut current_guard: Option<String> = None;
        let mut is_asleep = false;
        let mut sleep_start_minute: u32 = 0;
        let mut sleep_start_hour: u32 = 0;
        let mut counter = std::collections::HashMap::<String, u32>::new();
        let mut occurances =
            std::collections::HashMap::<String, std::collections::HashMap<u32, u32>>::new();
        for event in &self.events {
            let (_year, _month, _day, hour, minute, description) = event;
            if description.starts_with("Guard #") {
                // New guard begins shift
                let parts: Vec<&str> = description.split_whitespace().collect();
                current_guard = Some(parts[1].trim_start_matches('#').to_string());
                is_asleep = false;
            } else if description == "falls asleep" {
                // Guard falls asleep
                is_asleep = true;
                sleep_start_minute = *minute;
                sleep_start_hour = *hour;
            } else if description == "wakes up" {
                // Guard wakes up
                if is_asleep {
                    is_asleep = false;
                    let mut sleep_end_minute = *minute;
                    let mut sleep_end_hour = *hour;
                    // Record sleep period for current_guard from sleep_start_minute to sleep_end_minute
                    info!(
                        "Guard {:?} slept from {} to {}",
                        current_guard, sleep_start_minute, sleep_end_minute
                    );
                    *counter.entry(current_guard.clone().unwrap()).or_insert(0) +=
                        sleep_end_minute - sleep_start_minute;
                    loop {
                        *occurances
                            .entry(current_guard.clone().unwrap())
                            .or_default()
                            .entry(sleep_end_minute)
                            .or_insert(0) += 1;
                        if sleep_start_minute == sleep_end_minute
                            && sleep_start_hour == sleep_end_hour
                        {
                            break;
                        }
                        if sleep_end_minute == 0 {
                            sleep_end_minute = 59;
                            sleep_end_hour -= 1;
                        } else {
                            sleep_end_minute -= 1;
                        }
                    }
                } else {
                    panic!("Guard {:?} wakes up but was not asleep", current_guard);
                }
            }
        }
        let r = occurances
            .iter()
            .map(|(guard, minutes)| {
                let m = minutes.iter().max_by_key(|v| v.1).unwrap();
                info!("Guard {:?} slept most at {} ({} times)", guard, m.0, m.1);
                (
                    guard.parse::<ResultType>().unwrap(),
                    *m.0 as ResultType,
                    *m.1 as ResultType,
                )
            })
            .max_by_key(|v| v.2)
            .unwrap();
        Ok(r.0 * r.1 as ResultType) // Implement for problem
    }
}
