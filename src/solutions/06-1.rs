mod utils;

use utils::read_lines;

// const START_POS: (usize, usize) = (4, 6);
const START_POS: (usize, usize) = (68, 95);
// const SIZE: usize = 10;
const SIZE: usize = 130;
// const FILE_NAME: &str = "src/inputs/input-example.txt";
const FILE_NAME: &str = "src/inputs/06-1.txt";

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn main() {
    let mut matrix = write_matrix();
    traverse_matrix(&mut matrix);
    print_matrix(&matrix);
    let result = count_visited(&matrix);
    println!("result: {}", result);
}

fn count_visited(matrix: &[[char; SIZE]; SIZE]) -> i32 {
    let mut counter = 0;

    for y in 0..SIZE {
        for x in 0..SIZE {
            if matrix[x][y] == '@' {
                counter += 1;
            }
        }
    }

    return counter;
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

fn traverse_matrix(matrix: &mut [[char; SIZE]; SIZE]) {
    let (mut x, mut y) = START_POS;
    let (mut new_x, mut new_y);
    let mut dir = Direction::UP;

    loop {
        matrix[x][y] = '@';
        if move_dir(x, y, &dir).is_ok() {
            (new_x, new_y) = move_dir(x, y, &dir).unwrap();
        } else {
            break;
        }

        if matrix[new_x][new_y] != '#' {
            (x, y) = (new_x, new_y);
        } else {
            dir = change_dir(dir);
        }
    }
}

fn move_dir(x: usize, y: usize, current_dir: &Direction) -> Result<(usize, usize), &'static str> {
    return match current_dir {
        Direction::UP => {
            if y == 0 {
                Err("invalid size")
            } else {
                Ok((x, y - 1))
            }
        }
        Direction::DOWN => {
            if y == SIZE - 1 {
                Err("invalid size")
            } else {
                Ok((x, y + 1))
            }
        }
        Direction::RIGHT => {
            if x == SIZE - 1 {
                Err("invalid size")
            } else {
                Ok((x + 1, y))
            }
        }
        Direction::LEFT => {
            if x == 0 {
                Err("invalid size")
            } else {
                Ok((x - 1, y))
            }
        }
    };
}

fn change_dir(current_dir: Direction) -> Direction {
    return match current_dir {
        Direction::UP => Direction::RIGHT,
        Direction::DOWN => Direction::LEFT,
        Direction::RIGHT => Direction::DOWN,
        Direction::LEFT => Direction::UP,
    };
}
