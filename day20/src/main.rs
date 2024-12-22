use std::cmp::Ordering;
use std::collections::{HashSet, HashMap};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Vec2 {
    x: usize,
    y: usize,
}

impl Vec2 {
    fn empty() -> Vec2 {
        Vec2 {
            x: 0,
            y: 0
        }
    }

    fn new(x: usize, y: usize) -> Vec2 {
        Vec2 {
            x,
            y
        }
    }
}

impl Ord for Vec2 {
    fn cmp(&self, other: &Self) -> Ordering {
        other.x.cmp(&self.x)
            .then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for Vec2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn walk(start: &Vec2, end: &Vec2, map: &Vec<Vec<bool>>,
        width: usize, height: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    for row in map.iter() {
        let mut new_row = Vec::new();
        for _ in row.iter() {
            new_row.push(0);
        }
        result.push(new_row);
    }

    let mut current_pos = start.clone();
    let mut visited = HashSet::new();
    let mut score = 1;
    visited.insert(start.clone());
    loop {
        if current_pos == *end {
            break;
        }
        let px = current_pos.x as i32;
        let py = current_pos.y as i32;
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
            let new_pos = Vec2::new(nx, ny);
            if visited.contains(&new_pos) {
                continue;
            }
            visited.insert(new_pos);
            current_pos = new_pos;
            result[ny][nx] = score;
            score += 1;
        }
    }

    result
}

fn get_neighbour(map: &Vec<Vec<bool>>, width: usize, height: usize,
                 point: &Vec2,
                 delta_x: i32, delta_y: i32) -> Option<Vec2> {
    let px = point.x as i32;
    let py = point.y as i32;
    let nx = px as i32 + delta_x;
    let ny = py as i32 + delta_y;

    if nx < 0 || ny < 0 {
        return None;
    }
    let nx = nx as usize;
    let ny = ny as usize;

    if nx >= width || ny >= height {
        return None;
    }

    Some(Vec2::new(nx, ny))
}

fn dist(a: &Vec2, b: &Vec2) -> usize {
    let ax = a.x as i32;
    let ay = a.y as i32;
    let bx = b.x as i32;
    let by = b.y as i32;

    ((bx - ax).abs() + (by - ay).abs()) as usize
}

fn get_reachable_neighbours(map: &Vec<Vec<bool>>, width: usize, height: usize,
                            pos: &Vec2, radius: usize) -> Vec<Vec2> {
    let mut result = Vec::new();

    let radius = radius as i32;
    for y in 0..=(radius * 2) {
        for x in 0..=(radius * 2) {
            let yy = y - radius;
            let xx = x - radius;
            let nx = pos.x as i32 + xx;
            let ny = pos.y as i32 + yy;
            if nx < 0 || ny < 0 || nx as usize >= width || ny as usize >= height {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;
            if map[ny][nx] || (nx == pos.x && ny == pos.y) {
                continue;
            }
            let n = Vec2::new(nx, ny);
            if dist(&n, pos) <= radius as usize {
                result.push(n);
            }
        }
    }

    result
}

fn main() {
    let mut start_point = Vec2::empty();
    let mut end_point = Vec2::empty();
    let mut obstacles = Vec::new();
    let mut path = Vec::new();
    let mut map = include_str!("input.txt")
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    match c {
                        '.' => {
                            path.push(Vec2::new(x, y));
                            false
                        },
                        'S' => {
                            start_point = Vec2::new(x, y);
                            path.push(Vec2::new(x, y));
                            false
                        },
                        'E' => {
                            end_point = Vec2::new(x, y);
                            path.push(Vec2::new(x, y));
                            false
                        },
                        '#' => {
                            obstacles.push(Vec2::new(x, y));
                            true
                        },
                        _ => panic!(),
                    }
                })
                .collect()
        }).collect::<Vec<Vec<bool>>>();

    let width = map[0].len();
    let height = map.len();

    let score = walk(&end_point, &start_point, &map, width, height);

    for y in 0..height {
        for x in 0..width {
            if start_point.x == x && start_point.y == y {
                print!("S");
            } else if end_point.x == x && end_point.y == y {
                print!("E");
            } else if map[y][x] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!("{:?}({}) -> {:?}({})",
             start_point, score[start_point.y][start_point.x],
             end_point, score[end_point.y][end_point.x],);

    let mut scores = HashMap::new();
    let mut sum = 0;

    for node in path.iter() {
        let neighbours = get_reachable_neighbours(&map, width, height, &node, 20);
        for neighbour in neighbours.iter() {
            if score[neighbour.y][neighbour.x] >= score[node.y][node.x] {
                continue;
            }

            let score_start = score[start_point.y][start_point.x];
            let new_score = (score_start - score[node.y][node.x]) + score[neighbour.y][neighbour.x] + dist(node, neighbour);
            let delta = score_start - new_score;
            if delta >= 100 {
                *scores.entry(delta).or_insert(0) += 1;
                sum += 1;
            }
        }
    }

    println!("{:?} {}", scores, sum);
    //println!("{}", sum);
}
