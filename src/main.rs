mod ex_2022;
mod aoc_2023;


fn main() {
    let starttime = std::time::Instant::now();
    aoc_2023::day_1::day1::solve_day_1();
    println!("Time elapsed: {}ms", starttime.elapsed().as_millis());
}
