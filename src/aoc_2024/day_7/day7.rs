use std::collections::{HashMap, HashSet};
use crate::reader::file_io::read_file;

pub fn solve_day_7() {
    let content = read_file("src/aoc_2024/day_7/input.txt");
    println!("PART 1 - Result: {}", process1(content.clone()));
    println!("PART 2 - Result: {}", process2(content));
}

fn process1(input: String) -> i64 {
    let mut answer = 0;
    let lines = input.lines().collect::<Vec<&str>>();


    for line in lines {
        let parts = line.split(": ").collect::<Vec<&str>>();
        // parse to i64
        let equation_result = parts[0].parse::<i64>().unwrap();
        let operands = parts[1].split(" ").collect::<Vec<&str>>().iter().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        // Collection for results of each step
        let mut memo: Vec<HashMap<i64, String>> = vec![HashMap::new(); operands.len()];

        memo[0].insert(operands[0], operands[0].to_string());

        for j in 1..operands.len() {
            let mut new_memo = HashMap::new();
            for (key, value) in memo[j - 1].iter() {
                let add = key + operands[j];
                let mul = key * operands[j];
                new_memo.insert(add, format!("{} + {}", value, operands[j]));
                new_memo.insert(mul, format!("{} * {}", value, operands[j]));
            }
            memo[j] = new_memo;
        }

        if let Some(equation) = memo.last().unwrap().get(&equation_result) {
            println!("Equation possible: {}", equation);
            answer += equation_result;
        }
    }


    answer
}

fn process2(input: String) -> i64 {
    let mut answer = 0;
    let lines = input.lines().collect::<Vec<&str>>();


    for line in lines {
        let parts = line.split(": ").collect::<Vec<&str>>();
        // parse to i64
        let equation_result = parts[0].parse::<i64>().unwrap();
        let operands = parts[1].split(" ").collect::<Vec<&str>>().iter().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        // do in reverse by subtracting and dividing
        // discard all results that are negative or have a remainder
        // path is correct if the last result is the first operand

        let mut current_values = vec![equation_result];

        for &current in operands.iter().skip(1).rev() {
            let mut next_values= Vec::new();

            for &value in &current_values {
                next_values.push(value - current);

                if current != 0 && value % current == 0 {
                    next_values.push(value / current);
                }

                let value_str = value.to_string();
                let current_str = current.to_string();
                if value_str.ends_with(&current_str) {
                    if let Ok(remaining) = value_str[..value_str.len() - current_str.len()].parse::<i64>() {
                        next_values.push(remaining);
                    }
                }
            }
            current_values = next_values;
        }

        if current_values.contains(&operands[0]) {
            answer += equation_result;
        }
    }


    answer
}

