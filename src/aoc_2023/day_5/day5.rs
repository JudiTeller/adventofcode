use regex::Regex;
use crate::aoc_2023::day_5::day5_data::{SeedMapping, SeedMappingRanged};
use crate::reader::file_io::read_file;

pub fn solve_day_5() {
    let content = read_file("src/aoc_2023/day_5/input.txt");
    println!("PART 1 - Result: {}", process1(content.clone()));
    println!("PART 2 - Result: {}", process2(content.clone()));
}


fn process1(input: String) -> i64 {
    let seed_re = Regex::new(r"seeds:( \d+)+").unwrap();
    let mapping_re = Regex::new(r"\w+-to-\w+ map:\n((\d+ *)+\n?)+").unwrap();
    let number_re = Regex::new(r"\d+").unwrap();

    let seed_numbers = number_re.find_iter(seed_re.find(&input).unwrap().as_str()).filter_map(|mat| mat.as_str().parse::<i64>().ok()).collect::<Vec<i64>>();

    // collect all matches for the mapping regex
    let mut mappings = Vec::new();

    let mut seeds: Vec<SeedMapping> = Vec::new();

    for seed in seed_numbers {
        let new_seed = SeedMapping {
            idx: seed,
        };
        seeds.push(new_seed);
    }

    for mat in mapping_re.find_iter(&input) {
        let mat = mat.as_str();
        let mut mapper = Vec::new();
        // take every 3 matches and put them into a vector
        for mat in number_re.find_iter(mat).filter_map(|mat| mat.as_str().parse::<i64>().ok()).collect::<Vec<i64>>().chunks(3) {
            mapper.push(mat.to_vec());
        }
        mappings.push(mapper);
    }

    println!("Mappings: {:?}", mappings);

    for current in 0..mappings.len() {
        for mapper in mappings[current].iter() {
            for seed in seeds.iter_mut() {
                if seed.idx >= mapper[1] &&
                    seed.idx < mapper[1] + mapper[2] {
                    seed.idx = mapper[0] + seed.idx - mapper[1];
                }
            }
        }
    }

    for seed in seeds.iter() {
        println!("Seed: {:?}", seed);
    }

    let lowest = seeds.iter().min_by_key(|seed| seed.idx).unwrap().idx;
    lowest
}

fn process2(input: String) -> i64 {
    let seed_re = Regex::new(r"seeds:( \d+)+").unwrap();
    let mapping_re = Regex::new(r"\w+-to-\w+ map:\n((\d+ *)+\n?)+").unwrap();
    let number_re = Regex::new(r"\d+").unwrap();

    let seed_numbers = number_re.find_iter(seed_re.find(&input).unwrap().as_str()).filter_map(|mat| mat.as_str().parse::<i64>().ok()).collect::<Vec<i64>>();

    // collect all matches for the mapping regex
    let mut mappings = Vec::new();

    let mut seeds: Vec<SeedMappingRanged> = Vec::new();

    for i in ((0..seed_numbers.len()).step_by(2)).rev() {
        let range = seed_numbers[i + 1];


        let new_seed = SeedMappingRanged {
            start: seed_numbers[i],
            range,
            idx: [seed_numbers[i], 0, 0, 0, 0, 0, 0, 0],
        };
        seeds.push(new_seed);

    }

    println!("test");

    for mat in mapping_re.find_iter(&input) {
        let mat = mat.as_str();
        let mut mapper = Vec::new();
        // take every 3 matches and put them into a vector
        for mat in number_re.find_iter(mat).filter_map(|mat| mat.as_str().parse::<i64>().ok()).collect::<Vec<i64>>().chunks(3) {
            mapper.push(mat.to_vec());
        }
        mappings.push(mapper);
    }

    // sort mappers by start value
    for mapper in mappings.iter_mut() {
        mapper.sort_by(|a, b| a[0].cmp(&b[0]));
    }

    let mut lowest = i64::MAX;

    for current in 0..mappings.len() {
        for mapper in mappings[current].iter() {

        }
    }

    let mut mapvals: Vec<i64> = Vec::new();

    // iterate over mappings
    for mappers in mappings.last().iter() {
        let mut counter = 0;
    }



    lowest
}


