use std::str::FromStr;
use std::collections::HashMap;

fn count_digits(number: u64) -> u64 {
    let mut count = 0;
    let mut tmp = number;

    while tmp > 0 {
        count += 1;
        tmp /= 10;
    }

    count
}

fn mask_digits(number: u64, start: u64, end: u64) -> u64 {
    let mut count = 1;
    let mut result = 0;
    for i in start..end {
        let digit = (number / 10_u64.pow(i as u32)) % 10;
        result = result + digit * count;
        count *= 10;
    }

    result
}

fn blink(stones: &Vec<u64>) -> Vec<u64> {
    let mut result = Vec::new();

    for stone in stones.iter() {
        let digits = count_digits(*stone);
        if *stone == 0 {
            result.push(1);
        } else if digits % 2 == 0 {
            let left_half = mask_digits(*stone, digits / 2, digits);
            let right_half = mask_digits(*stone, 0, digits / 2);

            result.push(left_half);
            result.push(right_half);
        } else {
            result.push(*stone * 2024);
        }
    }

    result
}

fn blink_number(number: u64, height: usize, memory: &mut HashMap<(u64, usize), usize>) -> usize {
    if let Some(result) = memory.get(&(number, height)) {
        return *result;
    }

    let digits = count_digits(number);
    if height == 0 {
        if number == 0 {
            memory.insert((number, height), 1);
            return 1;
        } else if digits % 2 == 0 {
            memory.insert((number, height), 2);
            return 2;
        } else {
            memory.insert((number, height), 1);
            return 1;
        }
    }

    if number == 0 {
        let result = blink_number(1, height - 1, memory);
        memory.insert((number, height), result);
        return result;
    } else if digits % 2 == 0 {
        let left_half = mask_digits(number, digits / 2, digits);
        let right_half = mask_digits(number, 0, digits / 2);
        let result = blink_number(left_half, height - 1, memory) + blink_number(right_half, height - 1, memory);
        memory.insert((number, height), result);
        return result;
    } else {
        let result = blink_number(number * 2024, height - 1, memory);
        memory.insert((number, height), result);
        return result;
    }
}

fn main() {
    let data1 = "0 1 10 99 999";
    let data2 = "125 17";
    let input = "17639 47 3858 0 470624 9467423 5 188";
    let data = input;

    let stones = data.split_whitespace()
                     .map(|token| u64::from_str(token).unwrap())
                     .collect::<Vec<u64>>();

    println!("{:?}", stones);

    // brute force 1st
    /*
    let mut tmp = stones;
    for i in 0..75 {
        tmp = blink(&tmp);
        println!("{:?}: {:?}", i, tmp.len());
    }

    println!("{:?}", tmp.len());
    */

    let iterations = 74;

    // memoize 2nd
    let mut memory = HashMap::new();
    let mut sum = 0;
    for stone in stones {
        let result = blink_number(stone, iterations, &mut memory);
        sum += result;
    }
    println!("{:?}", sum);
    println!("{:?}", memory.len());

}
