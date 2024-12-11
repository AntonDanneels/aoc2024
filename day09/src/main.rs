use std::collections::VecDeque;

fn part_1() {
    let data: Vec<u32> = include_str!("input.txt")
        .lines()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
        .collect();

    let mut is_entry = true;
    let mut memory = Vec::new();
    let mut freelist = VecDeque::new();
    let mut id = 0;
    for (i, c) in data.iter().enumerate() {
        println!("{:?} {:?}", i, c);

        if is_entry {
            for _ in 0..*c {
                memory.push(Some(id));
            }

            id += 1;
        } else {
            for _ in 0..*c {
                freelist.push_back(memory.len());
                memory.push(None);
            }
        }

        is_entry = !is_entry;
    }

    println!("{:?}", memory);

    let mut index = memory.len() as i32 - 1;
    loop {
        if index < 0 {
            break;
        }
        if freelist.is_empty() {
            break;
        }
        if let Some(x) = memory[index as usize] {
            let freespot = freelist.pop_front().unwrap();
            if freespot >= index as usize {
                break;
            }

            memory[freespot] = Some(x);
            memory[index as usize] = None;
            freelist.push_back(index as usize);
        }
        //println!("{:?}", memory);
        index -= 1;
    }
    println!("{:?}", memory);

    let checksum: usize = memory
        .iter()
        .enumerate()
        .map(|(i, digit)| {
            if let Some(digit) = digit {
                return digit * i;
            }
            0
        })
        .sum();
    println!("{:?}", checksum);
}

fn part_2() {
    let data: Vec<u32> = include_str!("input.txt")
        .lines()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
        .collect();

    let mut is_entry = true;
    let mut memory = Vec::new();
    let mut freelist = VecDeque::new();
    let mut files = VecDeque::new();
    let mut id = 0;
    for (i, c) in data.iter().enumerate() {
        println!("{:?} {:?}", i, c);

        if is_entry {
            files.push_back((memory.len(), *c));
            for _ in 0..*c {
                memory.push(Some(id));
            }

            id += 1;
        } else {
            freelist.push_back((memory.len(), *c));
            for _ in 0..*c {
                memory.push(None);
            }
        }

        is_entry = !is_entry;
    }

    loop {
        if files.is_empty() {
            break;
        }

        let (start, length) = files.pop_back().unwrap();

        for (i, (free_start, free_length)) in freelist.iter().enumerate() {
            if *free_start >= start {
                continue;
            }

            if length > *free_length {
                continue;
            }

            for i in 0..length {
                memory[free_start + i as usize] = memory[start + i as usize];
                memory[start + i as usize] = None;
            }

            let remaining = free_length - length;
            if remaining > 0 {
                freelist.insert(
                    i + 1,
                    (free_start + (free_length - remaining) as usize, remaining),
                );
            }
            freelist.remove(i);

            break;
        }
    }
    println!("{:?}", memory);

    let checksum: usize = memory
        .iter()
        .enumerate()
        .map(|(i, digit)| {
            if let Some(digit) = digit {
                return digit * i;
            }
            0
        })
        .sum();
    println!("{:?}", checksum);
}

fn main() {
    //part_1();
    part_2();
}
