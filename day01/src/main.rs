use std::collections::HashMap;

fn main() {
    let mut l1 = Vec::new();
    let mut l2 = HashMap::<i32, i32>::new();

    include_str!("input1.txt").lines().for_each(|line| {
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|part| part.parse::<i32>().unwrap())
            .collect();
        l1.push(parts[0]);

        *l2.entry(parts[1]).or_insert(0) += 1;
    });

    l1.sort();
    //l2.sort();

    /*
    let total = std::iter::zip(l1, l2)
        .map(|(a, b)| {
            (a - b).abs()
        }).reduce(|acc, e| {
            acc + e
        });
    */

    let total = l1
        .iter()
        .map(|entry| {
            let multiplier = l2.get(entry).unwrap_or(&0);

            multiplier * entry
        })
        .reduce(|acc, e| acc + e);

    println!("{:?}", total);
}
