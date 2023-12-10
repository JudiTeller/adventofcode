use std::collections::HashMap;
use std::num::ParseIntError;
use std::ops::Add;
use crate::reader::file_io::read_file;


const DINGUSES: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"
];

pub fn solve_day_1() {
    let content = read_file("src/aoc_2023/day_1/michel.txt");
    println!("PART 1 - Result: {}", process1(content.clone()));
    println!("PART 2 - Result: {}", process2(content));
}

fn process1(input: String) -> i64 {
    let content_lines: Vec<_> = input.lines().collect();
    let mut answer: i64 = 0;

    for line in content_lines {
        let mut parts = "".to_string();

        let iter = line.chars();

        for c in iter.clone() {
            if c.is_digit(10){
                parts = parts.add(c.to_string().as_ref());
                break;
            }
        }

        for c in iter.rev() {
            if c.is_digit(10){
                parts = parts.add(c.to_string().as_ref());
                break;
            }
        }
        let maybe_number = parts.parse::<i64>();

        match maybe_number {
            Ok(num) => answer += num,
            Err(_) => println!("number of line '{}' could not be parsed", line)
        }
    }


    return answer;
}

fn process2(input: String) -> i64 {
    let content_lines: Vec<_> = input.lines().collect();
    let mut answer: i64 = 0;

    for line in content_lines {

        let mut parts = "".to_string();

        // find first and last substring from DINGUSES in line
        let  matches: HashMap<_,_> = DINGUSES.iter().flat_map(|dingus| line.match_indices(dingus)).collect();

        // take first and last key from matches and add to parts

        let first = matches.keys().min().unwrap();
        let last = matches.keys().max().unwrap();

        parts = parts.add(match_cap(matches.get(first).unwrap()).unwrap());
        parts = parts.add(match_cap(matches.get(last).unwrap()).unwrap());



        let maybe_number = parts.parse::<i64>();

        match maybe_number {
            Ok(num) => {
                answer += num.clone();

            },
            Err(_) => println!("number of line '{}' could not be parsed", line)
        }
    }


    return answer;
}

// match a single capture
fn match_cap(cap: &str) -> Result<&str, ParseIntError> {
    match cap {
        "one" => Ok("1"),
        "two" => Ok("2"),
        "three" => Ok("3"),
        "four" => Ok("4"),
        "five" => Ok("5"),
        "six" => Ok("6"),
        "seven" => Ok("7"),
        "eight" => Ok("8"),
        "nine" => Ok("9"),
        _ => Ok(cap)
    }
}