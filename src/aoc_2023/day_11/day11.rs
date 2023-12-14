use crate::reader::file_io::read_file;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Star {
    x: i64,
    y: i64,
}

pub fn solve_day_11() {
    let input = read_file("src/aoc_2023/day_11/input.txt");

    println!("Day 10, Part 1: {}", process1(input.clone()));
    println!("Day 10, Part 2: {}", process2(input));
}


fn process1(input: String) -> i64 {
    let lines = input.split("\n").collect::<Vec<&str>>();

    // check if any column in lines is only '.'
    let mut column_dots: Vec<i64> = Vec::new();

    for (i, c) in lines[0].chars().enumerate() {

        // if c is '.', add 1 to column_dots[i]
        if c == '.' {
            column_dots.push(i as i64);
        }
    }

    for line in lines.clone() {
        // iterate over chars and indices in line
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                if column_dots.contains(&(i as i64)) {
                    column_dots.remove(column_dots.iter().position(|&x| x == i as i64).unwrap());
                }
            }
        }
    }

    // create a copy of lines
    let mut expanded: Vec<String> = Vec::new();

    for line in lines.clone() {
        expanded.push(line.to_string());
    }

    for line in 0..lines.len() {
        // insert '.' into expanded at indices in column_dots
        for i in column_dots.clone().iter().rev(){
            expanded[line].insert((i + 1) as usize, '.');
        }

    }

    let mut stars: Vec<Star> = Vec::new();
    let mut last_expanded = false;

    let mut counter = lines.len() - 1;

    while counter > 0 {
        // if !last_expanded {
            // if expanded[line] only contains '.', insert a copy of expanded[line] at line + 1
            if lines[counter].chars().filter(|&x| x == '#').count() == 0 {
                expanded.insert(counter + 1, expanded[counter].clone());
                last_expanded = true;
            }
        // }
        // else {
        //     last_expanded = false;
        // }
        counter -= 1;
    }

    for (y, line) in expanded.clone().iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                stars.push(Star {x: x as i64, y: y as i64});
            }
        }
    }

    let mut result: i64 = 0;

    while !stars.is_empty() {
        let mut current = stars[0];
        stars.remove(0);

        // calculate length of vector to every other star
        for star in stars.clone() {
            let vector: (i64, i64) = (star.x - current.x, star.y - current.y);

            let distance = vector.0.abs() + vector.1.abs();

            result += distance as i64;
        }
    }

    result
}

fn process2(input: String) -> i64 {
    let lines = input.split("\n").collect::<Vec<&str>>();

    // check if any column in lines is only '.'
    let mut column_dots: Vec<i64> = Vec::new();

    for (i, c) in lines[0].chars().enumerate() {

        // if c is '.', add 1 to column_dots[i]
        if c == '.' {
            column_dots.push(i as i64);
        }
    }

    for line in lines.clone() {
        // iterate over chars and indices in line
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                if column_dots.contains(&(i as i64)) {
                    column_dots.remove(column_dots.iter().position(|&x| x == i as i64).unwrap());
                }
            }
        }
    }

    let mut stars: Vec<Star> = Vec::new();

    for (y, line) in lines.clone().iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                stars.push(Star {x: x as i64, y: y as i64});
            }
        }
    }

    let multiplier = 1000000;

    // iterate over lines and indices in reverse

    for line in (0..lines.clone().len()).rev() {
        // if line only contains '.', add multiplier to every y value of stars with y > line
        if lines[line].chars().filter(|&x| x == '#').count() == 0 {
            for star in 0..stars.clone().len() {
                if stars[star].y > line as i64{
                    stars[star].y += multiplier - 1;
                }
            }
        }

    }

    // iterate over column_dots in reverse and add multiplier to every x value of stars with x > column_dots[i]
    for i in column_dots.clone().iter().rev() {
        for star in 0..stars.clone().len() {
            if stars[star].x > *i {
                stars[star].x += multiplier - 1;
            }
        }
    }


    let mut result: i64 = 0;

    while !stars.is_empty() {
        let mut current = stars[0];
        stars.remove(0);

        // calculate length of vector to every other star
        for star in stars.clone() {
            let vector: (i64, i64) = (star.x - current.x, star.y - current.y);

            let distance = vector.0.abs() + vector.1.abs();

            result += distance as i64;
        }
    }

    result
}