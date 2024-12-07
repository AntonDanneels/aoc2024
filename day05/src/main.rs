use std::collections::{BinaryHeap, HashMap};

fn main() {
    let input = include_str!("input1.txt");

    let mut rules = HashMap::new();
    let mut updates = Vec::new();

    input.lines().for_each(|line| {
        if line.contains('|') {
            let parts: Vec<u32> = line
                .split('|')
                .map(|part| u32::from_str_radix(part, 10).unwrap())
                .collect();

            let heap = rules.entry(parts[0]).or_insert(BinaryHeap::new());
            heap.push(parts[1]);
        } else if line.contains(',') {
            let parts: Vec<u32> = line
                .split(',')
                .map(|part| u32::from_str_radix(part, 10).unwrap())
                .collect();
            updates.push(parts);
        }
    });

    let rules: HashMap<u32, Vec<u32>> = rules
        .into_iter()
        .map(|(k, v)| (k, v.into_sorted_vec()))
        .collect();

    println!("{:?}", rules);
    println!("{:?}", updates);

    let (valid_updates, mut invalid_updates): (Vec<Vec<u32>>, Vec<Vec<u32>>) =
        updates.into_iter().partition(|update| {
            for (i, v) in update.iter().enumerate() {
                for j in i + 1..update.len() {
                    let heap = match rules.get(&update[j]) {
                        Some(heap) => heap,
                        None => continue,
                    };

                    if heap.binary_search(v).is_ok() {
                        return false;
                    }
                }
            }

            true
        });

    println!("{:?} {}", valid_updates, valid_updates.len());

    let total: u32 = valid_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum();

    println!("{}", total);

    println!("{:?} {}", invalid_updates, invalid_updates.len());

    invalid_updates.iter_mut().for_each(|update| {
        update.sort_by(|a, b| {
            match rules.get(&b) {
                Some(heap) => {
                    if heap.binary_search(a).is_ok() {
                        return std::cmp::Ordering::Greater;
                    }
                }
                None => {}
            };

            match rules.get(&a) {
                Some(heap) => {
                    if heap.binary_search(b).is_ok() {
                        return std::cmp::Ordering::Less;
                    }
                }
                None => {}
            };

            std::cmp::Ordering::Equal
        });
    });

    println!("{:?}", invalid_updates);

    let total: u32 = invalid_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum();
    println!("{}", total);
}
