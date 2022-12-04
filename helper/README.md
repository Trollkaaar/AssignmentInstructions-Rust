# Kattis Template

For you who struggles with local testing while maintaining a Kattis-compatable `main` function. 

This test framework utilises that byte streams (`&[u8]`, `std::io::BufReader`, `std::io::StdIn`, etc.) implements the trait `Read`.

If you are used to the structure `std::io::Lines`, don't fear; `std::str::Lines` is a compareable structure and is retured from the `lines` function of `str`.

_Example_:
```rs
use std::io::{prelude::*, self};

fn main() {
    println!("{}", solve_problem(read_input(io::stdin())));
}

fn solve_problem(input: String) -> String {
    // Problem solution...
}

fn read_input<R: Read>(mut reader: R) -> String {/*...*/}

mod tests {
    use std::{fs::File, io::{BufReader}};
    use super::{read_input, solve_problem};

    #[test]
    fn local_test() {
        let (input_buffer, target_output_buffer) = get_io("edge-case-1");
        let output = solve_problem(read_input(input_buffer));
        // Assert output against target output...
    }

    fn get_io(test_name: &str) -> (BufReader<File>, BufReader<File>) {/*...*/}
}
```