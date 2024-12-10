use crate::reader::file_io::read_file;

pub fn solve_day_2() {
    let content = read_file("src/aoc_2024/day_2/input.txt");
    println!("PART 1 - Result: {}", process1(content.clone()));
    println!("PART 2 - Result: {}", process2(content));
}

fn process1(input: String) -> i64 {
    let content_lines: Vec<_> = input.lines().collect();
    let split_lines: Vec<Vec<&str>> = content_lines.iter().map(|x| x.split(" ").collect()).collect();
    // convert all &str to i64
    let number_lines = split_lines.iter().map(|x| x.iter().map(|y| y.parse::<i64>().unwrap()).collect::<Vec<i64>>()).collect::<Vec<Vec<i64>>>();

    let mut answer: i64 = 0;

    let _ = number_lines.iter().map(|x| {
        if is_ascending(x) || is_descending(x) {
            answer += 1;
        }
    }).collect::<Vec<()>>();

    answer
}

fn process2(input: String) -> i64 {
    let content_lines: Vec<_> = input.lines().collect();
    let split_lines: Vec<Vec<&str>> = content_lines.iter().map(|x| x.split(" ").collect()).collect();
    // convert all &str to i64
    let number_lines = split_lines.iter().map(|x| x.iter().map(|y| y.parse::<i64>().unwrap()).collect::<Vec<i64>>()).collect::<Vec<Vec<i64>>>();

    let mut answer: i64 = 0;

    let _ = number_lines.iter().map(|x| {
        if check_with_tolerance(x) {
            answer += 1;
        }
    }).collect::<Vec<()>>();
    answer
}


fn is_ascending(sequence: &Vec<i64>) -> bool {
    sequence.windows(2).all(|pair| pair[0] < pair[1] && pair[1] - pair[0] <= 3)
}

fn is_descending(sequence: &Vec<i64>) -> bool {
    sequence.windows(2).all(|pair| pair[0] > pair[1] && pair[0] - pair[1] <= 3)
}

fn check_with_tolerance(sequence: &Vec<i64>) -> bool {
    for i in 0..sequence.len() {
        let mut modified = sequence.clone();
        modified.remove(i);
        if is_ascending(&modified) || is_descending(&modified) {
            return true;
        }
    }
    false
}

