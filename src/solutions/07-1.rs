mod utils;

use std::i64;

use utils::read_lines;

// const FILE_NAME: &str = "src/inputs/input-example.txt";
const FILE_NAME: &str = "src/inputs/07-1.txt";

fn main() {
    parse_input(FILE_NAME);
}

fn parse_input(file_name: &str) {
    let mut answer = 0;
    if let Ok(lines) = read_lines(file_name) {
        for line in lines.flatten() {
            let (left, right) = line
                .split_once(":")
                .map(|(l, r)| (l.trim(), r.trim()))
                .expect("File should contain ':' separator");

            let result: i64 = left.parse().unwrap();
            let values: Vec<i32> = right
                .split(" ")
                .map(|val| val.trim().parse().unwrap())
                .collect();

            let combinations = variation(result, &values);
            if validate(result, values, combinations) {
                println!("true for: {}", result);
                answer += result;
            }
        }
    }
    println!("answer: {}", answer);
}

fn validate(result: i64, values: Vec<i32>, combinations: Vec<Vec<usize>>) -> bool {
    for combination in combinations {
        let mut sum: i64 = values[0] as i64;
        for i in 0..combination.len() {
            if combination[i] == 0 {
                sum = sum + values[i + 1] as i64;
            } else {
                sum = sum * values[i + 1] as i64;
            }
        }
        if result == sum {
            return true;
        }
    }
    return false;
}

// AI generated
fn variation(result: i64, values: &Vec<i32>) -> Vec<Vec<usize>> {
    let n = vec![0, 1];
    let k = values.len() - 1;
    let mut current = vec![0; k];
    let mut combinations = Vec::new();

    loop {
        combinations.push(current.clone());

        let mut i = k - 1;
        while i >= 0 {
            current[i] += 1;
            if current[i] < n.len() {
                break;
            }
            current[i] = 0;
            if i == 0 {
                return combinations;
            }
            i -= 1;
        }
    }
}
