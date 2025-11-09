use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
};
use tracing::error;
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum OpCode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}
impl OpCode {
    fn execute(&self, registers: &[usize], a: usize, b: usize, c: usize) -> Vec<usize> {
        let mut result = registers.to_vec();
        match self {
            OpCode::Addr => {
                result[c] = registers[a] + registers[b];
            }
            OpCode::Addi => {
                result[c] = registers[a] + b;
            }
            OpCode::Mulr => {
                result[c] = registers[a] * registers[b];
            }
            OpCode::Muli => {
                result[c] = registers[a] * b;
            }
            OpCode::Banr => {
                result[c] = registers[a] & registers[b];
            }
            OpCode::Bani => {
                result[c] = registers[a] & b;
            }
            OpCode::Borr => {
                result[c] = registers[a] | registers[b];
            }
            OpCode::Bori => {
                result[c] = registers[a] | b;
            }
            OpCode::Setr => {
                result[c] = registers[a];
            }
            OpCode::Seti => {
                result[c] = a;
            }
            OpCode::Gtir => result[c] = if a > registers[b] { 1 } else { 0 },
            OpCode::Gtri => result[c] = if registers[a] > b { 1 } else { 0 },
            OpCode::Gtrr => result[c] = if registers[a] > registers[b] { 1 } else { 0 },
            OpCode::Eqir => result[c] = if a == registers[b] { 1 } else { 0 },
            OpCode::Eqri => result[c] = if registers[a] == b { 1 } else { 0 },
            OpCode::Eqrr => result[c] = if registers[a] == registers[b] { 1 } else { 0 },
        }
        result
    }
}

#[derive(Debug, Default)]
struct Sample {
    before: Vec<usize>,
    opcode: usize,
    a: usize,
    b: usize,
    c: usize,
    after: Vec<usize>,
}

#[derive(Debug, Default)]
pub struct Solution {
    samples: Vec<Sample>,
    program: Vec<(usize, usize, usize, usize)>,
}
impl Solution {
    fn add_sample(&mut self, sample: Sample) {
        self.samples.push(sample);
    }

    fn add_program_step(&mut self, opcode: usize, a: usize, b: usize, c: usize) {
        self.program.push((opcode, a, b, c));
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        let mut in_test_program = false;
        let mut num_blanks = 0;

        let mut before: Vec<usize>;
        let mut after: Vec<usize>;
        let mut sample = Sample::default();
        let r = regex_lite::Regex::new(
            r"^(?<mode>\w+):\s*\[(?<r0>\d+),\s*(?<r1>\d+),\s*(?<r2>\d+),\s*(?<r3>\d+)\]$",
        )
        .unwrap();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            let line = line.trim();
            if line.is_empty() {
                num_blanks += 1;
            } else if line.starts_with("Before: ") {
                num_blanks = 0;
                if let Some(capt) = r.captures(line) {
                    let r0 = capt.name("r0").unwrap().as_str().parse().unwrap();
                    let r1 = capt.name("r1").unwrap().as_str().parse().unwrap();
                    let r2 = capt.name("r2").unwrap().as_str().parse().unwrap();
                    let r3 = capt.name("r3").unwrap().as_str().parse().unwrap();
                    before = vec![r0, r1, r2, r3];
                    info!("before: {:?}", before);
                    sample.before = before;
                } else {
                    error!(line, "Failed to parse");
                }
            } else if line.starts_with("After: ") {
                num_blanks = 0;
                if let Some(capt) = r.captures(line) {
                    let r0 = capt.name("r0").unwrap().as_str().parse().unwrap();
                    let r1 = capt.name("r1").unwrap().as_str().parse().unwrap();
                    let r2 = capt.name("r2").unwrap().as_str().parse().unwrap();
                    let r3 = capt.name("r3").unwrap().as_str().parse().unwrap();
                    after = vec![r0, r1, r2, r3];
                    info!("after: {:?}", after);
                    sample.after = after;
                    solution.add_sample(sample);
                    sample = Sample::default();
                } else {
                    error!(line, "Failed to parse");
                }
            } else {
                if num_blanks > 1 {
                    in_test_program = true;
                }
                num_blanks = 0;
                if let [opcode, a, b, c, ..] = line
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
                    .as_slice()
                {
                    info!("code: {} {} {} {}", opcode, a, b, c);
                    if !in_test_program {
                        sample.opcode = *opcode;
                        sample.a = *a;
                        sample.b = *b;
                        sample.c = *c;
                    } else {
                        solution.add_program_step(*opcode, *a, *b, *c);
                    }
                } else {
                    error!(line, "Failed to parse");
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
        use OpCode::*;

        let mut r = 0;
        for sample in &self.samples {
            let mut possibilities = 0;
            for opcode in [
                Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir,
                Eqri, Eqrr,
            ] {
                let result = opcode.execute(&sample.before, sample.a, sample.b, sample.c);
                if result == sample.after {
                    debug!(
                        sample = debug(sample),
                        opcode = debug(opcode),
                        result = debug(result),
                        "candidate"
                    );
                    possibilities += 1;
                }
            }
            if possibilities >= 3 {
                r += 1;
            }
        }
        // Implement for problem
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        use OpCode::*;

        let mut mapping = HashMap::new();
        for sample in &self.samples {
            let mut possibilities = HashSet::new();
            for opcode in [
                Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir,
                Eqri, Eqrr,
            ] {
                let result = opcode.execute(&sample.before, sample.a, sample.b, sample.c);
                if result == sample.after {
                    possibilities.insert(opcode.clone());
                    debug!(
                        sample = debug(sample),
                        opcode = debug(opcode),
                        result = debug(result),
                        "candidate"
                    );
                }
            }
            match mapping.entry(sample.opcode) {
                std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                    let existing: &mut HashSet<OpCode> = occupied_entry.get_mut();
                    existing.retain(|v| possibilities.contains(v));
                }
                std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(possibilities);
                }
            }
        }
        debug!(mapping = debug(&mapping), "mapping");

        let mut changed;
        let mut reverse_mapping = HashMap::new();
        loop {
            changed = false;
            for (opcode, operations) in mapping.iter() {
                let unresolved = operations
                    .iter()
                    .filter(|operation| !reverse_mapping.contains_key(*operation))
                    .cloned()
                    .collect::<Vec<_>>();
                if unresolved.len() == 1 {
                    let operation = unresolved.first().cloned().unwrap();
                    reverse_mapping.insert(operation, opcode);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        let mapping = reverse_mapping.iter().fold(HashMap::new(), |mut acc, v| {
            acc.insert(**v.1, v.0.to_owned());
            acc
        });
        debug!(
            reverse_mapping = debug(&reverse_mapping),
            mapping = debug(&mapping),
            "mapping"
        );

        let mut registers = vec![0, 0, 0, 0];
        for (command, a, b, c) in &self.program {
            registers = mapping
                .get(command)
                .unwrap()
                .execute(&registers, *a, *b, *c);
        }
        debug!(registers = debug(&registers), "done");
        let r = registers.into_iter().next().unwrap();
        Ok(r as ResultType)
    }
}
