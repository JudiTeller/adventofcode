use crate::reader::file_io::read_file;

pub fn solve_day_4() {
    let content = read_file("src/aoc_2024/day_4/input.txt");
    println!("PART 1 - Result: {}", process1(content.clone()));
    println!("PART 2 - Result: {}", process2(content));
}

fn process1(input: String) -> i64 {
    let mut field: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        field.push(line.chars().collect());
    }

    find_xmas1(&field)
}

fn process2(input: String) -> i64 {
    let mut field: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        field.push(line.chars().collect());
    }

    find_xmas2(&field)
}

// kernel depicting XMAS in all directions
// S 0 0 S 0 0 S
// 0 A 0 A 0 A 0
// 0 0 M M M 0 0
// S A M X M A S
// 0 0 M M M 0 0
// 0 A 0 A 0 A 0
// S 0 0 S 0 0 S

fn find_xmas1(field: &Vec<Vec<char>>) -> i64 {
    let mut answer = 0;

    for i in 0..field.len() {
        for j in 0..field[i].len() {
            if field[i][j] == 'X' {
                answer += check_kernel1(&field, i, j);
            }
        }
    }

    answer
}

fn check_kernel1(field: &Vec<Vec<char>>, x: usize, y: usize) -> i64 {
    // x and y are the coordinates at which the X is located
    // check in all directions if the char == the char at the kernel
    let mut found_xmas = 0;
    // check up
    if x >= 3 {
        if field[x - 1][y] == 'M' && field[x - 2][y] == 'A' && field[x - 3][y] == 'S' {
            found_xmas += 1;
        }
    }
    // check down
    if x <= field.len() - 4 {
        if field[x + 1][y] == 'M' && field[x + 2][y] == 'A' && field[x + 3][y] == 'S' {
            found_xmas += 1;
        }
    }
    // check left
    if y >= 3 {
        if field[x][y - 1] == 'M' && field[x][y - 2] == 'A' && field[x][y - 3] == 'S' {
            found_xmas += 1;
        }
    }
    // check right
    if y <= field[x].len() - 4 {
        if field[x][y + 1] == 'M' && field[x][y + 2] == 'A' && field[x][y + 3] == 'S' {
            found_xmas += 1;
        }
    }
    // check up left
    if x >= 3 && y >= 3 {
        if field[x - 1][y - 1] == 'M' && field[x - 2][y - 2] == 'A' && field[x - 3][y - 3] == 'S' {
            found_xmas += 1;
        }
    }

    // check up right
    if x >= 3 && y <= field[x].len() - 4 {
        if field[x - 1][y + 1] == 'M' && field[x - 2][y + 2] == 'A' && field[x - 3][y + 3] == 'S' {
            found_xmas += 1;
        }
    }
    // check down left
    if x <= field.len() - 4 && y >= 3 {
        if field[x + 1][y - 1] == 'M' && field[x + 2][y - 2] == 'A' && field[x + 3][y - 3] == 'S' {
            found_xmas += 1;
        }
    }
    // check down right
    if x <= field.len() - 4 && y <= field[x].len() - 4 {
        if field[x + 1][y + 1] == 'M' && field[x + 2][y + 2] == 'A' && field[x + 3][y + 3] == 'S' {
            found_xmas += 1;
        }
    }

    found_xmas
}

fn find_xmas2(field: &Vec<Vec<char>>) -> i64 {
    let mut answer = 0;

    for i in 1..field.len() - 1 {
        for j in 1..field[i].len() - 1 {
            if field[i][j] == 'A' {
                answer += check_kernel2(&field, i, j);
            }
        }
    }

    answer
}

fn check_kernel2(field: &Vec<Vec<char>>, x: usize, y: usize) -> i64 {
    let mut found_x_mas = 0;

    //only need to check the diagonals for any combination of MAS
    // check up left down right
    if field[x - 1][y - 1] == 'M' && field[x + 1][y + 1] == 'S' || field[x - 1][y - 1] == 'S' && field[x + 1][y + 1] == 'M' {
        // can only be found in combination with the other diagonal
        // check up right down left
        if field[x - 1][y + 1] == 'M' && field[x + 1][y - 1] == 'S' || field[x - 1][y + 1] == 'S' && field[x + 1][y - 1] == 'M' {
            found_x_mas += 1;
        }
    }

    found_x_mas
}