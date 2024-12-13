use num::integer::gcd;

const COST_A: i64 = 3;
const COST_B: i64 = 1;

fn main() {
    let x1 = 17;
    let x2 = 84;
    let y1 = 86;
    let y2 = 37;
    let target1 = 7870;
    let target2 = 6450;

    if is_valid(x1, x2, y1, y2, target1, target2) {
        find_cheapest(x1, x2, y1, y2, target1, target2);
    }
}

fn is_valid(x1: i64, x2: i64, y1: i64, y2: i64, target1: i64, target2: i64) -> bool {
    target1 % gcd(x1, x2) == 0 && target2 % gcd(y1, y2) == 0
}

fn find_cheapest(x1: i64, x2: i64, y1: i64, y2: i64, target1: i64, target2: i64) {
    let mut i = 0;
    while i * x2 < target1 {
        let temp = target1 - i * x2;
        if temp % x1 == 0 {
            let first_multiplier = temp / x1;

            if i * y2 + first_multiplier * y1 == target2 {
                println!("{} + {}", first_multiplier, i);
                println!("coins: {}", first_multiplier * COST_A + i * COST_B);
                // break;
            }
        }
        i += 1;
    }
}
