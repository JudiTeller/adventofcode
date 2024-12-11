use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::reader::file_io::read_file;

pub fn solve_day_8() {
    let content = read_file("src/aoc_2024/day_8/input.txt");
    println!("PART 1 - Result: {}", process1(content.clone()));
    println!("PART 2 - Result: {}", process2(content));
}

fn process1(input: String) -> i64 {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut locations: HashMap<char, Vec<Location>> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, char)| char != &'.')
                .map(move |(x, char)| (char, Location { x: x as i8, y: y as i8}))
        })
        .fold(HashMap::new(), |mut acc, (char, location)| {
            acc.entry(char)
                .or_insert_with(Vec::new)
                .push(location);
            acc
        });

    let grid_y_size = lines.len() as i8;
    let grid_x_size = lines[0].len() as i8;

    let mut antinodes: HashSet<Location> = HashSet::new();

    let mut answer = 0;

    // iterate over every key of locations
    for (key, value) in locations.iter() {
        if value.len() > 1 {
            // for each pairing in value, calculate a vector from point a to point b
            // apply this vector to point b, and the reverse vector to point a
            // check if these new points are inside the grid and increase answer for each new point if inside

            for i in 0..value.len() - 1 {
                for j in i+1..value.len() {
                    let vector = (value[j].x  - value[i].x , value[j].y  - value[i].y );
                    let reverse_vector = (-vector.0, -vector.1);

                    let mut new_point = (value[j].x  + vector.0, value[j].y  + vector.1);
                    let mut reverse_point = (value[i].x  + reverse_vector.0, value[i].y  + reverse_vector.1);

                    check_location(new_point, &mut antinodes, (grid_x_size, grid_y_size));
                    check_location(reverse_point, &mut antinodes, (grid_x_size, grid_y_size));
                }
            }

        }
    }

    antinodes.len() as i64
}

fn check_location(location: (i8, i8), antinodes: &mut HashSet<Location>, grid_size: (i8, i8)) {
    if (0..grid_size.0).contains(&location.0) && (0..grid_size.1).contains(&location.1) {
        antinodes.insert(Location { x: location.0, y: location.1 });
    }
}

fn process2(input: String) -> i64 {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut locations: HashMap<char, Vec<Location>> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, char)| char != &'.')
                .map(move |(x, char)| (char, Location { x: x as i8, y: y as i8}))
        })
        .fold(HashMap::new(), |mut acc, (char, location)| {
            acc.entry(char)
                .or_insert_with(Vec::new)
                .push(location);
            acc
        });

    let grid_y_size = lines.len() as i8;
    let grid_x_size = lines[0].len() as i8;

    let mut antinodes: HashSet<Location> = HashSet::new();

    let mut answer = 0;

    // iterate over every key of locations
    for (key, value) in locations.iter() {
        if value.len() > 1 {
            // for each pairing in value, calculate a vector from point a to point b
            // apply this vector to point b, and the reverse vector to point a
            // check if these new points are inside the grid and increase answer for each new point if inside

            for i in 0..value.len() - 1 {
                for j in i+1..value.len() {
                    antinodes.insert(Location { x: value[i].x, y: value[i].y });
                    antinodes.insert(Location { x: value[j].x, y: value[j].y });

                    let vector = (value[j].x  - value[i].x , value[j].y  - value[i].y );
                    let reverse_vector = (-vector.0, -vector.1);

                    let mut new_point = (value[j].x  + vector.0, value[j].y  + vector.1);

                    while (0..grid_x_size).contains(&new_point.0) && (0..grid_y_size).contains(&new_point.1) {
                        antinodes.insert(Location { x: new_point.0, y: new_point.1 });
                        new_point = (new_point.0 + vector.0, new_point.1 + vector.1);
                    }

                    let mut reverse_point = (value[i].x  + reverse_vector.0, value[i].y  + reverse_vector.1);

                    while (0..grid_x_size).contains(&reverse_point.0) && (0..grid_y_size).contains(&reverse_point.1) {
                        antinodes.insert(Location { x: reverse_point.0, y: reverse_point.1 });
                        reverse_point = (reverse_point.0 + reverse_vector.0, reverse_point.1 + reverse_vector.1);
                    }

                }
            }
        }
    }

    antinodes.len() as i64
}

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Location {
    y: i8,
    x: i8,
}