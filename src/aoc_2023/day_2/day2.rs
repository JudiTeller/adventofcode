use std::collections::HashMap;
use regex::Regex;
use crate::reader::file_io::read_file;

pub fn solve_day_2() {
    let content = read_file("src/aoc_2023/day_2/input.txt");
    println!("PART 1 - Result: {}", process1(content.clone()));
    println!("PART 2 - Result: {}", process2(content));
}


fn process1(input: String) -> i64 {
    let amounts: HashMap<&str, i64> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

    let game_re: Regex = Regex::new(r"Game \d+: ").unwrap();
    let game_id_re: Regex = Regex::new(r"\d+").unwrap();

    let lines: Vec<_> = input.lines().collect();

    let mut result: i64 = 0;

    // iterate over each line in input, each line represents a single game
    for line in lines {
        let mut valid: bool = true;

        // get the game id from the line
        let game_id = (game_id_re.find(line).unwrap().as_str()).parse::<i64>().unwrap();

        // remove the game header from the line
        let line_clean = game_re.replace_all(line, " ");

        // split the line into groups, each group is a round of the current game
        let line_groups: Vec<_> = line_clean.split(";").collect();

        //  iterate over each group in the line, each group is a round of the current game
        'outer: for group in line_groups {
            // clean up the group, remove whitespace and split into parts
            let group_parts: Vec<_> = group.split(",").collect();

            // separate each part into its color and amount and check if the amount is valid
            for part in group_parts {
                let part_parts: Vec<_> = part.trim().split(" ").collect();

                let  maybe_number = part_parts[0].parse::<i64>();

                match maybe_number {
                    Ok(num) => {
                        // check the corresponding amount in the hashmap
                        if num > amounts[part_parts[1]] {
                            valid = false;
                            break 'outer;
                        }
                    },
                    Err(_) => println!("{} is not a number", part_parts[1])
                }


            }
        }

        // if the game is valid, add its id to the result
        if valid {
            result += game_id;
        }

    }

    return result;
}


// searches the fewest possible amount of each color to complete a game and multiplies them
fn process2(input: String) -> i64 {
    let game_re: Regex = Regex::new(r"Game \d+: ").unwrap();

    let lines: Vec<_> = input.lines().collect();

    let mut result: i64 = 0;

    // iterate over each line in input, each line represents a single game
    for line in lines {

        let mut amounts: HashMap<&str, i64> = HashMap::new();

        // remove the game header from the line
        let line_clean = game_re.replace_all(line, " ");

        // split the line into groups, each group is a round of the current game
        let line_groups: Vec<_> = line_clean.split(";").collect();


        // iterate over each group in the line
        for group in line_groups {
            // clean up the group, remove whitespace and split into parts
            // each part is a pulled color and its amount
            let group_parts: Vec<_> = group.trim().split(",").collect();

            // separate each part into its color and amount and collect it into a hashmap
            for part in group_parts {
                let part_parts: Vec<_> = part.trim().split(" ").collect();

                let  maybe_number = part_parts[0].parse::<i64>();

                match maybe_number {
                    Ok(num) => {
                        if !amounts.contains_key(part_parts[1]) {
                            amounts.insert(part_parts[1], num);
                        }
                        else if num > amounts[part_parts[1]] {
                            amounts.insert(part_parts[1], num);
                        }
                    },
                    Err(_) => println!("{} is not a number", part_parts[1])
                }


            }
        }

        let mut temp = 1;

        // multiply the amounts of each color together
        for key in amounts.keys() {
            temp *= amounts[key];
        }

        result += temp;

    }

    return result;
}