pub mod d01;
pub mod d02;

use d01::soln as soln1;
use d02::soln as soln2;

pub fn get_solution_for_day(day: &str) -> String {
    match day {
        "1"=>soln1(),
        "2"=>soln2(),
        _=>String::from("No solution found")
    }
}