mod utils;

use regex::Regex;
use utils::read_lines;

// const FILE_NAME: &str = "src/inputs/input-example.txt";
const FILE_NAME: &str = "src/inputs/03-1.txt";

fn main() {
    parse_file(FILE_NAME);
}

fn parse_file(file_name: &str) {
    let re = Regex::new(r"(mul\(\d+,\d+\)|don't\(\)|do\(\))").unwrap();

    if let Ok(lines) = read_lines(file_name) {
        let mut result = 0;
        let mut is_on = true;
        for line in lines.flatten() {
            for x in re.find_iter(&line).map(|i| i.as_str()) {
                match x {
                    x if x.contains("mul") => result += mul_operation(x, is_on),
                    x if x.contains("don't") => {
                        println!("OFF: {}", x);
                        is_on = false;
                    }
                    x if x.contains("do") => {
                        println!("ON: {}", x);
                        is_on = true;
                    }
                    _ => println!("ERROR"),
                }
            }
        }
        println!("{result}");
    }
}

fn mul_operation(mul: &str, is_on: bool) -> i32 {
    let mut result: i32 = 0;
    if is_on {
        println!("calc for: {}", mul);
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        for (_, [left, right]) in re.captures_iter(&mul).map(|c| c.extract()) {
            result = left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap();
            return result;
        }
    }
    return result;
}
