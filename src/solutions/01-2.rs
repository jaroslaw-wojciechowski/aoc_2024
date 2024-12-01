mod utils;

use std::collections::HashMap;

use utils::read_lines;

// const FILE_NAME: &str = "src/inputs/input-example.txt";
const FILE_NAME: &str = "src/inputs/01-1.txt";

fn main() {
    let (left, right_map): (Vec<i32>, HashMap<i32, i32>) = parse_file(FILE_NAME);
    let result = calculate_result((left, right_map));

    println!("Result: {}", result);
}

fn parse_file(file_name: &str) -> (Vec<i32>, HashMap<i32, i32>) {
    let mut left_vec: Vec<i32> = Vec::new();
    let mut right_map: HashMap<i32, i32> = HashMap::new();

    if let Ok(lines) = read_lines(file_name) {
        for line in lines.flatten() {
            let (left, right) = line
                .split_once("   ")
                .expect("File should contain '   ' separator");

            let left_num = left.parse().expect("Left value should be a number");
            let right_num = right.parse().expect("Right value should be a number");

            left_vec.push(left_num);
            right_map
                .entry(right_num)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }

    return (left_vec, right_map);
}

fn calculate_result((left, right_map): (Vec<i32>, HashMap<i32, i32>)) -> i32 {
    left.iter()
        .map(|num| num * right_map.get(num).unwrap_or(&0))
        .sum()
}
