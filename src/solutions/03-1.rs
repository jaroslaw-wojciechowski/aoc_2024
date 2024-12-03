mod utils;

use regex::Regex;
use utils::read_lines;

// const FILE_NAME: &str = "src/inputs/input-example.txt";
const FILE_NAME: &str = "src/inputs/03-1.txt";

fn main() {
    parse_file(FILE_NAME);
}

fn parse_file(file_name: &str) {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    if let Ok(lines) = read_lines(file_name) {
        let mut result = 0;
        for line in lines.flatten() {
            for (_, [left, right]) in re.captures_iter(&line).map(|c| c.extract()) {
                let mul: i32 = left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap();
                result += mul;
            }
        }
        println!("{result}");
    }
}
