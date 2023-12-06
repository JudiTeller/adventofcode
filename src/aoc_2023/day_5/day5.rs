use regex::Regex;
use crate::aoc_2023::day_5::day5_data::{SeedMapping, SeedMappingRanged};
use crate::reader::file_io::read_file;

pub fn solve_day_5() {
    let content = read_file("src/aoc_2023/day_5/input.txt");
    // println!("PART 1 - Result: {}", process1(content.clone()));
    println!("PART 2 - Result: {}", process2(content));
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
            idx: [seed, 0, 0, 0, 0, 0, 0, 0],
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
                if seed.idx[current] >= mapper[1] &&
                    seed.idx[current] < mapper[1] + mapper[2] {
                    seed.idx[current + 1] = mapper[0] + seed.idx[current] - mapper[1];
                } else if seed.idx[current + 1] == 0 {
                    seed.idx[current + 1] = seed.idx[current];
                }
            }
        }
    }

    for seed in seeds.iter() {
        println!("Seed: {:?}", seed);
    }

    let lowest = seeds.iter().min_by_key(|seed| seed.idx[7]).unwrap().idx[7];
    lowest
}


fn process2(input: String) -> i64 {
    let seed_re = Regex::new(r"seeds:( \d+)+").unwrap();
    let mapping_re = Regex::new(r"\w+-to-\w+ map:\n((\d+ *)+\n?)+").unwrap();
    let number_re = Regex::new(r"\d+").unwrap();

    let seed_numbers = number_re.find_iter(seed_re.find(&input).unwrap().as_str()).filter_map(|mat| mat.as_str().parse::<i64>().ok()).collect::<Vec<i64>>();
    let mut seeds: Vec<SeedMappingRanged> = Vec::new();

    for i in ((0..seed_numbers.len()).step_by(2)).rev() {
        let range = seed_numbers[i + 1];
        let new_seed = SeedMappingRanged {
            start: seed_numbers[i],
            range: seed_numbers[i + 1],
            idx: [seed_numbers[i], 0, 0, 0, 0, 0, 0, 0],
        };
        seeds.push(new_seed);
    }

    println!("amount of seeds: {}", seeds.len());

    // collect all matches for the mapping regex
    let mut mappings = Vec::new();


    for mat in mapping_re.find_iter(&input) {
        let mat = mat.as_str();
        let mut mapper = Vec::new();
        // take every 3 matches and put them into a vector
        for mat in number_re.find_iter(mat).filter_map(|mat| mat.as_str().parse::<i64>().ok()).collect::<Vec<i64>>().chunks(3) {
            mapper.push(mat.to_vec());
        }
        mappings.push(mapper);
    }



    for current in 0..mappings.len() {
        let mut new_vec: Vec<SeedMappingRanged> = Vec::new();
        println!("current map: {}", current);

        let mut seed_counter = 0;
        for seed in seeds.iter() {
            seed_counter += 1;

            let mut parts: Vec<SeedMappingRanged> = Vec::new();
            parts.push(seed.clone());

            while !parts.is_empty() {
                let mut current_part = parts.pop().unwrap();

                let part_start = current_part.idx[current];
                let part_end = current_part.idx[current] + current_part.range - 1;


                for mapper in mappings[current].iter() {
                    let mapper_start = mapper[1];
                    let mapper_end = mapper[1] + mapper[2];




                    // check if seed is fully in range
                    if part_start >= mapper[1] &&
                        part_end <= mapper[1] + mapper[2] {
                        current_part.idx[current + 1] = mapper[0] + current_part.idx[current] - mapper[1];
                        new_vec.push(current_part.into());
                    }

                    // check if seed is partially in range on the left side
                    else if part_start < mapper[1] && part_end > mapper[1] &&
                        part_end <= mapper[1] + mapper[2] {

                        // create new seed for part in range
                        let mut new_seed_inside = current_part.clone();
                        new_seed_inside.start = mapper[1];
                        new_seed_inside.range = part_end - mapper[1];
                        new_seed_inside.idx[current + 1] = mapper[0];
                        new_vec.push(new_seed_inside.into());

                        // create new seed for part out of range
                        let mut new_seed_outside = current_part.clone();
                        new_seed_outside.idx[current] = part_start;
                        new_seed_outside.range = mapper[1] - part_start;
                        parts.push(new_seed_outside.into());

                    }

                    // check if seed is partially in range on the right side
                    else if part_start >= mapper[1] && part_start < mapper[1] + mapper[2] &&
                        part_end > mapper[1] + mapper[2] {

                        // create new seed for part in range
                        let mut new_seed_inside = current_part.clone();
                        new_seed_inside.start = part_start;
                        new_seed_inside.range = mapper[1] + mapper[2] - part_start;
                        new_seed_inside.idx[current + 1] = mapper[0] + mapper[2] - new_seed_inside.range;;
                        new_vec.push(new_seed_inside.into());

                        // create new seed for part out of range
                        let mut new_seed_outside = current_part.clone();
                        new_seed_outside.idx[current] = mapper[1] + mapper[2] + 1;
                        new_seed_outside.range = part_end - mapper[1] - mapper[2];
                        parts.push(new_seed_outside.into());


                    }

                    // check if seed fully envelops range
                    else if part_start < mapper[1] &&
                        part_end > mapper[1] + mapper[2] {

                        // create new seed for part in range
                        let mut new_seed_inside = current_part.clone();
                        new_seed_inside.start = mapper[1];
                        new_seed_inside.range = mapper[2];
                        new_seed_inside.idx[current + 1] = mapper[0];
                        new_vec.push(new_seed_inside.into());

                        // create new seed for part out of range on the left
                        let mut new_seed_outside_left = current_part.clone();
                        new_seed_outside_left.idx[current] = part_start;
                        new_seed_outside_left.range = mapper[1] - part_start - 1;
                        parts.push(new_seed_outside_left.into());

                        // create new seed for part out of range on the right
                        let mut new_seed_outside_right = current_part.clone();
                        new_seed_outside_right.idx[current] = mapper[1] + mapper[2] + 1;
                        new_seed_outside_right.range = part_end - mapper[1] - mapper[2];
                        parts.push(new_seed_outside_right.into());

                    }

                    // check if seed is fully outside of the range
                    else if part_end < mapper[1] ||
                        part_start > mapper[1] + mapper[2] {
                        if current_part.idx[current + 1] == 0 && mapper == mappings[current].last().unwrap(){
                            current_part.idx[current + 1] = current_part.idx[current];
                            new_vec.push(current_part.into());
                        }
                    }


                }
            }


        }
        seeds = new_vec;

    }

    let lowest = seeds.iter().min_by_key(|seed| seed.idx[7]).unwrap().idx[7];

    // get second lowest value from seed.idx[7]
    let mut second_lowest = i64::MAX;
    for seed in seeds.iter() {
        if seed.idx[7] < second_lowest && seed.idx[7] > lowest {
            second_lowest = seed.idx[7];
        }
    }
    println!("second lowest: {}", second_lowest);

    lowest
}