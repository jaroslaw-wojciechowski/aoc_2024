use num::integer::gcd;

fn main() {
    // Button A: X+94, Y+34
    // Button B: X+22, Y+67
    // Prize: X=8400, Y=5400
    if is_valid(94, 22, 34, 67, 8400, 5400) {
        find_cheapest(94, 22, 34, 67, 8400, 5400);
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
                println!("coins: {}", first_multiplier * 3 + i * 1);
                break;
            }
        }
        i += 1;
    }
}
