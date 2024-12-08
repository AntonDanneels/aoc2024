use std::str::FromStr;

#[derive(Debug)]
enum Operator {
    Mul,
    Add,
    Concat,
}

fn get_concat_multiplier(number: u64) -> u64 {
    let mut count = 1;
    let mut tmp = number;

    while tmp != 0 {
        tmp /= 10;
        count *= 10;
    }

    count
}

fn is_computable(target: u64, current: u64, index: usize, entries: &[u64]) -> bool {
    for operator in [Operator::Mul, Operator::Concat, Operator::Add] {
        let mut new_current = current;
        match operator {
            Operator::Mul => {
                new_current *= entries[index];
            }
            Operator::Concat => {
                new_current = get_concat_multiplier(entries[index]) * current + entries[index];
            }
            Operator::Add => {
                new_current += entries[index];
            }
        }

        if new_current > target {
            continue;
        }

        if index == entries.len() - 1 {
            if new_current == target {
                return true;
            }
            continue;
        }

        if is_computable(target, new_current, index + 1, entries) {
            return true;
        }
    }

    false
}

fn main() {
    let data = include_str!("input1.txt")
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(":").collect();
            let entries: Vec<u64> = parts[1]
                .split_whitespace()
                .map(|part| u64::from_str(part).unwrap())
                .collect();

            (u64::from_str(parts[0]).unwrap(), entries)
        })
        .collect::<Vec<(u64, Vec<u64>)>>();

    println!("{:?}", data);

    let valid_entries = data
        .iter()
        .filter(|(result, entries)| {
            if entries.len() < 2 {
                return *result == entries[0];
            } else if entries.len() == 2 {
                return *result == entries[0] + entries[1]
                    || *result == entries[0] * entries[1]
                    || *result == get_concat_multiplier(entries[1]) * entries[0] + entries[1];
            }

            let sum = entries[0];
            is_computable(*result, sum, 0, &entries[1..])
        })
        .collect::<Vec<&(u64, Vec<u64>)>>();

    println!("{:?}", valid_entries);

    let total_sum: u64 = valid_entries.iter().map(|(result, _)| result).sum();

    println!("{:?}", total_sum);
}
