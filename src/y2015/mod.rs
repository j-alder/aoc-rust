pub mod d01;

use d01::soln as soln1;

pub fn get_solution_for_day(day: &str) -> String {
    match day {
        "1"=>soln1(),
        _=>String::from("No solution found")
    }
}