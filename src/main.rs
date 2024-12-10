mod utils;

use std::collections::HashSet;

use utils::read_lines;

// const SIZE: usize = 8;
const SIZE: usize = 54;
// const FILE_NAME: &str = "src/inputs/input-example.txt";
const FILE_NAME: &str = "src/inputs/10-1.txt";

fn main() {
    let mut matrix = write_matrix();
    // search(&mut matrix, (2, 5), 1); => 1
    // let mut coord_set: HashSet<(usize, usize)> = HashSet::new();
    // search(&mut matrix, &mut coord_set, (5, 5), 1); //=> 3
    // println!("coord_set: {:?}", coord_set);
    traverse_matrix(matrix);
    // print_matrix(&matrix);
}

fn print_matrix(matrix: &[[char; SIZE]; SIZE]) {
    for y in 0..SIZE {
        for x in 0..SIZE {
            print!("{}", matrix[x][y]);
        }
        println!();
    }
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

fn traverse_matrix(mut matrix: [[char; SIZE]; SIZE]) {
    let mut result = 0;
    for y in 0..SIZE {
        for x in 0..SIZE {
            if matrix[x][y].to_digit(10).unwrap() == 0 {
                let mut coord_set: HashSet<(usize, usize)> = HashSet::new();
                search(&mut matrix, &mut coord_set, (x, y), 1);
                // println!("hashset: {:?}", coord_set);
                result += coord_set.len();
            }
        }
    }
    println!("result: {}", result);
}

fn search(
    matrix: &mut [[char; SIZE]; SIZE],
    coord_set: &mut HashSet<(usize, usize)>,
    coords: (usize, usize),
    hight: u32,
) {
    let (x, y) = coords;
    // check top
    if y > 0 && hight == matrix[x][y - 1].to_digit(10).unwrap() {
        if hight == 9 {
            coord_set.insert((x, y - 1));
        } else {
            search(matrix, coord_set, (x, y - 1), hight + 1);
        }
    }
    // check right
    if x < SIZE - 1 && hight == matrix[x + 1][y].to_digit(10).unwrap() {
        if hight == 9 {
            coord_set.insert((x + 1, y));
        } else {
            search(matrix, coord_set, (x + 1, y), hight + 1);
        }
    }
    // check bot
    if y < SIZE - 1 && hight == matrix[x][y + 1].to_digit(10).unwrap() {
        if hight == 9 {
            coord_set.insert((x, y + 1));
        } else {
            search(matrix, coord_set, (x, y + 1), hight + 1);
        }
    }
    // check left
    if x > 0 && hight == matrix[x - 1][y].to_digit(10).unwrap() {
        if hight == 9 {
            coord_set.insert((x - 1, y));
        } else {
            search(matrix, coord_set, (x - 1, y), hight + 1);
        }
    }
}
