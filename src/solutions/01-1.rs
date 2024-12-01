mod utils;

use utils::read_lines;

// const FILE_NAME: &str = "src/inputs/input-example.txt";
const FILE_NAME: &str = "src/inputs/01-1.txt";

fn main() {
    let (left, right): (Vec<i32>, Vec<i32>) = parse_file(FILE_NAME);
    let result = calculate_result((left, right));

    println!("Result: {}", result);
}

fn parse_file(file_name: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_vec: Vec<i32> = Vec::new();
    let mut right_vec: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(file_name) {
        for line in lines.flatten() {
            let (left, right) = line
                .split_once("   ")
                .expect("File should contain '   ' separator");

            let left_num = left.parse().expect("Left value should be a number");
            let right_num = right.parse().expect("Right value should be a number");
            left_vec.push(left_num);
            right_vec.push(right_num);
        }
    }

    return (left_vec, right_vec);
}

fn calculate_result((mut left, mut right): (Vec<i32>, Vec<i32>)) -> u32 {
    let mut sum: u32 = 0;
    left.sort();
    right.sort();

    for (index, left_num) in left.into_iter().enumerate() {
        sum += right[index].abs_diff(left_num);
    }
    sum
}
