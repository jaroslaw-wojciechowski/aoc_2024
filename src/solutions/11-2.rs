use std::collections::HashMap;

fn main() {
    let input = "475449 2599064 213 0 2 65 5755 51149";
    // let input = String::from("125 17");
    let mut map: HashMap<String, i64> = HashMap::new();
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

fn process_input(in_map: &HashMap<String, i64>) -> HashMap<String, i64> {
    let mut out_map: HashMap<String, i64> = HashMap::with_capacity(in_map.len() * 2);

    for (key, &val) in in_map {
        match key.as_str() {
            "0" => {
                *out_map.entry(String::from("1")).or_insert(0) += val;
            }
            "1" => {
                *out_map.entry(String::from("2024")).or_insert(0) += val;
            }
            "20" => {
                *out_map.entry(String::from("2")).or_insert(0) += val;
                *out_map.entry(String::from("0")).or_insert(0) += val;
            }
            "24" => {
                *out_map.entry(String::from("2")).or_insert(0) += val;
                *out_map.entry(String::from("4")).or_insert(0) += val;
            }
            "2024" => {
                *out_map.entry(String::from("20")).or_insert(0) += val;
                *out_map.entry(String::from("24")).or_insert(0) += val;
            }
            key if key.len() % 2 == 0 => {
                let (l, r) = split_stone(key.to_string());
                *out_map.entry(l).or_insert(0) += val;
                *out_map.entry(r).or_insert(0) += val;
            }
            key => {
                let mul_op = mul_stone(key.to_string());
                *out_map.entry(mul_op).or_insert(0) += val;
            }
        }
    }

    out_map
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
