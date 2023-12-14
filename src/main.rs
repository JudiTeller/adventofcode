use std::time::Instant;

mod ex_2022;
mod aoc_2023;
pub mod reader;



fn main() {
    let starttime = Instant::now();
    aoc_2023::day_11::day11::solve_day_11();
    println!("Time elapsed: {}ms", starttime.elapsed().as_millis());

    if starttime > Instant::now() {
        use_unused_functions()
    }
}

fn use_unused_functions() {
    aoc_2023::day_1::day1::solve_day_1();
    aoc_2023::day_2::day2::solve_day_2();
    aoc_2023::day_3::day3::solve_day_3();
    aoc_2023::day_4::day4::solve_day_4();
    aoc_2023::day_5::day5::solve_day_5();

    aoc_2023::day_7::day7::solve_day_7();
    aoc_2023::day_8::day8::solve_day_8();
    aoc_2023::day_9::day9::solve_day_9();
}
