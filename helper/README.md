# Kattis Template

For you who struggles with local testing while maintaining a Kattis-compatable `main` function. 

This test framework utilises that byte streams (`&[u8]`, `std::io::BufReader`, `std::io::StdIn`, etc.) implements the trait `Read`.

If you are used to the structure `std::io::Lines`, don't fear; `std::str::Lines` is a compareable structure and is retured from the `lines` function of `str`.

_In short_:
```rs
use std::io::{prelude::*, self};

fn main() {
    println!("{}", solve_problem(helper::read_input(io::stdin())));
}

fn solve_problem(input: String) -> String {
    // Problem solution...
}

/// Local tests.
mod tests {
    #[test]
    fn local_test() {
        let (input_buffer, target_output_buffer) = helper::get_io(/*<test-name>*/);
        let output = solve_problem(helper::read_input(input_buffer));
        // Assert output against target output...
    }
}
```