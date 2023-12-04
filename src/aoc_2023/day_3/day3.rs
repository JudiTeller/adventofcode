use regex::{Matches, Regex};
use crate::reader::file_io::read_file;

pub fn solve_day_3() {
    let content = read_file("src/aoc_2023/day_3/input.txt");
    let result = process1(content.clone());
    println!("PART 1 - Result: {}", result.0);
    println!("PART 2 - Result: {}", result.1);
}

fn process1(input: String) -> (i64, i64) {
    let mut lines: Vec<_> = input.lines().collect();
    let mut lines_cleaned: Vec<String> = Vec::new();

    let mut result1: Vec<i64> = Vec::new();
    let mut result2: Vec<i64> = Vec::new();
    for mut line in lines {
        let cleaned = line.replace(".", " ");
        lines_cleaned.push(cleaned.to_owned());
    }


    for line_index in 0..lines_cleaned.len() {

        println!("");
        for char_index in 0..lines_cleaned[line_index].len() {
            let mut char = lines_cleaned[line_index].chars().nth(char_index).unwrap();
            let mut ret: Vec<i64> = Vec::new();
            if !char.is_digit(10) && !char.is_whitespace() {
                ret = check_surrounding(lines_cleaned.clone().to_owned(),
                                            line_index,
                                            char_index);

                result1.append(&mut ret.clone());

                if char == '*' && ret.len() == 2 {
                    result2.push(ret.iter().product());
                }
            }

        }

    }


    return (result1.iter().sum(), result2.iter().sum());
}

// function that checks cardinals and diagonals of a given coordinate
// if there is a digit, collect the whole number
fn check_surrounding(lines: Vec<String>, line_index:usize, index: usize) -> Vec<i64> {
    let re = Regex::new(r"\d+").unwrap();
    let mut result: Vec<i64> = Vec::new();
    let line_size = lines[line_index].len();

    if line_index > 0 {
        let mut upper = lines[line_index - 1].to_owned();
        let mut upper_numbers = re.find_iter(&upper);
        result.append(&mut check_matches(upper_numbers, index, line_size, &re));

    }

    if line_index < lines.len() - 1 {
        let mut lower = lines[line_index + 1].to_owned();
        let mut lower_numbers = re.find_iter(&lower);
        result.append(&mut check_matches(lower_numbers, index, line_size, &re));
    }

    let mut line = lines[line_index].to_owned();
    let mut line_numbers = re.find_iter(&line);

    result.append(&mut check_matches(line_numbers, index, line_size, &re));

    result

}

fn check_matches(line: Matches, index: usize, line_size: usize, re: &Regex) -> (Vec<i64>) {
    let mut result: Vec<i64> = Vec::new();

    let left_bound = if index <= 3 {3 - index} else {0};
    let right_bound = if index <= line_size - 1 {0} else {1};

    for mat in line {
        if mat.start() >= index - (3 - left_bound) && mat.start() <= index + (1 - right_bound)
            && mat.end() >= index && mat.end() <= index + 4{
            result.push(mat.as_str().parse::<i64>().unwrap());
            // re.replace(&mut mat.as_str().to_owned(), " ");
        }
    }

    result
}

fn process2(input: String) -> i64 {
    0
}