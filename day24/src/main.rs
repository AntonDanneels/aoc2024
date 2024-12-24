use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Wire<'a> {
    And(&'a str, &'a str),
    Xor(&'a str, &'a str),
    Or(&'a str, &'a str),
    Value(bool),
}

fn compute_state<'a>(val: &'a str, values: &mut HashMap<&'a str, Wire<'a>>) -> bool {
    match values.get_mut(val).unwrap().clone() {
        Wire::And(x, y) => {
            let new_value = Wire::Value(compute_state(x, values) && compute_state(y, values));
            values.insert(val, new_value);
        },
        Wire::Or(x, y) => {
            let new_value = Wire::Value(compute_state(x, values) || compute_state(y, values));
            values.insert(val, new_value);
        },
        Wire::Xor(x, y) => {
            let new_value = Wire::Value(compute_state(x, values) ^ compute_state(y, values));
            values.insert(val, new_value);
        },
        _ => {}
    }

    if let Wire::Value(x) = values[val] {
        return x;
    }

    false
}

fn calc_number<'a>(bits: &'a Vec<&'a str>,  
                   values: &mut HashMap<&'a str, Wire<'a>>) -> u64 {
    let mut number: u64 = 0;
    for (i, target) in bits.iter().enumerate() {
        let bit = compute_state(target, values);
        if bit {
            number |= 1 << i;
        }
    }

    number
}

fn incorrect_bits(a: u64, b: u64, len: usize) -> Vec<usize> {
    let mut result = Vec::new();

    for i in 0..len {
        let x = (a >> i) & 0b1;
        let y = (b >> i) & 0b1;

        if x != y {
            result.push(i);
        }
    }

    result
}

fn main() {
    let mut values = HashMap::new();
    let mut targets = Vec::new();
    let mut input1 = Vec::new();
    let mut input2 = Vec::new();

    include_str!("sample3.txt").lines().for_each(|line| {
        if line.contains(':') {
            let parts = line.split(":").collect::<Vec<&str>>();
            values.insert(
                parts[0],
                Wire::Value(if parts[1].trim() == "1" { true } else { false }),
            );

            if parts[0].starts_with("x") {
                input1.push(parts[0]);
            }
            if parts[0].starts_with("y") {
                input2.push(parts[0]);
            }

        } else if line.contains("->") {
            let parts = line.split("->").collect::<Vec<&str>>();
            let op = parts[0].split_whitespace().collect::<Vec<&str>>();

            let target = parts[1].trim();

            if target.starts_with("z") {
                targets.push(target);
            }

            match op[1] {
                "AND" => {
                    values.insert(target, Wire::And(op[0], op[2]));
                }
                "OR" => {
                    values.insert(target, Wire::Or(op[0], op[2]));
                }
                "XOR" => {
                    values.insert(target, Wire::Xor(op[0], op[2]));
                }
                _ => panic!(),
            }
        }
    });

    println!("{:?}", values);

    targets.sort_by(|x, y| {
        u64::from_str(&x[1..]).unwrap().cmp(&u64::from_str(&y[1..]).unwrap())
    });

    input1.sort_by(|x, y| {
        u64::from_str(&x[1..]).unwrap().cmp(&u64::from_str(&y[1..]).unwrap())
    });
    input2.sort_by(|x, y| {
        u64::from_str(&x[1..]).unwrap().cmp(&u64::from_str(&y[1..]).unwrap())
    });
    println!("{:?} = {:?} + {:?}", targets, input1, input2);

    let number = calc_number(&targets, &mut values.clone());
    let a = calc_number(&input1, &mut values.clone());
    let b = calc_number(&input2, &mut values.clone());
    let target_number = a + b;

    println!("{} + {} ?= {:10b} {:10b}", a, b, number, target_number);
    println!("{:?}", incorrect_bits(number, target_number, targets.len()));
}
