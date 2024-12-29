use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let mut links = HashMap::new();
    let mut clusters = VecDeque::new();

    include_str!("input.txt").lines().for_each(|line| {
        let parts: Vec<&str> = line.split("-").map(|part| part.trim()).collect();

        let l = links.entry(parts[0]).or_insert(Vec::new());
        l.push(parts[1]);
        let l = links.entry(parts[1]).or_insert(Vec::new());
        l.push(parts[0]);

        clusters.push_back(Vec::from([parts[0], parts[1]]));
    });

    println!("{:?}", links.len());

    //println!("{} {:?}", link, neighbours);
    //let mut unique_clusters = HashSet::new();
    for (link, neighbours) in links.iter() {
        //println!("{:?}", neighbours.len());
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

    let mut processed_clusters = Vec::new();
    let mut memory = HashSet::new();
    loop {
        let mut change = false;
        let cluster = match clusters.pop_front() {
            Some(x) => x,
            None => {
                break;
            }
        };

        //println!("{:?}", cluster);

        let mut common = Vec::new();
        for (i, l) in cluster.iter().enumerate() {
            if i == 0 {
                common = links.get(l).unwrap().clone();
            } else {
                common.retain(|x| links.get(l).unwrap().contains(x) && !cluster.contains(x));
            }

            //println!("{:?}", common);
        }

        //println!("Common neighbours: {:?}", common);
        //let line = std::io::stdin().lines().next().unwrap().unwrap();

        for e in common.iter() {
            let mut new_cluster = cluster.clone();
            new_cluster.push(e);
            new_cluster.sort();
            if !memory.contains(&new_cluster) {
                clusters.push_back(new_cluster.clone());
                memory.insert(new_cluster);
                change = true;
            }
        }

        if !change {
            processed_clusters.push(cluster);
        }
    }

    println!("{:?}", clusters.len());
    println!("{:?}", processed_clusters.len());

    println!(
        "{:?}",
        clusters.iter().max_by(|c1, c2| c1.len().cmp(&c2.len()))
    );
    println!(
        "{:?}",
        processed_clusters
            .iter()
            .max_by(|c1, c2| c1.len().cmp(&c2.len()))
    );

    let cluster = processed_clusters
        .iter()
        .max_by(|c1, c2| c1.len().cmp(&c2.len()));
    let mut cluster = cluster.unwrap().clone();
    cluster.sort();

    println!("{:?}", cluster.join(","));
}
