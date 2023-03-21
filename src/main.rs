mod y2015;

use std::env;
use y2015::get_solution_for_day;

fn get_solution_for_year_and_day(year: &str, day: &str) -> String {
    match year {
        "2015"=>get_solution_for_day(day),
        _=>String::from("No solution found")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = &args[1];
    let day = &args[2];

    let soln = get_solution_for_year_and_day(year, day);
    println!("{soln}");
}
