use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;
use crate::reader::file_io::read_file;

pub fn solve_day_6() {
    let content = read_file("src/aoc_2024/day_6/input.txt");
    println!("PART 1 - Result: {}", process1(content.clone()));
    println!("PART 2 - Result: {}", process2(content));
}

fn process1(input: String) -> u64 {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let mut position = (0, 0);
    let mut direction = '^';
    let y_boundaries = 0..grid.len();
    let x_boundaries = 0..grid[0].len();


    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            match grid[y][x] {
                '^' => {
                    position = (y, x);
                    direction = '^';
                    grid[y][x] = 'X';
                },
                'v' => {
                    position = (y, x);
                    direction = 'v';
                    grid[y][x] = 'X';
                },
                '<' => {
                    position = (y, x);
                    direction = '<';
                    grid[y][x] = 'X';
                },
                '>' => {
                    position = (y, x);
                    direction = '>';
                    grid[y][x] = 'X';
                },
                _ => {},
            }
        }
    }

    while y_boundaries.contains(&position.0) && x_boundaries.contains(&position.1) {
        // match position on grid, move in direction
        // # = wall, . = open
        // if moving on a . mark it as visited with an X
        // if moving on a #, turn right instead, so ^ -> > -> v -> < -> ^
        // already visited spots can be visited again
        // going outside the boundaries will end the loop

        match direction {
            '^' => {
                let next_position = (position.0 - 1, position.1);
                if y_boundaries.contains(&next_position.0) && x_boundaries.contains(&next_position.1) {
                    if grid[next_position.0][next_position.1] == '.' || grid[next_position.0][next_position.1] == 'X' {
                        grid[next_position.0][next_position.1] = 'X';
                        position = next_position;
                    } else {
                        direction = '>';
                    }
                } else {
                    break;
                }
            },
            'v' => {
                let next_position = (position.0 + 1, position.1);
                if y_boundaries.contains(&next_position.0) && x_boundaries.contains(&next_position.1) {
                    if grid[next_position.0][next_position.1] == '.' || grid[next_position.0][next_position.1] == 'X' {
                        grid[next_position.0][next_position.1] = 'X';
                        position = next_position;
                    } else {
                        direction = '<';
                    }
                } else {
                    break;
                }
            },
            '<' => {
                let next_position = (position.0, position.1 - 1);
                if y_boundaries.contains(&next_position.0) && x_boundaries.contains(&next_position.1) {
                    if grid[next_position.0][next_position.1] == '.' || grid[next_position.0][next_position.1] == 'X' {
                        grid[next_position.0][next_position.1] = 'X';
                        position = next_position;
                    } else {
                        direction = '^';
                    }
                } else {
                    break;
                }
            },
            '>' => {
                let next_position = (position.0, position.1 + 1);
                if y_boundaries.contains(&next_position.0) && x_boundaries.contains(&next_position.1) {
                    if grid[next_position.0][next_position.1] == '.' || grid[next_position.0][next_position.1] == 'X' {
                        grid[next_position.0][next_position.1] = 'X';
                        position = next_position;
                    } else {
                        direction = 'v';
                    }
                } else {
                    break;
                }
            },
            _ => {},
        }
    }

    grid.iter().map(|row| row.iter().filter(|&&c| c == 'X').count()).sum::<usize>() as u64
}

fn process2(input: String) -> u64 {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();


    let mut position = (0, 0);
    let mut start_position = (0, 0);
    let mut direction = '^';
    let mut start_direction = '^';
    let y_boundaries = 0..grid.len();
    let x_boundaries = 0..grid[0].len();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            match grid[y][x] {
                '^' => {
                    start_position = (y, x);
                    start_direction = '^';
                },
                'v' => {
                    start_position = (y, x);
                    start_direction = 'v';
                },
                '<' => {
                    start_position = (y, x);
                    start_direction = '<';
                },
                '>' => {
                    start_position = (y, x);
                    start_direction = '>';
                },
                _ => {},
            }
        }
    }


    let mut answer = 0;

    let debug_x = 3;  // middle of grid
    let debug_y = 6;

    // once per simulation ONE new obstacle may be placed
    // iterate over the grid, if position is . make #
    'main: for y in 0..grid.len() {
        println!("processing line {}", y + 1);
        for x in 0..grid[y].len() {

            if grid[y][x] == '.' {
                position = start_position;
                direction = start_direction;
                let mut grid_copy = grid.clone();
                grid_copy[y][x] = '#';
                grid_copy[start_position.0][start_position.1] = '.';

                let mut visited: HashSet<State> = HashSet::new();

                let is_debug = x == debug_x && y == debug_y;
                let mut last_moves = VecDeque::with_capacity(100);


                'sim: while y_boundaries.contains(&position.0) && x_boundaries.contains(&position.1) {
                    // match position on grid, move in direction
                    // # = wall, . = open
                    // if moving on a . mark it as visited with an X
                    // if moving on a #, turn right instead, so ^ -> > -> v -> < -> ^
                    // already visited spots can be visited again
                    // visited spots can be marked with | for vertical movement, - for horizontal movement and + for intersections
                    // when moving vertical and moving over an already visited spot marked wiht |, loop detected
                    // when moving horizontal and moving over an already visited spot marked with -, loop detected
                    // going outside the boundaries will end the loop
                    // detect if the character moves in a loop by checking if the same position is visited twice with the same direction
                    if is_debug {
                        println!("debugging");
                    }

                    let current_state = State { pos: position, direction };

                    let next_position = match direction {
                        '^' => match position.0.checked_sub(1) {
                            Some(new_x) => (new_x, position.1),
                            None => break 'sim,
                        },
                        'v' => {
                            let next_pos = (position.0 + 1, position.1);
                            if !y_boundaries.contains(&next_pos.0) {
                                break 'sim;
                            }
                            next_pos
                        },
                        '<' => match position.1.checked_sub(1) {
                            Some(new_y) => (position.0, new_y),
                            None => break 'sim,
                        },
                        '>' => {
                            let next_pos = (position.0, position.1 + 1);
                            if !x_boundaries.contains(&next_pos.1) {
                                break 'sim;
                            }
                            next_pos
                        },
                        _ => break 'sim,
                    };

                    if !y_boundaries.contains(&next_position.0) || !x_boundaries.contains(&next_position.1) {
                        break 'sim;
                    }

                    if is_debug {
                        last_moves.push_back((position, direction));
                        if last_moves.len() > 100 {
                            last_moves.pop_front();
                        }
                        println!("{}: {:?}", visited.len(), last_moves);
                    }


                    if grid_copy[next_position.0][next_position.1] == '#' {
                        // Only change direction when hitting a wall
                        direction = match direction {
                            '^' => '>',
                            '>' => 'v',
                            'v' => '<',
                            '<' => '^',
                            _ => direction,
                        };
                    } else {
                        // Only add state to visited when we actually move
                        if visited.contains(&current_state) {
                            answer += 1;
                            break 'sim;
                        }
                        visited.insert(current_state);
                        position = next_position;
                    }
                }
            }
        }
    }
    answer
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct State {
    pos: (usize, usize),
    direction: char,
}