use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let map = include_str!("input.txt")
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
    let mut sum_edges = 0;
    for (val, shape) in shapes.iter_mut() {
        let subsum: usize = shape.iter().map(|(_, _, e)| e).sum();
        sum += subsum * shape.len();

        let mut shape_points = HashSet::new();
        shape.iter().for_each(|(x, y, _)| { 
            shape_points.insert((*x as i32, *y as i32)); 
        });

        let mut points = 0;
        for point in shape.iter() {
            //check 4x2 cases: corner
            // . .
            // . A
            // if air to left and above: point
            // or
            // . A .
            // . A A
            // if air diagonal and shape above or right

            let px: i32 = point.0 as i32;
            let py: i32 = point.1 as i32;
            let n1x: i32 = px - 1;
            let n1y: i32 = py - 1;
            let n2x: i32 = px + 1;
            let n2y: i32 = py + 1;

            let combos_edge = [
                ((n1x, py), (px, n1y)), //left and above empty
                ((px, n1y), (n2x, py)), //above and right empty
                ((n2x, py), (px, n2y)), //right and below empty
                ((px, n2y), (n1x, py)), //below and left empty
            ];

            for ((p1x,p1y), (p2x, p2y)) in combos_edge.iter() {
                /*
                if *p1x < 0 || *p1y < 0 || *p2x < 0 || *p2y < 0 {
                    continue;
                }
                if *p1x as usize >= map[0].len() || *p2x as usize >= map[0].len() {
                    continue;
                }
                if *p1y as usize >= map.len() || *p2y as usize >= map.len() {
                    continue;
                }
                */

                if !shape_points.contains(&(*p1x, *p1y)) && 
                    !shape_points.contains(&(*p2x, *p2y)) {
                    points += 1;
                }
            }

            let combos_corner = [
                ((px, n1y), (n2x, py), (n2x, n1y)), // above and right exist
                ((n2x, py), (px, n2y), (n2x, n2y)), // right and below exist
                ((px, n2y), (n1x, py), (n1x, n2y)), // below and left exist
                ((n1x, py), (px, n1y), (n1x, n1y)), // left and above exist
            ];

            for ((p1x, p1y), (p2x, p2y), (p3x, p3y)) in combos_corner.iter() {
                if shape_points.contains(&(*p1x, *p1y)) &&
                   shape_points.contains(&(*p2x, *p2y)) &&
                   !shape_points.contains(&(*p3x, *p3y)) {
                    points += 1;
                }
            }
        }
        println!("{:?} {}", shape, points);
        sum_edges += points * shape.len();

        // First attempt: count differences between intervals: doesn't work
        /*
        // Not strictly needed, should be sorted already ?
        shape.sort_by(|a, b| {
            if a.1 == b .1 {
                return a.0.cmp(&b.0);
            }

            a.1.cmp(&b.1)
        });
        println!("{:?} {:?} {:?}", val, shape, subsum * shape.len());

        let mut intervals = Vec::new();
        let mut intervals_row = Vec::new();
        let mut current_y = shape[0].1;
        let mut start = shape[0].0 as i32;
        let mut end = (shape[0].0 + 1) as i32;

        println!("Start: {} {} {}", current_y, start, end);

        for (x, y, _) in shape.iter() {
            println!("Processing: {} {}: {} {}", x, y, start, end);
            if *y != current_y {
                println!("Have moving to new row {}  {}", start, end);
                intervals_row.push((start, end));
                intervals.push(intervals_row);
                intervals_row = Vec::new();
                start = *x as i32;
                end = (*x + 1) as i32;
            }
            if *x as i32 > end {
                println!("Have interval {} > {}", x, end);
                intervals_row.push((start, end));
                start = *x as i32;
            }
            current_y = *y;
            end = (*x + 1) as i32;
        }
        intervals.push([(start, end)].to_vec());

        println!("{:?}", intervals);

        println!("----------------------");

        let mut previous_row = &[(-1, 100000000)].to_vec();

        let mut edges = 0;
        for row in intervals.iter() {
            let mut new_edges = 0;

            for interval in row.iter() {
                let mut left_match = true;
                let mut right_match = true;
                for previous_interval in previous_row.iter() {
                    if interval.0 == previous_interval.0 {
                        left_match = false;
                    }
                    if interval.1 == previous_interval.1 {
                        right_match = false;
                    }
                }

                if left_match {
                    new_edges += 1;
                }
                if right_match {
                    new_edges += 1;
                }
            }

            println!("Row has {} edges", new_edges);
            edges += new_edges;
            previous_row = row;
        }

        println!("total edges: {:?}", edges * 2);
        sum_edges += edges * 2 * shape.len()
        */
    }

    println!("{:?} {:?}", sum, sum_edges);
}
