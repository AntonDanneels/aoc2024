use std::collections::HashMap;
use std::str::FromStr;

fn prune(v: u64) -> u64 {
    return v % 16777216;
}

fn mix(v: u64, s: u64) -> u64 {
    return v ^ s;
}

fn next_secret(current: u64) -> u64 {
    let mut secret = prune(mix(current * 64, current));
    secret = prune(mix(secret / 32, secret));
    secret = prune(mix(secret * 2048, secret));
    prune(secret)
}

fn last_digit(number: u64) -> u64 {
    number % 10
}

fn encode(window: &[(u64, i32)]) -> u64 {
    let window = window
        .iter()
        .map(|(_, x)| (x + 9) as u64)
        .collect::<Vec<u64>>();
    window[0] | window[1] << 8 | window[2] << 16 | window[3] << 24
}

fn decode(val: u64) -> Vec<i32> {
    [
        ((val >> 0) & 0xff) as i32 - 9,
        ((val >> 8) & 0xff) as i32 - 9,
        ((val >> 16) & 0xff) as i32 - 9,
        ((val >> 24) & 0xff) as i32 - 9,
    ]
    .to_vec()
}

fn main() {
    let secrets = [1, 10, 100, 2024];

    let input = include_str!("input.txt")
        .lines()
        .map(|line| u64::from_str(line).unwrap())
        .collect::<Vec<u64>>();

    /*
    let input = [123_u64];

    let input = [
        1,
        2,
        3,
        2024
    ];
    */

    let mut sum = 0;
    let mut all_deltas = Vec::new();
    for secret in input.iter() {
        let mut deltas = Vec::new();
        let mut secret = *secret;
        let mut digit = last_digit(secret);
        for _ in 0..2000 {
            secret = next_secret(secret);
            let new_digit = last_digit(secret);
            let delta = new_digit as i32 - digit as i32;
            deltas.push((new_digit, delta));
            digit = new_digit;
        }
        //println!("{}", secret);
        //println!("{:?}", deltas.len());

        let mut deltas = deltas
            .windows(4)
            .enumerate()
            .map(|(i, window)| {
                /*
                println!("{} {:?} {} {}",
                             i,
                             window, encode(window), window[3].0);
                             */
                (encode(window), window[3].0)
            })
            .collect::<Vec<(u64, u64)>>();

        deltas.sort_by(|a, b| a.0.cmp(&b.0));

        all_deltas.push(deltas);
        sum += secret;
    }

    println!("{}", sum);
    println!("{:?}", all_deltas.len());

    let mut best_sequence = HashMap::new();

    for deltas in all_deltas.iter() {
        let mut highest = HashMap::new();
        for (d, v) in deltas.iter() {
            highest.entry(d).or_insert(v);
        }

        for (d, v) in highest.iter() {
            *best_sequence.entry(*d).or_insert(0) += *v;
        }
    }

    let best = best_sequence.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
    println!("{:?}, {:?}", best, decode(**best.0));
}
