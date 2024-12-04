mod utils;

use utils::read_lines;

// const SIZE: usize = 10;
const SIZE: usize = 140;
// const FILE_NAME: &str = "src/inputs/input-example.txt";
const FILE_NAME: &str = "src/inputs/04-1.txt";

fn main() {
    let matrix = write_matrix();
    // print_matrix(&matrix);
    traverse_matrix(&matrix);
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

fn traverse_matrix(matrix: &[[char; SIZE]; SIZE]) {
    let mut counter = 0;

    for y in 0..SIZE {
        for x in 0..SIZE {
            if matrix[x][y] == 'X' {
                // test top
                if y >= 3 {
                    let mut temp = String::new();
                    temp.push(matrix[x][y]);
                    temp.push(matrix[x][y - 1]);
                    temp.push(matrix[x][y - 2]);
                    temp.push(matrix[x][y - 3]);
                    if temp.eq("XMAS") {
                        counter += 1;
                    }
                }
                // test top-right
                if y >= 3 && x < SIZE - 3 {
                    let mut temp = String::new();
                    temp.push(matrix[x][y]);
                    temp.push(matrix[x + 1][y - 1]);
                    temp.push(matrix[x + 2][y - 2]);
                    temp.push(matrix[x + 3][y - 3]);
                    if temp.eq("XMAS") {
                        counter += 1;
                    }
                }
                // test right
                if x < SIZE - 3 {
                    let mut temp = String::new();
                    temp.push(matrix[x][y]);
                    temp.push(matrix[x + 1][y]);
                    temp.push(matrix[x + 2][y]);
                    temp.push(matrix[x + 3][y]);
                    if temp.eq("XMAS") {
                        counter += 1;
                    }
                }
                // test down-right
                if y < SIZE - 3 && x < SIZE - 3 {
                    let mut temp = String::new();
                    temp.push(matrix[x][y]);
                    temp.push(matrix[x + 1][y + 1]);
                    temp.push(matrix[x + 2][y + 2]);
                    temp.push(matrix[x + 3][y + 3]);
                    if temp.eq("XMAS") {
                        counter += 1;
                    }
                }
                // test down
                if y < SIZE - 3 {
                    let mut temp = String::new();
                    temp.push(matrix[x][y]);
                    temp.push(matrix[x][y + 1]);
                    temp.push(matrix[x][y + 2]);
                    temp.push(matrix[x][y + 3]);
                    if temp.eq("XMAS") {
                        counter += 1;
                    }
                }
                // test down-left
                if y < SIZE - 3 && x >= 3 {
                    let mut temp = String::new();
                    temp.push(matrix[x][y]);
                    temp.push(matrix[x - 1][y + 1]);
                    temp.push(matrix[x - 2][y + 2]);
                    temp.push(matrix[x - 3][y + 3]);
                    if temp.eq("XMAS") {
                        counter += 1;
                    }
                }
                // test left
                if x >= 3 {
                    let mut temp = String::new();
                    temp.push(matrix[x][y]);
                    temp.push(matrix[x - 1][y]);
                    temp.push(matrix[x - 2][y]);
                    temp.push(matrix[x - 3][y]);
                    if temp.eq("XMAS") {
                        counter += 1;
                    }
                }
                // test top-left
                if y >= 3 && x >= 3 {
                    let mut temp = String::new();
                    temp.push(matrix[x][y]);
                    temp.push(matrix[x - 1][y - 1]);
                    temp.push(matrix[x - 2][y - 2]);
                    temp.push(matrix[x - 3][y - 3]);
                    if temp.eq("XMAS") {
                        counter += 1;
                    }
                }
            }
        }
    }
    println!("result: {}", counter);
}

fn print_matrix(matrix: &[[char; SIZE]; SIZE]) {
    for y in 0..SIZE {
        for x in 0..SIZE {
            print!("{}", matrix[x][y]);
        }
        println!();
    }
}
