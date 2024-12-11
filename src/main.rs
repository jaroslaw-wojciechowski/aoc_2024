use std::collections::HashMap;

fn main() {
    let input = "475449 2599064 213 0 2 65 5755 51149";
    // let input = String::from("125 17");
    let mut map: HashMap<String, i32> = HashMap::new();
    for i in input.split(" ").into_iter() {
        if map.contains_key(&String::from(i)) {
            let val = map.get(&String::from(i)).unwrap();
            map.insert(String::from(i), val + 1);
        } else {
            map.insert(String::from(i), 1);
        }
    }

    println!("init map: {:?}", map);

    for i in 0..75 {
        map = process_input(&map);
        let sum: i64 = map.values().map(|&x| x as i64).sum();
        // println!("iter: {} map len: {:?}; map: {:?}", i + 1, sum, map);
        println!("iter: {} map len: {:?}", i + 1, sum);
    }
}

fn process_input(in_map: &HashMap<String, i32>) -> HashMap<String, i32> {
    let map = in_map.clone();
    let mut out_map: HashMap<String, i32> = HashMap::new();

    for (key, val) in map.iter() {
        if key.eq("0") {
            let count: i32 = *out_map.get(&String::from("1")).unwrap_or(&0);
            out_map.insert(String::from("1"), val.clone() + count);
        } else if key.eq("1") {
            let count: i32 = *out_map.get(&String::from("2024")).unwrap_or(&0);
            out_map.insert(String::from("2024"), val.clone() + count);
        } else if key.eq("20") {
            let count: i32 = *out_map.get(&String::from("2")).unwrap_or(&0);
            out_map.insert(String::from("2"), val.clone() + count);
            let count: i32 = *out_map.get(&String::from("0")).unwrap_or(&0);
            out_map.insert(String::from("0"), val.clone() + count);
        } else if key.eq("24") {
            let count: i32 = *out_map.get(&String::from("2")).unwrap_or(&0);
            out_map.insert(String::from("2"), val.clone() + count);
            let count: i32 = *out_map.get(&String::from("4")).unwrap_or(&0);
            out_map.insert(String::from("4"), val.clone() + count);
        } else if key.eq("2024") {
            let count: i32 = *out_map.get(&String::from("20")).unwrap_or(&0);
            out_map.insert(String::from("20"), val.clone() + count);
            let count: i32 = *out_map.get(&String::from("24")).unwrap_or(&0);
            out_map.insert(String::from("24"), val.clone() + count);
        } else if key.len() % 2 == 0 {
            let (l, r) = split_stone(key.clone());
            let count: i32 = *out_map.get(&l).unwrap_or(&0);
            out_map.insert(l, val.clone() + count);
            let count: i32 = *out_map.get(&r).unwrap_or(&0);
            out_map.insert(r, val.clone() + count);
        } else if key.len() % 2 != 0 {
            let mul_op = mul_stone(key.clone());
            let count: i32 = *out_map.get(&mul_op).unwrap_or(&0);
            out_map.insert(mul_op, val.clone() + count);
        }
    }

    return out_map;
}

fn mul_stone(s: String) -> String {
    let big_num: i64 = s.parse().unwrap();
    let sum: i64 = big_num * 2024;
    return sum.to_string();
}

fn split_stone(s: String) -> (String, String) {
    let (left, mut right) = s.split_at(s.len() / 2);

    right = right.trim_start_matches('0');
    if right.len() == 0 {
        right = "0";
    }
    return (String::from(left), String::from(right));
}
