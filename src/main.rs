mod ex_2022;
mod aoc_2023;
pub mod reader;



fn main() {
    let starttime = std::time::Instant::now();
    aoc_2023::day_9::day9::solve_day_9();
    println!("Time elapsed: {}ms", starttime.elapsed().as_millis());
}
