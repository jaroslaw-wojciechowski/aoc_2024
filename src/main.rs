use num::integer::gcd;
mod utils;

use utils::read_lines;

const FILE_NAME: &str = "src/inputs/input-example.txt";
// const FILE_NAME: &str = "src/inputs/01-1.txt";

const COST_A: i64 = 3;
const COST_B: i64 = 1;

struct Game {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
    target1: i64,
    target2: i64,
}

fn main() {
    let game = Game {
        x1: 17,
        x2: 84,
        y1: 86,
        y2: 37,
        target1: 7870,
        target2: 6450,
    };

    if is_valid(&game) {
        find_cheapest(&game);
    }
}

fn is_valid(game: &Game) -> bool {
    game.target1 % gcd(game.x1, game.x2) == 0 && game.target2 % gcd(game.y1, game.y2) == 0
}

fn find_cheapest(game: &Game) {
    let mut i = 0;
    while i * game.x2 < game.target1 {
        let temp = game.target1 - i * game.x2;
        if temp % game.x1 == 0 {
            let first_multiplier = temp / game.x1;

            if i * game.y2 + first_multiplier * game.y1 == game.target2 {
                println!("{} + {}", first_multiplier, i);
                println!("coins: {}", first_multiplier * COST_A + i * COST_B);
                // break;
            }
        }
        i += 1;
    }
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
