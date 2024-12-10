use crate::reader::file_io::read_file;
use itertools::Itertools;

pub fn solve_day_1() {
    let content = read_file("src/aoc_2024/day_1/input.txt");
    println!("PART 1 - Result: {}", process1(content.clone()));
    println!("PART 2 - Result: {}", process2(content));
}

fn process1(input: String) -> i64 {
    let content_lines: Vec<_> = input.lines().collect();
    let mut answer: i64 = 0;

    let mut firsts: Vec<i64> = vec![];
    let mut seconds: Vec<i64> = vec![];

    for line in content_lines {
        let pair: Vec<&str> = line.split("   ").collect();
        firsts.push(pair[0].parse::<i64>().unwrap());
        seconds.push(pair[1].parse::<i64>().unwrap())
    }

    firsts.sort();
    seconds.sort();

    for i in 0..firsts.len() {
        let distance = (firsts[i] - seconds[i]).abs();
        answer += distance;
    }

    println!("{}", answer);
    answer
}

fn process2(input: String) -> i64 {
    let content_lines: Vec<_> = input.lines().collect();
    let mut answer: i64 = 0;

    let mut firsts: Vec<i64> = vec![];
    let mut seconds: Vec<i64> = vec![];

    for line in content_lines {
        let pair: Vec<&str> = line.split("   ").collect();
        firsts.push(pair[0].parse::<i64>().unwrap());
        seconds.push(pair[1].parse::<i64>().unwrap())
    }

    let second_counts = seconds.into_iter().counts();

    for i in 0..firsts.len() {
        let number = firsts[i];
        let amount = second_counts.get(&number);

        match amount {
            None => {}
            Some(x) => {
                answer += (*x as i64) * number;
            }
        }
    }

    println!("{}", answer);
    answer
}
