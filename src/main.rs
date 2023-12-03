mod ex_2022;
mod aoc_2023;
pub mod reader;



fn main() {
    let starttime = std::time::Instant::now();
    aoc_2023::day_2::day2::solve_day_2();
    println!("Time elapsed: {}ms", starttime.elapsed().as_millis());
}
