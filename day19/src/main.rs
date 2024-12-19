use std::collections::HashMap;

fn is_possible(target: &str, patterns: &HashMap<char, Vec<&str>>) -> Option<()> {
    let mut ends = false;
    for (_, list) in patterns.iter() {
        for pattern in list.iter() {
            if target.ends_with(pattern) {
                ends = true;
                break;
            }
        }

        if ends {
            break;
        }
    }

    if !ends {
        return None;
    }

    Some(())
}

fn find_matching_towels(
    target: &str,
    patterns: &HashMap<char, Vec<&str>>,
    current_str: &mut Vec<String>,
    memory: &mut HashMap<String, usize>,
) -> Option<usize> {
    //println!("{}", target);
    if target.is_empty() {
        return Some(1);
    }

    if let Some(x) = memory.get(target) {
        return Some(*x);
    }

    let c = target.chars().nth(0).unwrap();

    if !patterns.contains_key(&c) {
        return None;
    }

    let mut sum = 0;
    for pattern in patterns.get(&c).unwrap().iter() {
        if pattern.len() > target.len() {
            continue;
        }
        if target.starts_with(pattern) {
            current_str.push(pattern.to_string());
            if let Some(x) =
                find_matching_towels(&target[pattern.len()..], patterns, current_str, memory)
            {
                sum += x;
            }
            current_str.pop();
        }
    }
    memory.insert(target.to_string(), sum);

    Some(sum)
}

fn main() {
    let mut towel_patterns: HashMap<char, Vec<&str>> = HashMap::new();
    let mut assignments = Vec::new();

    include_str!("input.txt").lines().for_each(|line| {
        if line.contains(",") {
            line.split(",").map(|part| part.trim()).for_each(|part| {
                let v = towel_patterns
                    .entry(part.chars().nth(0).unwrap())
                    .or_insert(Vec::new());

                println!("Insert {} into {:?}", part, v);

                match v.binary_search_by(|other: &&str| part.len().cmp(&other.len())) {
                    Ok(pos) => v.insert(pos, part),
                    Err(pos) => v.insert(pos, part),
                }
            });
        } else if line.len() != 0 {
            assignments.push(line);
        }
    });

    println!("{:?}", towel_patterns);
    println!("{:?}", assignments);

    let mut sum = 0;
    for assignment in assignments.iter() {
        println!("Trying: {assignment}");
        if is_possible(&assignment, &towel_patterns).is_some() {
            let mut current = Vec::new();
            let mut memory = HashMap::new();
            let x = find_matching_towels(&assignment, &towel_patterns, &mut current, &mut memory);
            println!(
                "Design for {}: {:?} {:?}",
                assignment, x, memory
            );
            if let Some(x) = x {
                sum += x;
            }
        } else {
            println!("Design is impossible for {}", assignment);
        }
    }

    println!("{}", sum);
}
