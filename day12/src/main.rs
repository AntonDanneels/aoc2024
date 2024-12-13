use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let map = include_str!("sample.txt")
        .lines()
        .map(|line| {
            line.chars()
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let mut visited = HashMap::new();
    let mut shapes =  Vec::new();

    let height = map.len();
    let width = map[0].len();

    println!("{:?}", map);

    for (y, row) in map.iter().enumerate() {
        for (x, val) in row.iter().enumerate()  {
            if visited.contains_key(&(x, y)) {
                continue;
            }

            let mut nodes = VecDeque::new();
            nodes.push_back((x, y));

            // suboptimal: check row per row, but this also works
            let mut current_shape = Vec::new();
            while !nodes.is_empty() {
                let (nx, ny) = nodes.pop_front().unwrap();
                let mut edges = 4;

                if visited.contains_key(&(nx, ny)) {
                    continue;
                }
                visited.insert((nx, ny), true);

                if nx as i32 - 1 >= 0 && map[ny][nx - 1] == *val {
                    edges -= 1;
                    nodes.push_back((nx - 1, ny));
                }
                if nx + 1 < width && map[ny][nx + 1] == *val {
                    nodes.push_back((nx + 1, ny));
                    edges -= 1;
                }
                if ny as i32 - 1 >= 0 && map[ny - 1][nx] == *val {
                    edges -= 1;
                    nodes.push_back((nx, ny - 1));
                }
                if ny + 1 < height && map[ny + 1][nx] == *val {
                    nodes.push_back((nx, ny + 1));
                    edges -= 1;
                }

                current_shape.push((nx, ny, edges));
            }

            shapes.push((val, current_shape));
        }
    }

    let mut sum = 0;
    for (val, shape) in shapes.iter_mut() {
        let subsum: usize = shape.iter().map(|(_, _, e)| e).sum();
        println!("{:?} {:?} {:?}", val, shape, subsum * shape.len());

        sum += subsum * shape.len();

        // Not strictly needed, should be sorted already ?
        shape.sort_by(|a, b| {
            if a.1 == b .1 {
                return a.0.cmp(&b.0);
            }

            a.1.cmp(&b.1)
        });

        let mut intervals = Vec::new();
        let mut intervals_row = Vec::new();
        let mut current_y = shape[0].1;
        let mut start = shape[0].0;
        let mut end = shape[0].0 + 1;

        println!("Start: {} {} {}", current_y, start, end);

        for (x, y, _) in shape.iter() {
            if *y != current_y {
                intervals_row.push((start, end));
                intervals.push(intervals_row);
                intervals_row = Vec::new();
                start = *x;
            }
            if *x > end {
                intervals_row.push((start, end));
                start *= x;
            }
            current_y = *y;
            end = *x + 1;
        }
        intervals.push([(start, end)].to_vec());

        println!("{:?}", intervals);

        println!("----------------------");

    }

    println!("{:?}", sum);
}
