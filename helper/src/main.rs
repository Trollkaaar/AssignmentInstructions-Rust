/**
 * Solution to '<task-title>'
 * @see https://kth.kattis.com/problems/<problem-title>
 * @see https://open.kattis.com/problems/<problem-title>
 * 
 * Task #x -- DD13XX <inda-course-name>
 * 
 * @author <name> <<kth-id>@kth.se>
 * @created <date>
 * @last-edited <date>
 */

use std::io::{prelude::*, self};

fn main() {
    // Kattis submission:
    println!("{}", solve_problem(read_input(io::stdin())));
}

fn solve_problem(input: String) -> String {
    // Problem solution...
}

/// Reads string from given byte stream.
/// 
/// Takes a stream which implements trait `Read` and returns a string with all content from mensioned stream in UTF-8
/// 
/// # Examples
/// _From byte slice (probably unnessecary)_:
/// ```
/// let input = b"5\n1 815 4 43 54";
/// let input_buffer = read_input(&input[..]);
/// ```
/// _From standard input stream:_
/// ```
/// let input_buffer = read_input(std::io::stdin());
/// ```
/// _From file:_
/// ```
/// let input_file = std::fs::File::open("../tests/test.indata");
/// let input_buffer = std::io::BufReader::new(input_file.unwrap());
/// let input_buffer = read_input(input_buffer);
/// ```
fn read_input<R: Read>(mut reader: R) -> String {
    let mut input = String::new();
    reader.read_to_string(&mut input);
    input
}

/// Local tests.
mod tests {
    use std::{fs::File, io::{BufReader}};
    use super::{read_input, solve_problem};

    #[test]
    fn it_works() {
        assert!(true);
    }

    /// Get buffered byte stream readers of given test name at `../tests/<given-test-name>test.indata`.
    /// 
    /// # Example
    /// ```
    /// let (input_buffer, target_output_buffer) = get_io(/*<test-name>*/);
    /// let output = solve_problem(read_input(input_buffer));
    /// // Assert output against target...
    /// ```
    fn get_io(test_name: &str) -> (BufReader<File>, BufReader<File>) {
        let input_file = File::open(format!("../tests/{}test.indata", test_name));
        let input_buffer = BufReader::new(input_file.ok().unwrap());

        let output_file = File::open(format!("../tests/{}test.utdata", test_name));
        let target_output_buffer = BufReader::new(output_file.ok().unwrap());

        (input_buffer, target_output_buffer)
    }
}