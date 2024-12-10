use crate::reader::file_io::read_file;
use std::cmp::max;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: usize,
    y: usize,
    steps: i64,
    start: i64,
}

pub fn solve_day_10() {
    let input = read_file("src/aoc_2023/day_10/input.txt");

    println!("Day 10, Part 1: {}", process1(input.clone()));
    println!("Day 10, Part 2: {}", process2(input));
}

fn process1(input: String) -> i64 {
    let mut start_pos: Position = Position {
        x: 0,
        y: 0,
        steps: 0,
        start: -1,
    };

    let mut lines = input.split("\n").collect::<Vec<&str>>();

    // create a 2d Vec of the same size as input, but filled with zeroes
    let mut visited: Vec<Vec<i64>> = Vec::new();
    for _ in 0..lines.len() {
        let mut new_vec: Vec<i64> = Vec::new();
        for _ in 0..lines[0].len() {
            new_vec.push(0);
        }
        visited.push(new_vec);
    }

    for y in 0..lines.len() {
        if lines[y].contains("S") {
            start_pos = Position {
                x: lines[y].find("S").unwrap(),
                y,
                steps: 0,
                start: -1,
            };
        }
    }

    let mut open_list: Vec<Position> = Vec::new();
    visited[start_pos.y][start_pos.x] = 1;

    let up_allowed = "|7F";
    let down_allowed = "|JL";
    let left_allowed = "-LF";
    let right_allowed = "-J7";

    if start_pos.y > 0
        && up_allowed.contains(lines[start_pos.y - 1].chars().nth(start_pos.x).unwrap())
    {
        open_list.push(Position {
            x: start_pos.x,
            y: start_pos.y - 1,
            steps: 1,
            start: 1,
        });
        visited[start_pos.y - 1][start_pos.x] = 1;
    }
    if start_pos.y < lines[0].len() - 1
        && down_allowed.contains(lines[start_pos.y + 1].chars().nth(start_pos.x).unwrap())
    {
        open_list.push(Position {
            x: start_pos.x,
            y: start_pos.y + 1,
            steps: 1,
            start: 2,
        });
        visited[start_pos.y + 1][start_pos.x] = 1;
    }
    if start_pos.x > 0
        && left_allowed.contains(lines[start_pos.y].chars().nth(start_pos.x - 1).unwrap())
    {
        open_list.push(Position {
            x: start_pos.x - 1,
            y: start_pos.y,
            steps: 1,
            start: 3,
        });
        visited[start_pos.y][start_pos.x - 1] = 1;
    }
    if start_pos.x < lines.len() - 1
        && right_allowed.contains(lines[start_pos.y].chars().nth(start_pos.x + 1).unwrap())
    {
        open_list.push(Position {
            x: start_pos.x + 1,
            y: start_pos.y,
            steps: 1,
            start: 4,
        });
        visited[start_pos.y][start_pos.x + 1] = 1;
    }

    let (result, (_unused, _unused2)) = flood_fill(&mut lines, &mut visited, open_list);

    result
}

