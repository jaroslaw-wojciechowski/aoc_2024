mod utils;

use std::collections::HashMap;

use utils::read_lines;

const FILE_NAME_1: &str = "src/inputs/05-1.txt";
const FILE_NAME_2: &str = "src/inputs/05-2.txt";

fn main() {
    let order_map = build_map(FILE_NAME_1);
    parse_input(FILE_NAME_2, order_map);
}

fn build_map(file_name: &str) -> HashMap<i32, Vec<i32>> {
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

    if let Ok(lines) = read_lines(file_name) {
        for line in lines.flatten() {
            let (left, right): (i32, i32) = line
                .split_once("|")
                .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
                .expect("File should contain '|' separator");

            let values = &mut map
                .entry(left)
                .or_insert_with(|| Vec::from([right]))
                .clone();
            values.push(right);
            map.insert(left, values.clone());
        }
    }

    return map;
}

fn parse_input(file_name: &str, order_map: HashMap<i32, Vec<i32>>) {
    let mut sum: i32 = 0;
    if let Ok(lines) = read_lines(file_name) {
        for line in lines.flatten() {
            let vector: Vec<i32> = line.split(",").map(|val| val.parse().unwrap()).collect();
            if validate(&mut vector.clone(), &order_map) {
                let mid = vector[vector.len() / 2];
                sum += mid;
            }
        }
    }
    println!("sum: {}", sum);
}

fn validate(vector: &mut Vec<i32>, order_map: &HashMap<i32, Vec<i32>>) -> bool {
    vector.reverse();
    for (i, val) in vector.iter().enumerate() {
        if order_map.contains_key(val) {
            let temp_vec: Vec<i32> = order_map.get(val).unwrap().clone();
            for j in i..vector.len() {
                if temp_vec.contains(&vector[j]) {
                    return false;
                }
            }
        }
    }
    return true;
}
