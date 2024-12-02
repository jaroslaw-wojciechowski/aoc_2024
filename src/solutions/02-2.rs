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
    let mut result: bool = true;
    let mut counter: i32 = 0;

    if input.first().unwrap() > input.last().unwrap() {
        input.reverse();
    }

    for i in input.windows(2) {
        if i[0] >= i[1] || (i[1] - i[0]) > 3 {
            counter += 1;
            result = false;
        }
    }

    if !result && counter < 3 {
        result = check_sub(input.clone(), true) || check_sub(input.clone(), false);
    }

    return result;
}

fn check_sub(mut input: Vec<i32>, is_left: bool) -> bool {
    let mut result: bool;
    for i in 0..input.len() - 1 {
        if input[i] >= input[i + 1] || (input[i + 1] - input[i]) > 3 {
            if is_left {
                input.remove(i);
            } else {
                input.remove(i + 1);
            }
            break;
        }
    }

    result = true;
    for i in input.windows(2) {
        if i[0] >= i[1] || (i[1] - i[0]) > 3 {
            result = false;
        }
    }

    return result;
}
