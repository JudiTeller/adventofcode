use crate::reader::file_io::read_file;
use regex::Regex;
use std::collections::HashMap;
use std::ops::Add;

pub fn solve_day_12() {
    let input = read_file("src/aoc_2023/day_12/testinput.txt");

    println!("Day 10, Part 1: {}", process1(input.clone()));
    println!("Day 10, Part 2: {}", process2(input));
}

// ???.### 1,1,3
// ? can be . or #
// .??.### or #??.###
// ..?.###, .#?.###, #.?.###, ##?.###
// ....###, ..#.###, .#..###, .##.###, #...###, #.#.###, ##..###, ###.###
// iterate through all possible combinations of . and #, and check if they match the rules

fn process1(input: String) -> i64 {
    let lines = input.split("\n").collect::<Vec<&str>>();

    let cells_re = Regex::new(r"[.?#]+").unwrap();
    let rule_re = Regex::new(r"[\d,]+").unwrap();

    let mut result = 0;
    // let mut counter = 0;
    // let total = lines.len();

    let mut permu_map: HashMap<String, Vec<Vec<i64>>> = HashMap::new();

    for line in lines {
        let cells = cells_re.find(line).unwrap().as_str();
        let ruleset = rule_re.find(line).unwrap().as_str();
        // collect digits from ruleset, parse to i64, and store in vector
        let rule = ruleset
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        create_permutations(cells.to_string(), &mut permu_map);

        result += permu_map
            .get(cells)
            .unwrap()
            .iter()
            .filter(|x| x == &&rule)
            .count() as i64;
        // counter += 1;
    }

    result
}

fn create_permutations(
    input: String,
    perm_map: &mut HashMap<String, Vec<Vec<i64>>>,
) -> Vec<Vec<i64>> {
    let mut permutations = Vec::new();
    // let line = input.clone();

    if perm_map.contains_key(&input) {
        permutations = perm_map.get(&input).unwrap().clone();
    } else if input.contains('?') {
        // replace first ? with . and recurse
        let mut line = input.clone();
        let index = line.find('?').unwrap();
        line.replace_range(index..index + 1, ".");
        let perms1 = create_permutations(line.clone(), perm_map);
        // replace same index with # and recurse
        line.replace_range(index..index + 1, "#");
        let perms2 = create_permutations(line.clone(), perm_map);

        // combine permutations
        permutations.extend(perms1);
        permutations.extend(perms2);

        // add permutation to perm_map
        perm_map.insert(input.clone(), permutations.clone());
    } else {
        let mut permutation = Vec::new();
        let mut counter = 0;

        for c in input.chars() {
            if c == '#' {
                counter += 1;
            } else if c == '.' && counter > 0 {
                permutation.push(counter);
                counter = 0;
            }
        }
        if counter > 0 {
            permutation.push(counter);
        }
        // add permutation to perm_map
        permutations.push(permutation);
        perm_map.insert(input.clone(), permutations.clone());
    }

    permutations
}

fn process2(input: String) -> i64 {
    let lines = input.split("\n").collect::<Vec<&str>>();

    let cells_re = Regex::new(r"[.?#]+").unwrap();
    let rule_re = Regex::new(r"[\d,]+").unwrap();

    let mut result = 0;
    let mut counter = 0;
    let total = lines.len();

    let mut permu_map: HashMap<String, Vec<Vec<i64>>> = HashMap::new();

    for line in lines {
        let cells = cells_re.find(line).unwrap().as_str();
        let ruleset = rule_re.find(line).unwrap().as_str();
        // collect digits from ruleset, parse to i64, and store in vector
        let rule = ruleset
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let mut extended_cells: String = String::from("");
        let mut extended_rule: Vec<i64> = Vec::new();

        for i in 0..5 {
            if i > 0 {
                extended_cells = extended_cells.clone().add("?");
            }
            extended_cells = extended_cells.clone().add(cells);

            extended_rule.extend(rule.clone());
        }

        create_permutations(extended_cells.to_string(), &mut permu_map);

        result += permu_map
            .get(&extended_cells)
            .unwrap()
            .iter()
            .filter(|x| x == &&extended_rule)
            .count() as i64;
        counter += 1;
        println!("{} / {}", counter, total);
    }

    result
}
