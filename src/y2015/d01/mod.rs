use std::fs;

pub fn soln() -> String {
    let input = fs::read_to_string("src/y2015/d01/input.txt")
        .unwrap();

    let mut f = 0;
    let mut b: usize = 0;

    for (i, c) in input.chars().enumerate() {
        f += match c {
            '(' => 1,
            ')' => -1,
            _ => 0
        };
        if b == 0 && f == -1 {
            b = i + 1;
        }
    }

    return format!("\nPart 1: {}\nPart 2: {}\n", f.to_string(), b.to_string())
}
