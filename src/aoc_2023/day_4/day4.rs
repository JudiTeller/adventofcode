use regex::Regex;
use crate::aoc_2023::day_4::day4_data::CardGame;
use crate::reader::file_io::read_file;

pub fn solve_day_4() {
    let content = read_file("src/aoc_2023/day_4/input.txt");
    println!("PART 1 - Result: {}", process1(content.clone()));
    println!("PART 2 - Result: {}", process2(content));
}

fn process1(input: String) -> i64 {
    let card_id_re: Regex = Regex::new(r"Card +\d+: ").unwrap();
    let card_number_re: Regex = Regex::new(r"\d+").unwrap();

    let mut result = 0;

    let mut lines = input.lines();
    let mut card_nr = 1;

    for line in lines {
        let mut line_result = 0;
        let mut winner_count = 0;
        let mut cleaned = card_id_re.replace_all(line, " ");

        println!("Cleaned: {}", cleaned);

        let split: Vec<_> = cleaned.split("|").collect();
        let winning_numbers: Vec<_> = card_number_re.find_iter(split[0]).filter_map(|mat| mat.as_str().parse::<i64>().ok()).collect();
        let guesses: Vec<_> = card_number_re.find_iter(split[1]).filter_map(|mat| mat.as_str().parse::<i64>().ok()).collect();

        println!("Winning numbers: {:?}", winning_numbers);
        println!("Guesses: {:?}", guesses);

        for card in guesses {
            if winning_numbers.contains(&card) {
                if line_result == 0 {
                    line_result = 1;
                    winner_count += 1;
                } else {
                    line_result *= 2;
                    winner_count += 1;
                }
            }
        }
        println!("Card No. {}: Winner count: {}, line result: {}\n\n", card_nr, winner_count, line_result);
        result += line_result;
        card_nr += 1;
    }

    result
}

fn process2(input: String) -> i64 {
    let mut deck = CardGame::new();

    let card_id_re: Regex = Regex::new(r"Card +\d+: ").unwrap();
    let card_number_re: Regex = Regex::new(r"\d+").unwrap();

    let mut result = 0;

    let mut lines = input.lines();
    let mut card_nr = 1;

    for line in lines {
        let mut line_result = 0;
        let mut winner_count = 0;

        let mut cleaned = card_id_re.replace_all(line, " ");

        println!("Cleaned: {}", cleaned);

        let split: Vec<_> = cleaned.split("|").collect();
        let winning_numbers: Vec<_> = card_number_re.find_iter(split[0]).filter_map(|mat| mat.as_str().parse::<i64>().ok()).collect();
        let guesses: Vec<_> = card_number_re.find_iter(split[1]).filter_map(|mat| mat.as_str().parse::<i64>().ok()).collect();

        println!("Winning numbers: {:?}", winning_numbers);
        println!("Guesses: {:?}", guesses);

        for card in guesses.clone() {
            if winning_numbers.contains(&card) {
                if line_result == 0 {
                    line_result = 1;
                    winner_count += 1;
                } else {
                    line_result *= 2;
                    winner_count += 1;
                }
            }
        }

        deck.add_card(card_nr, winning_numbers, guesses, line_result, winner_count);
        card_nr += 1;
    }

    card_nr = 1;

    for id in 1..(deck.cards.len() + 1) as i64 {
        // print current amount of card
        println!("\n\nCard No. {}: Amount: {}", id, deck.cards.get(&id).unwrap().amount);

        // 38759447
        let amount = deck.cards.get(&id).unwrap().amount;
        let matches = deck.cards.get(&id).unwrap().matches;

        for win in 1..matches + 1{
            deck.cards.get_mut(&(id + win)).unwrap().amount += amount;
        }
        result += amount;

    }

    result
}