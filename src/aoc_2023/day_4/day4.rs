use regex::Regex;
use crate::reader::file_io::read_file;

pub fn solve_day_4() {
    let content = read_file("src/aoc_2023/day_3/input.txt");
    let result = process1(content.clone());
    println!("PART 1 - Result: {}", process1(content.clone()));
    println!("PART 2 - Result: {}", process2(content));
}

fn process1(input: String) -> i64 {
    let card_id_re: Regex = Regex::new(r"Card \d+: ").unwrap();
    let card_number_re: Regex = Regex::new(r"\d+").unwrap();


    let mut lines = input.lines();

    for line in lines {
        let mut cleaned = card_id_re.replace_all(line, " ");

        let split: Vec<_> = cleaned.split("|").collect();
        let winners = card_number_re.find_iter(split[0]).collect::<Vec<_>>();
        let cards = card_number_re.find_iter(split[1]).collect::<Vec<_>>();


    }

    0
}

fn process2(input: String) -> i64 {


    0
}