fn flood_fill(
    lines: &mut Vec<&str>,
    visited: &mut Vec<Vec<i64>>,
    mut open_list: Vec<Position>,
) -> (i64, (i64, i64)) {
    let mut longest_path = 1;
    let mut starters = (0, 0);

    while !open_list.is_empty() {
        let current: Position = open_list.first().unwrap().clone();
        open_list.remove(0);
        visited[current.y][current.x] = current.start;

        // println!("x{:?} - y {:?}", current.x + 1, current.y + 1);

        let mut new_pos: Position = Position {
            x: 0,
            y: 0,
            steps: -1,
            start: -1,
        };

        // match for cases | - L J 7 F .
        match lines[current.y].chars().nth(current.x).unwrap() {
            '|' => {
                if current.y > 0 && visited[current.y - 1][current.x] == 0 {
                    new_pos = Position {
                        x: current.x,
                        y: current.y - 1,
                        steps: current.steps + 1,
                        start: current.start,
                    };
                } else if current.y < lines[0].len() - 1 && visited[current.y + 1][current.x] == 0 {
                    new_pos = Position {
                        x: current.x,
                        y: current.y + 1,
                        steps: current.steps + 1,
                        start: current.start,
                    };
                }
            }
            '-' => {
                if current.x > 0 && visited[current.y][current.x - 1] == 0 {
                    new_pos = Position {
                        x: current.x - 1,
                        y: current.y,
                        steps: current.steps + 1,
                        start: current.start,
                    };
                } else if current.x < lines[0].len() - 1 && visited[current.y][current.x + 1] == 0 {
                    new_pos = Position {
                        x: current.x + 1,
                        y: current.y,
                        steps: current.steps + 1,
                        start: current.start,
                    };
                }
            }
            // L connecting up and right
            'L' => {
                if current.x < lines.len() - 1
                    && visited[current.y][current.x + 1] == 0
                    && lines[current.y].chars().nth(current.x + 1).unwrap() != ' '
                {
                    new_pos = Position {
                        x: current.x + 1,
                        y: current.y,
                        steps: current.steps + 1,
                        start: current.start,
                    };
                } else if current.y > 0
                    && visited[current.y - 1][current.x] == 0
                    && lines[current.y - 1].chars().nth(current.x).unwrap() != ' '
                {
                    new_pos = Position {
                        x: current.x,
                        y: current.y - 1,
                        steps: current.steps + 1,
                        start: current.start,
                    };
                }
            }
            // J connecting up and left
            'J' => {
                if current.x > 0 && visited[current.y][current.x - 1] == 0 {
                    new_pos = Position {
                        x: current.x - 1,
                        y: current.y,
                        steps: current.steps + 1,
                        start: current.start,
                    };
                } else if current.y > 0 && visited[current.y - 1][current.x] == 0 {
                    new_pos = Position {
                        x: current.x,
                        y: current.y - 1,
                        steps: current.steps + 1,
                        start: current.start,
                    };
                }
            }
            // 7 connecting down and left
            '7' => {
                if current.x > 0 && visited[current.y][current.x - 1] == 0 {
                    new_pos = Position {
                        x: current.x - 1,
                        y: current.y,
                        steps: current.steps + 1,
                        start: current.start,
                    };
                } else if current.y < lines.len() - 1 && visited[current.y + 1][current.x] == 0 {
                    new_pos = Position {
                        x: current.x,
                        y: current.y + 1,
                        steps: current.steps + 1,
                        start: current.start,
                    };
                }
            }
            // F connecting down and right
            'F' => {
                if current.x < lines.len() - 1 && visited[current.y][current.x + 1] == 0 {
                    new_pos = Position {
                        x: current.x + 1,
                        y: current.y,
                        steps: current.steps + 1,
                        start: current.start,
                    };
                } else if current.y < lines.len() - 1 && visited[current.y + 1][current.x] == 0 {
                    new_pos = Position {
                        x: current.x,
                        y: current.y + 1,
                        steps: current.steps + 1,
                        start: current.start,
                    };
                }
            }

            _ => {}
        }

        // check if newPos is on a Position in open_list
        for pos in open_list.iter() {
            if pos.x == new_pos.x && pos.y == new_pos.y {
                longest_path = max(longest_path, new_pos.steps);
                starters = (new_pos.start, pos.start);
            }
        }

        if new_pos.start != -1 {
            open_list.push(new_pos);
        }
    }

    (longest_path, starters)
}

