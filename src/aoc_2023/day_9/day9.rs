use crate::reader::file_io::read_file;

pub fn solve_day_9() {
    let input = read_file("src/aoc_2023/day_9/input.txt");

    println!("Day 9, Part 1: {}", process1(input.clone()));
    println!("Day 9, Part 2: {}", process2(input));
}



fn process1(input: String) -> i64 {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let number_re = regex::Regex::new(r"(-?\d+)").unwrap();

    let mut result = 0;

    for line in lines {
        // collect numbers with regex and parse to i64
        let mut numbers = number_re.captures_iter(line).map(|cap| cap[1].parse::<i64>().unwrap()).collect::<Vec<i64>>();

        let mut series: Vec<Vec<i64>> = Vec::new();
        series.push(numbers);

        // while the last entry of series does not only contain 0s
        while series.last().unwrap().iter().filter(|&x| *x != 0).count() > 0 {
            let last_entry = series.last().unwrap();

            let mut new_series = Vec::new();
            for i in 0..last_entry.len() - 1 {
                new_series.push(last_entry[i+1] - last_entry[i]);
            }
            series.push(new_series);
        }


        // extrapolate up
        let mut counter = 0usize;
        let mut new_vec: Vec<i64> = Vec::new();
        let index_length = series.len() - 1;

        loop {
            let calc_counter = index_length - counter;

            if series.get(calc_counter).unwrap() == series.last().unwrap() {
                new_vec.push(0);
            }

            else {

                new_vec.push(series.get(calc_counter).unwrap().last().unwrap() + new_vec.get(counter - 1).unwrap());
            }

            if series.get(calc_counter).unwrap() == series.first().unwrap() {
                break;
            }

            counter += 1;
        }

        result += new_vec.last().unwrap();
    }


    result
}

fn process2(input: String) -> i64 {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let number_re = regex::Regex::new(r"(-?\d+)").unwrap();

    let mut result = 0;

    for line in lines {
        // collect numbers with regex and parse to i64
        let mut numbers = number_re.captures_iter(line).map(|cap| cap[1].parse::<i64>().unwrap()).collect::<Vec<i64>>();

        let mut series: Vec<Vec<i64>> = Vec::new();
        series.push(numbers);

        // while the last entry of series does not only contain 0s
        while series.last().unwrap().iter().filter(|&x| *x != 0).count() > 0 {
            let last_entry = series.last().unwrap();

            let mut new_series = Vec::new();
            for i in 0..last_entry.len() - 1 {
                new_series.push(last_entry[i+1] - last_entry[i]);
            }
            series.push(new_series);
        }


        // extrapolate up
        let mut counter = 0usize;
        let mut new_vec: Vec<i64> = Vec::new();
        let index_length = series.len() - 1;

        loop {
            let calc_counter = index_length - counter;

            if series.get(calc_counter).unwrap() == series.last().unwrap() {
                new_vec.push(0);
            }

            else {

                new_vec.push(series.get(calc_counter).unwrap().first().unwrap() - new_vec.get(counter - 1).unwrap());
            }

            if series.get(calc_counter).unwrap() == series.first().unwrap() {
                break;
            }

            counter += 1;
        }

        result += new_vec.last().unwrap();
    }


    result
}