use std::collections::{HashMap, HashSet};
use crate::reader::file_io::read_file;

pub fn solve_day_5() {
    let content = read_file("src/aoc_2024/day_5/input.txt");
    println!("PART 1 - Result: {}", process1(content.clone()));
    println!("PART 2 - Result: {}", process2(content));
}

fn process1(input: String) -> u64 {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let instruction_strings = parts[0].lines().collect::<Vec<&str>>();
    let instructions = parse_instruction(instruction_strings);

    let update_strings = parts[1].lines().collect::<Vec<&str>>();
    let updates = parse_updates(update_strings);

    let mut orderValidator = OrderValidator::new();
    for instruction in instructions {
        orderValidator.add_dependency(instruction.0, instruction.1);
    }

    let mut answer: u64 = 0;

    for update in updates {
        if orderValidator.validate_sequence(&update) {
            // find the middle number of the sequence, all sequences are of odd length
            let middle = update.len() / 2;
            answer += update[middle] as u64;
        }
    }

    answer
}

fn process2(input: String) -> u64 {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let instruction_strings = parts[0].lines().collect::<Vec<&str>>();
    let instructions = parse_instruction(instruction_strings);

    let update_strings = parts[1].lines().collect::<Vec<&str>>();
    let updates = parse_updates(update_strings);

    let mut orderValidator = OrderValidator::new();
    for instruction in instructions {
        orderValidator.add_dependency(instruction.0, instruction.1);
    }

    let mut answer: u64 = 0;

    for update in updates {
        if !orderValidator.validate_sequence(&update) {
            // find the middle number of the sequence, all sequences are of odd length
            let sorted = orderValidator.sort_sequence(update);
            let middle = sorted.len() / 2;
            answer += sorted[middle] as u64;
        }
    }

    answer
}

fn parse_instruction(instruction_strings: Vec<&str>) -> Vec<(u8, u8)> {
    let mut instructions: Vec<(u8, u8)> = Vec::new();

    for istring in instruction_strings {
        let parts = istring.split('|').collect::<Vec<&str>>();
        let x = parts[0].parse::<u8>().unwrap();
        let y = parts[1].parse::<u8>().unwrap();
        instructions.push((x, y));
    }

    return instructions;
}

fn parse_updates(update_strings: Vec<&str>) -> Vec<Vec<u8>> {
    let mut updates: Vec<Vec<u8>> = Vec::new();

    for ustring in update_strings {
        let parts = ustring.split(',').collect::<Vec<&str>>();
        let mut update: Vec<u8> = Vec::new();
        for part in parts {
            update.push(part.parse::<u8>().unwrap());
        }
        updates.push(update);
    }

    return updates;
}


#[derive(Default)]
struct OrderValidator {
    graph: HashMap<u8, HashSet<u8>>,
    reverse_graph: HashMap<u8, HashSet<u8>>,
    cache: HashMap<(u8, u8), bool>
}

impl OrderValidator {
    fn new() -> Self {
        Self::default()
    }

    fn add_dependency(&mut self, before: u8, after: u8) {
        self.graph.entry(before).or_default().insert(after);
        self.graph.entry(after).or_default();

        self.reverse_graph.entry(after).or_default().insert(before);
        self.reverse_graph.entry(before).or_default();
    }

    fn check_order(&self, number1: u8, number2: u8) -> bool {
        self.graph
            .get(&number1)
            .map(|neighbours| neighbours.contains(&number2))
            .unwrap_or(false)
    }

    fn validate_sequence(&self, sequence: &Vec<u8>) -> bool {
        for (i, &num1) in sequence.iter().enumerate() {
            for &num2 in sequence.iter().skip(i+1) {
                if self.check_order(num2, num1) {
                    return false;
                }
            }
        }

        true
    }
    fn calculate_sequence_specific_in_degree(&self, sequence: &[u8]) -> HashMap<u8, usize> {
        let sequence_set: HashSet<u8> = sequence.iter().copied().collect();

        // Only calculate in-degree from nodes that exist in this sequence
        sequence.iter()
            .map(|&node| {
                let count = self.reverse_graph
                    .get(&node)
                    .map(|deps| {
                        // Only count incoming edges from nodes that are in this sequence
                        deps.iter().filter(|&&dep| sequence_set.contains(&dep)).count()
                    })
                    .unwrap_or(0);
                (node, count)
            })
            .collect()
    }

    fn sort_sequence(&self, sequence: Vec<u8>) -> Vec<u8> {
        let mut sorted: Vec<u8> = Vec::new();
        let mut available: HashSet<u8> = sequence.iter().cloned().collect();

        let mut in_degree = self.calculate_sequence_specific_in_degree(&sequence);

        while !available.is_empty() {
            let mut ready: Vec<u8> = available.iter()
                .filter(|&&node| in_degree.get(&node).map_or(true, |&count| count == 0))
                .copied()
                .collect();

            ready.sort_by_key(|&num| {
                sequence.iter()
                    .position(|&x| x == num)
                    .unwrap_or(usize::MAX);
            });

            let next = ready[0];
            sorted.push(next);
            available.remove(&next);

            if let Some(deps) = self.graph.get(&next) {
                for &dep in deps {
                    if let Some(count) = in_degree.get_mut(&dep) {
                        *count = count.saturating_sub(1)
                    }
                }
            }
        }
        sorted
    }
}