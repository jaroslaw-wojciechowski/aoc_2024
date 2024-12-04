mod utils;

use utils::read_lines;

// const SIZE: usize = 10;
const SIZE: usize = 140;
// const FILE_NAME: &str = "src/inputs/input-example.txt";
const FILE_NAME: &str = "src/inputs/04-1.txt";

fn main() {
    let matrix = write_matrix();
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
            if matrix[x][y] == 'A' {
                if (y >= 1 && x < SIZE - 1)
                    && (y < SIZE - 1 && x < SIZE - 1)
                    && (y < SIZE - 1 && x >= 1)
                    && (y >= 1 && x >= 1)
                {
                    let mut temp_left = String::new();
                    let mut temp_right = String::new();
                    temp_left.push(matrix[x - 1][y - 1]);
                    temp_left.push(matrix[x][y]);
                    temp_left.push(matrix[x + 1][y + 1]);

                    temp_right.push(matrix[x + 1][y - 1]);
                    temp_right.push(matrix[x][y]);
                    temp_right.push(matrix[x - 1][y + 1]);

                    if (temp_left.eq("MAS") || temp_left.eq("SAM"))
                        && (temp_right.eq("MAS") || temp_right.eq("SAM"))
                    {
                        counter += 1;
                    }
                }
            }
        }
    }
    println!("result: {}", counter);
}
