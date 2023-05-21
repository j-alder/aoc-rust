mod util;
mod y2015;
mod y2022;

use std::env;
use y2015::get_solution_for_day as get_solution_for_day_2015;
use y2022::get_solution_for_day as get_solution_for_day_2022;

fn get_solution_for_year_and_day(year: &str, day: &str) -> String {
    match year {
        "2015" => get_solution_for_day_2015(day),
        "2022" => get_solution_for_day_2022(day),
        _ => String::from("No solution found"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = &args[1];
    let day = &args[2];

    let soln = get_solution_for_year_and_day(year, day);
    println!("{soln}");
}
