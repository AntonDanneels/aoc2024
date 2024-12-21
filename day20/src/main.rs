use std::cmp::Ordering;
use std::collections::{HashSet};

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
    let mut score = 0;
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

fn main() {
    let mut start_point = Vec2::empty();
    let mut end_point = Vec2::empty();
    let mut obstacles = Vec::new();
    let mut map = include_str!("sample.txt")
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    match c {
                        '.' => false,
                        'S' => {
                            start_point = Vec2::new(x, y);
                            false
                        },
                        'E' => {
                            end_point = Vec2::new(x, y);
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

    for obstacle in obstacles.iter() {
        if obstacle.x == 0 || obstacle.x == width - 2 {
            continue;
        }
        if obstacle.y == 0 || obstacle.y == height - 2 {
            continue;
        }
        let px = obstacle.x as i32;
        let py = obstacle.y as i32;
        let n1x = px as i32 - 1;
        let n1y = py as i32 - 1;
        let n2x = px as i32 + 1;
        let n2y = py as i32 + 1;

        let neighbours = [
            ((n1x, py), (n2x, py)),
            ((px, n1y), (px, n2y)),
        ];

        for ((n1x, n1y), (n2x, n2y)) in neighbours.iter() {
            if *n1x < 0 || *n1y < 0 || *n2x < 0 || *n2y < 0 {
                continue;
            }
            let n1x = *n1x as usize;
            let n1y = *n1y as usize;
            let n2x = *n2x as usize;
            let n2y = *n2y as usize;
            if n1x >= width || n1y >= height || n2x >= width || n2y >= height {
                continue;
            }
            if map[n1y][n1x] || map[n2y][n2x] {
                continue;
            }

            let new_score = 
            if score[n1y][n1x] > score[n2y][n2x] {
                score[n2y][n2x] + 1
            } else {
                score[n1y][n1x] + 1
            };

            let delta = score[start_point.y][start_point.x] - new_score;

            println!("{}", delta);
        }
    }
}
