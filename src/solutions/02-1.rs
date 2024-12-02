mod utils;

use utils::read_lines;

// const FILE_NAME: &str = "src/inputs/input-example.txt";
const FILE_NAME: &str = "src/inputs/02-1.txt";

fn main() {
    parse_file(FILE_NAME);
}

fn parse_file(file_name: &str) {
    if let Ok(lines) = read_lines(file_name) {
        let mut counter = 0;
        for line in lines.flatten() {
            let input: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
            if is_safe(input) {
                counter += 1;
            }
        }
        println!("Result: {}", counter);
    }
}

fn is_safe(mut input: Vec<i32>) -> bool {
    if input.first().unwrap() > input.last().unwrap() {
        input.reverse();
    }
    for i in input.windows(2) {
        if i[0] >= i[1] || (i[1] - i[0]) > 3 {
            return false;
        }
    }
    return true;
}
