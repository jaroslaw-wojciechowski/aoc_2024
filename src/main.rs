mod utils;

use std::collections::HashMap;

use utils::read_lines;

// const FILE_NAME: &str = "src/inputs/input-example.txt";
const FILE_NAME: &str = "src/inputs/01-1.txt";

fn main() {
    let result = parse_file(FILE_NAME, 2020);
    println!("{result}");
}

fn parse_file(file_name: &str, target: i32) -> i32 {
    let mut num_map: HashMap<i32, i32> = HashMap::new();

    if let Ok(lines) = read_lines(file_name) {
        for line in lines.flatten() {
            let current: i32 = line.trim().parse::<i32>().unwrap();
            let diff: i32 = target - current;

            if num_map.contains_key(&current) {
                return current * num_map.get(&current).unwrap();
            } else {
                num_map.insert(diff, current);
            }
        }
    }
    return 0;
}
