use crate::aoc_2023::day_7::day7_data::{Hand, Hand2};
use crate::reader::file_io::read_file;

pub fn solve_day_7() {
    let content = read_file("src/aoc_2023/day_7/input.txt");
    println!("PART 1 - Result: {}", process1(content.clone()));
    println!("PART 2 - Result: {}", process2(content));
}


fn process1(input: String) -> i64 {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut hands: Vec<Hand> = Vec::new();

    let mut result = 0;

    for line in lines {
        let split = line.split(" ").collect::<Vec<&str>>();

        let card_string = split[0];
        let bet = split[1].parse::<i64>().unwrap();

        hands.push(Hand::new(card_string, bet));
    }

    // sort hands by comparing
    hands.sort_by(|a, b| a.compare(b));

    // get the index + 1 of each hand
    for (idx, hand) in hands.iter().enumerate() {
        result += hand.bet * (idx + 1) as i64;
    }


    // println!("Hands: {:#?}", hands);
    result
}



fn process2(input: String) -> i64 {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut hands: Vec<Hand2> = Vec::new();

    let mut result = 0;

    for line in lines {
        let split = line.split(" ").collect::<Vec<&str>>();

        let card_string = split[0];
        let bet = split[1].parse::<i64>().unwrap();

        hands.push(Hand2::new(card_string, bet));
    }

    // sort hands by comparing
    hands.sort_by(|a, b| a.compare(b));

    // get the index + 1 of each hand
    for (idx, hand) in hands.iter().enumerate() {
        result += hand.bet * (idx + 1) as i64;
    }


    // println!("Hands: {:#?}", hands);
    result
}
