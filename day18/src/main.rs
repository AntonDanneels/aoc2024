use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Point {
    cost: usize,
    position: (usize, usize)
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_path(map: &Vec<Vec<bool>>, width: usize, height: usize) -> Option<usize> {
    let mut pqueue = BinaryHeap::new();
    let mut dist = HashMap::new();
    let mut prev: HashMap<(usize, usize), Option<Point>> = HashMap::new();

    for y in 0..height {
        for x in 0..height {
            if map[y][x] {
                continue;
            }

            if x == 0 && y == 0 {
                dist.insert((0,0), 0);
            } else {
                dist.insert((x, y), 1000000000);
                prev.insert((x, y), None);
            }
        }
    }

    pqueue.push(Point {cost: 0, position: (0, 0)});
    while let Some(p) = pqueue.pop() {
        if p.position.0 == width - 1 &&
            p.position.1 == height - 1 {
            break;
        }

        let px = p.position.0 as i32;
        let py = p.position.1 as i32;
        let n1x = px as i32 - 1;
        let n1y = py as i32 - 1;
        let n2x = px as i32 + 1;
        let n2y = py as i32 + 1;

        let neighbours = [
            (px, n1y),
            (n2x, py),
            (px, n2y),
            (n1x, py),
        ];

        for (nx, ny) in neighbours.iter() {
            if *nx < 0 || *ny < 0 {
                continue;
            }
            let nx = *nx as usize;
            let ny = *ny as usize;
            if nx >= width || ny >= height {
                continue;
            }
            if map[ny][nx] {
                continue;
            }

            let alt = dist.get(&p.position).unwrap() + 1;

            if alt < *dist.get(&(nx, ny)).unwrap() {
                prev.insert((nx, ny), Some(p));
                dist.insert((nx, ny), alt);
                pqueue.push(Point {cost: alt, position: (nx, ny)});
            }
        }
    }

    if let Some(x) = prev.get(&(width - 1, height -1)) {
        match x {
            Some(p) => { return Some(p.cost); },
            _ => {}
        }
    }

    None

    /*
    let mut current = (width - 1, height - 1);
    let mut points = HashMap::new();
    loop {
        if let Some(p) = prev.get(&current) {
            points.insert(p.unwrap().position, true);
            println!("{:?}", p);
            current = p.unwrap().position;
        } else {
            break;
        }
    }
    */
}

fn create_map(width: usize, height: usize, points:&[(usize, usize)]) -> Vec<Vec<bool>> {
    let mut map = Vec::new();

    for _ in 0..height {
        let mut row = Vec::new();
        for _ in 0..width {
            row.push(false);
        }
        map.push(row);
    }

    for (x,y) in points.iter() {
        map[*y][*x] = true;
    }

    map
}

fn main() {
    let mut map = Vec::new();
    let width = 71;
    let height = 71;

    for _ in 0..height {
        let mut row = Vec::new();
        for _ in 0..width {
            row.push(false);
        }
        map.push(row);
    }

    let points = include_str!("input.txt")
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let coords = line.split(",")
                              .map(|part| {
                                  usize::from_str(part).unwrap()
                              }).collect::<Vec<usize>>();

            (coords[0], coords[1])
        }).collect::<Vec<(usize, usize)>>();

    for y in 0..height {
        for x in 0..width {
            if map[y][x] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    let map = create_map(width, height, &points[0..12]);
    println!("{:?}", find_path(&map, width, height));

    let mut current = points.len() / 2;
    let mut range = points.len() / 2;

    loop {
        let map = create_map(width, height, &points[0..current]);
        let path = find_path(&map, width, height);

        range /= 2;

        if range == 0 {
            println!("{:?} {:?}", current, points[current]);
            break;
        }

        match path {
            Some(_) => {
                current = current + range;
            },
            None => {
                current = current - range;
            }
        }
    }
}
