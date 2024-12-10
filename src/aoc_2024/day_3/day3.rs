use regex::Regex;
use crate::reader::file_io::read_file;

pub fn solve_day_3() {
    let content = read_file("src/aoc_2024/day_3/input.txt");
    println!("PART 1 - Result: {}", crate::aoc_2024::day_3::day3::process1(content.clone()));
    println!("PART 2 - Result: {}", crate::aoc_2024::day_3::day3::process2(content));
}

fn process1(input: String) -> i64{
    // create a regex
    let re = Regex::new(r"mul\((?P<first>\d+),(?P<second>\d+)\)").unwrap();
    let mut answer: i64 = 0;

    // collect all matches in input
    let matches: Vec<_> = re.captures_iter(&input).collect();

    // extract the numbers from the named capture groups
    for cap in matches {
        let first = cap.name("first").unwrap().as_str().parse::<i64>().unwrap();
        let second = cap.name("second").unwrap().as_str().parse::<i64>().unwrap();
        answer += first * second;
    }

    answer
}

fn process2(input: String) -> i64{
    // create a regex
    let re = Regex::new(r"(?P<instruction>(mul)|(do)|(don't))\((?P<first>\d+)?,?(?P<second>\d+)?\)").unwrap();
    let mut answer: i64 = 0;

    // collect all matches in input
    let matches: Vec<_> = re.captures_iter(&input).collect();

    let mut enable = true;

    // extract the numbers from the named capture groups
    for cap in matches {
        let instruction = cap.name("instruction").unwrap().as_str();
        match instruction {
            "mul" => {
                if enable {
                    let first = cap.name("first").unwrap().as_str().parse::<i64>().unwrap();
                    let second = cap.name("second").unwrap().as_str().parse::<i64>().unwrap();
                    answer += first * second;
                }
            }
            _ => {
                if instruction == "do" {
                    enable = true;
                } else if instruction == "don't" {
                    enable = false;
                }
            }
        }



    }

    answer

}