fn process2(input: String) -> i64 {
    let mut start_pos: Position = Position {
        x: 0,
        y: 0,
        steps: 0,
        start: -1,
    };

    let mut lines = input.split("\n").collect::<Vec<&str>>();

    // create a 2d Vec of the same size as input, but filled with zeroes
    let mut visited: Vec<Vec<i64>> = Vec::new();
    for _ in 0..lines.len() {
        let mut new_vec: Vec<i64> = Vec::new();
        for _ in 0..lines[0].len() {
            new_vec.push(0);
        }
        visited.push(new_vec);
    }

    for y in 0..lines.len() {
        if lines[y].contains("S") {
            start_pos = Position {
                x: lines[y].find("S").unwrap(),
                y,
                steps: 0,
                start: -1,
            };
        }
    }

    let mut open_list: Vec<Position> = Vec::new();
    visited[start_pos.y][start_pos.x] = 1;

    let up_allowed = "|7F";
    let down_allowed = "|JL";
    let left_allowed = "-LF";
    let right_allowed = "-J7";

    if start_pos.y > 0
        && up_allowed.contains(lines[start_pos.y - 1].chars().nth(start_pos.x).unwrap())
    {
        open_list.push(Position {
            x: start_pos.x,
            y: start_pos.y - 1,
            steps: 1,
            start: 1,
        });
        visited[start_pos.y - 1][start_pos.x] = 1;
    }
    if start_pos.y < lines[0].len() - 1
        && down_allowed.contains(lines[start_pos.y + 1].chars().nth(start_pos.x).unwrap())
    {
        open_list.push(Position {
            x: start_pos.x,
            y: start_pos.y + 1,
            steps: 1,
            start: 2,
        });
        visited[start_pos.y + 1][start_pos.x] = 1;
    }
    if start_pos.x > 0
        && left_allowed.contains(lines[start_pos.y].chars().nth(start_pos.x - 1).unwrap())
    {
        open_list.push(Position {
            x: start_pos.x - 1,
            y: start_pos.y,
            steps: 1,
            start: 3,
        });
        visited[start_pos.y][start_pos.x - 1] = 1;
    }
    if start_pos.x < lines.len() - 1
        && right_allowed.contains(lines[start_pos.y].chars().nth(start_pos.x + 1).unwrap())
    {
        open_list.push(Position {
            x: start_pos.x + 1,
            y: start_pos.y,
            steps: 1,
            start: 4,
        });
        visited[start_pos.y][start_pos.x + 1] = 1;
    }

    let (_, starter) = flood_fill(&mut lines, &mut visited, open_list);

    // create a 2d Vec of the same size as lines, but filled with spaces
    let mut converted: Vec<Vec<char>> = Vec::new();
    for _ in 0..lines.len() {
        let mut new_vec: Vec<char> = Vec::new();
        for _ in 0..lines[0].len() {
            new_vec.push('.');
        }
        converted.push(new_vec);
    }

    // for cells with same value as starter, copy the char from lines to visited
    for y in 0..lines.len() {
        for x in 0..lines[0].len() {
            if starter.0 == visited[y][x]
                || starter.1 == visited[y][x]
                || lines[y].chars().nth(x).unwrap() == 'S'
            {
                converted[y][x] = lines[y].chars().nth(x).unwrap();
            }
        }
    }

    let mut outside: bool = true;
    let mut counter = 0;
    let mut last = 'S';

    // create a 2d Vec of the same size as converted filled with 0s called insides
    let mut insides: Vec<Vec<i64>> = Vec::new();
    for _ in 0..converted.len() {
        let mut new_vec: Vec<i64> = Vec::new();
        for _ in 0..converted[0].len() {
            new_vec.push(0);
        }
        insides.push(new_vec);
    }

    // iterate over converted and use the raycasting algorithm to count the number of cells inside the area
    for y in 0..converted.len() {
        for x in 0..converted[0].len() {
            let current_char = converted[y][x];

            // morph current char to | - L J 7 F according to how it is connected to its neighbours
            if current_char == 'S' {
                if converted[y][x - 1] == '-' && converted[y][x + 1] == '-' {
                    continue;
                }
                outside = !outside;
            } else if current_char == '.' && !outside {
                counter += 1;
                insides[y][x] = 1;
            } else if current_char == '.' && outside {
                continue;
            }
            // flip outside if crossing a wall
            // L to 7 with any number of '-' in between count as a continuous wall
            // F to J with any number of '-' in between count as a continuous wall
            // otherwise '|' 'L' 'J' '7' 'F' count as a wall
            else if current_char == '-' {
                continue;
            } else if current_char == '|' {
                outside = !outside;
            } else if current_char == 'L' {
                outside = !outside;
                last = 'L';
            } else if current_char == 'F' {
                outside = !outside;
                last = 'F';
            } else if current_char == '7' && last == 'L' {
                last = ' ';
            } else if current_char == 'J' && last == 'F' {
                last = ' ';
            } else if current_char == '7' || converted[y][x] == 'J' {
                outside = !outside;
                last = ' ';
            }
        }
    }

    // plot converted
    for y in 0..converted.len() {
        for x in 0..converted[0].len() {
            print!("{}", converted[y][x]);
        }
        println!();
    }

    println!("");

    // plot converted, when insides == 1 print in color
    for y in 0..converted.len() {
        for x in 0..converted[0].len() {
            if insides[y][x] == 1 {
                print!("\x1b[0;31m{}\x1b[0m", converted[y][x]);
            } else {
                print!("{}", converted[y][x]);
            }
        }
        println!();
    }

    counter
}
