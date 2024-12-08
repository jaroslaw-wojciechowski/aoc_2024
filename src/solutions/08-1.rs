mod utils;

use std::collections::HashMap;

use utils::read_lines;

// const SIZE: usize = 12;
const SIZE: usize = 50;
// const FILE_NAME: &str = "src/inputs/input-example.txt";
const FILE_NAME: &str = "src/inputs/08-1.txt";

fn main() {
    let mut matrix = write_matrix();
    let coords_map = build_coords(&mut matrix);
    count_hash(coords_map, &mut matrix);
    print_matrix(matrix);
}

fn count_hash(coords_map: HashMap<char, Vec<(i32, i32)>>, matrix: &mut [[char; SIZE]; SIZE]) {
    let mut counter: i32 = 0;
    let mut overlap_vec: Vec<(i32, i32)> = Vec::new();
    for vec in coords_map.values().into_iter() {
        for i in 0..vec.len() {
            for j in i + 1..vec.len() {
                let x_diff = (vec[i].0 - vec[j].0).abs();
                let y_diff = (vec[i].1 - vec[j].1).abs();

                let antinode_1_x = match (vec[i].0, vec[j].0) {
                    (x1, x2) if x1 > x2 => x1 + x_diff,
                    (x1, x2) if x1 < x2 => x1 - x_diff,
                    (x1, x2) if x1 == x2 => x1,
                    _ => panic!("something wend wrong"),
                };

                let antinode_1_y = match (vec[i].1, vec[j].1) {
                    (y1, y2) if y1 > y2 => y1 + y_diff,
                    (y1, y2) if y1 < y2 => y1 - y_diff,
                    (y1, y2) if y1 == y2 => y1,
                    _ => panic!("something wend wrong"),
                };

                let antinode_2_x = match (vec[i].0, vec[j].0) {
                    (x1, x2) if x2 > x1 => x2 + x_diff,
                    (x1, x2) if x2 < x1 => x2 - x_diff,
                    (x1, x2) if x2 == x1 => x2,
                    _ => panic!("something wend wrong"),
                };

                let antinode_2_y = match (vec[i].1, vec[j].1) {
                    (y1, y2) if y2 > y1 => y2 + y_diff,
                    (y1, y2) if y2 < y1 => y2 - y_diff,
                    (y1, y2) if y2 == y1 => y2,
                    _ => panic!("something wend wrong"),
                };

                put_antinode(
                    antinode_1_x,
                    antinode_1_y,
                    matrix,
                    &mut counter,
                    &mut overlap_vec,
                );
                put_antinode(
                    antinode_2_x,
                    antinode_2_y,
                    matrix,
                    &mut counter,
                    &mut overlap_vec,
                );
            }
        }
    }
    println!("counter: {}", counter);
}

fn put_antinode(
    x: i32,
    y: i32,
    matrix: &mut [[char; SIZE]; SIZE],
    counter: &mut i32,
    overlap_vec: &mut Vec<(i32, i32)>,
) {
    if (x < SIZE as i32 && x >= 0) && (y < SIZE as i32 && y >= 0) {
        let current_char = matrix[x as usize][y as usize];

        match current_char {
            '.' => {
                matrix[x as usize][y as usize] = '#';
                *counter = *counter + 1;
            }
            '#' => println!("overlap #"),
            _ => {
                println!("overlap other");
                if overlap_vec.contains(&(x, y)) {
                    println!("2nd overlap other");
                } else {
                    println!("1nd overlap other");
                    *counter = *counter + 1;
                    overlap_vec.push((x, y));
                }
            }
        }
    }
}

fn build_coords(matrix: &mut [[char; SIZE]; SIZE]) -> HashMap<char, Vec<(i32, i32)>> {
    let mut coords_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for y in 0..SIZE {
        for x in 0..SIZE {
            if matrix[x][y] != '.' {
                if coords_map.contains_key(&matrix[x][y]) {
                    let coords_vec: &mut Vec<(i32, i32)> =
                        coords_map.get_mut(&matrix[x][y]).unwrap();
                    coords_vec.push((x as i32, y as i32));
                } else {
                    coords_map.insert(matrix[x][y], vec![(x as i32, y as i32)]);
                }
            }
        }
    }
    return coords_map;
}

fn write_matrix() -> [[char; SIZE]; SIZE] {
    let mut matrix: [[char; SIZE]; SIZE] = [[' '; SIZE]; SIZE];

    let file_name = FILE_NAME;
    if let Ok(lines) = read_lines(file_name) {
        for (y, line) in lines.flatten().enumerate() {
            for (x, char) in line.chars().enumerate() {
                matrix[x][y] = char;
            }
        }
    }
    return matrix;
}

fn print_matrix(matrix: [[char; SIZE]; SIZE]) {
    let mut counter = 0;
    for y in 0..SIZE {
        for x in 0..SIZE {
            if matrix[x][y] == '#' {
                counter += 1;
            }
            print!("{}", matrix[x][y]);
        }
        println!();
    }
}
