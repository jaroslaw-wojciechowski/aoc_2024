use num::integer::gcd;
mod utils;

use utils::read_lines;

// const FILE_NAME: &str = "src/inputs/input-example.txt";
const FILE_NAME: &str = "src/inputs/13-1.txt";

const OFFSET: i64 = 10000000000000;
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
    let x1 = game.x1;
    let x2 = game.x2;
    let target1 = game.target1;

    let y1 = game.y1;
    let y2 = game.y2;
    let target2 = game.target2;

    if ((x2 * target2) - (y2 * target1)) % ((x2 * y1) - (y2 * x1)) != 0 {
        return 0;
    }

    let a2 = ((x2 * target2) - (y2 * target1)) / ((x2 * y1) - (y2 * x1));
    let b2 = (target1 - (x1 * a2)) / x2;

    let cost2 = a2 * COST_A + b2 * COST_B;

    println!("A2: {}, B2: {}, cost: {}", a2, b2, cost2);
    return cost2;
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

            let target1: i64 = t1.parse().expect("target1 value should be a number");
            let target2: i64 = t2.parse().expect("target2 value should be a number");

            let game = Game {
                x1: x1_str.parse().expect("x1 value should be a number"),
                x2: x2_str.parse().expect("x2 value should be a number"),
                y1: y1_str.parse().expect("y1 value should be a number"),
                y2: y2_str.parse().expect("y2 value should be a number"),
                target1: OFFSET + target1,
                target2: OFFSET + target2,
            };

            if is_valid(&game) {
                result += find_cheapest(&game);
            }
        }
    }
    println!("result: {}", result);
}
