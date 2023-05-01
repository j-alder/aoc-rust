pub mod d01;
//pub mod d02;
//pub mod d03;
//pub mod d04;
//pub mod d05;
//pub mod d06;

use d01::soln as soln1;
//use d02::soln as soln2;
//use d03::soln as soln3;
//use d04::soln as soln4;
//use d05::soln as soln5;
//use d06::soln as soln6;

pub fn get_solution_for_day(day: &str) -> String {
    match day {
        "1"=>soln1(),
//        "2"=>soln2(),
//        "3"=>soln3(),
//        "4"=>soln4(),
//        "5"=>soln5(),
//        "6"=>soln6(),
        _=>String::from("No solution found")
    }
}
