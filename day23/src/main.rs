use std::collections::{HashMap, HashSet};

fn main() {
    let mut links = HashMap::new();

    include_str!("input.txt")
        .lines()
        .for_each(|line| {
            let parts: Vec<&str> = line.split("-").map(|part| part.trim()).collect();

            let l = links.entry(parts[0]).or_insert(Vec::new());
            l.push(parts[1]);
            let l = links.entry(parts[1]).or_insert(Vec::new());
            l.push(parts[0]);
        });

    println!("{:?}", links);

    //println!("{} {:?}", link, neighbours);
    //let mut unique_clusters = HashSet::new();
    for (link, neighbours) in links.iter() {
        println!("{:?}", neighbours.len());
        if link.starts_with("t") {
            /*
            for i in 0..neighbours.len() {
                for j in i + 1..neighbours.len() {
                    if links.get(neighbours[j]).unwrap().contains(&neighbours[i]) {
                        println!("{} {} {}",
                                 link, neighbours[i], neighbours[j]);
                        let mut triplet = [link, neighbours[i], neighbours[j]].to_vec();
                        triplet.sort();
                        unique_clusters.insert(triplet);
                    }
                }
            }
            */
        }
    }

    //println!("{:?}", unique_clusters.len());
}
