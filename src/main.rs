mod utils;

use utils::read_lines;

// const SIZE: usize = 10;
const SIZE: usize = 140;
// const FILE_NAME: &str = "src/inputs/input-example.txt";
const FILE_NAME: &str = "src/inputs/12-1.txt";

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
    let mut visited: Vec<(usize, usize)> = Vec::new();
    let mut diff = 0;
    let mut result = 0;

    for y in 0..SIZE {
        for x in 0..SIZE {
            if !visited.contains(&(x, y)) {
                let wynik = bfs(&matrix, &mut visited, x, y);
                result += (visited.len() - diff) * wynik as usize;
                println!("visited: {} * wynik: {}", visited.len() - diff, wynik);
                diff = visited.len();
            }
        }
    }
    println!("result {}", result);
}

fn bfs(
    matrix: &[[char; SIZE]; SIZE],
    visited: &mut Vec<(usize, usize)>,
    x: usize,
    y: usize,
) -> i32 {
    let mut fields: Vec<(usize, usize)> = Vec::new();
    let mut count: i32 = 0;

    if visited.contains(&(x, y)) {
        return 0;
    }
    visited.push((x, y));

    if y > 0 {
        fields.push((x, y - 1));
    } else {
        count += 1;
    }

    if y < SIZE - 1 {
        fields.push((x, y + 1));
    } else {
        count += 1;
    }

    if x > 0 {
        fields.push((x - 1, y));
    } else {
        count += 1;
    }

    if x < SIZE - 1 {
        fields.push((x + 1, y));
    } else {
        count += 1;
    }

    for coords in fields {
        if matrix[coords.0][coords.1] != matrix[x][y] {
            count += 1;
        } else {
            count += bfs(matrix, visited, coords.0, coords.1);
        }
    }

    return count;
}
