use num::integer::gcd;
mod utils;

use utils::read_lines;

// const FILE_NAME: &str = "src/inputs/input-example.txt";
const FILE_NAME: &str = "src/inputs/13-1.txt";

const COST_A: i64 = 3;
const COST_B: i64 = 1;

#[derive(Debug)]
struct Game {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
    target1: i64,
    target2: i64,
}

fn main() {
    parse_file(FILE_NAME);
}

fn is_valid(game: &Game) -> bool {
    game.target1 % gcd(game.x1, game.x2) == 0 && game.target2 % gcd(game.y1, game.y2) == 0
}

fn find_cheapest(game: &Game) -> i64 {
    let mut i = 0;
    while i * game.x2 < game.target1 {
        let temp = game.target1 - i * game.x2;
        if temp % game.x1 == 0 {
            let first_multiplier = temp / game.x1;

            if i * game.y2 + first_multiplier * game.y1 == game.target2 {
                println!("{} + {}", first_multiplier, i);
                println!("coins: {}", first_multiplier * COST_A + i * COST_B);
                return first_multiplier * COST_A + i * COST_B;
            }
        }
        i += 1;
    }
    return 0;
}

fn parse_file(file_name: &str) {
    let mut result: i64 = 0;
    if let Ok(lines) = read_lines(file_name) {
        for line in lines.flatten() {
            let (left, targets) = line
                .split_once("=")
                .expect("File should contain '=' separator");

            let (first_values, second_values) = left
                .split_once(";")
                .expect("x values and y values should contain ';' separator");

            let (x1_str, y1_str) = first_values
                .split_once(",")
                .expect("first values should contain ',' separator");

            let (x2_str, y2_str) = second_values
                .split_once(",")
                .expect("second values should contain ',' separator");

            let (t1, t2) = targets
                .split_once(",")
                .expect("target should contain ',' separator");

            let game = Game {
                x1: x1_str.parse().expect("x1 value should be a number"),
                x2: x2_str.parse().expect("x2 value should be a number"),
                y1: y1_str.parse().expect("y1 value should be a number"),
                y2: y2_str.parse().expect("y2 value should be a number"),
                target1: t1.parse().expect("target1 value should be a number"),
                target2: t2.parse().expect("target2 value should be a number"),
            };

            if is_valid(&game) {
                result += find_cheapest(&game);
            }
        }
    }
    println!("result: {}", result);
}
