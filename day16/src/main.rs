use std::collections::{HashSet, HashMap, VecDeque};

#[derive(PartialEq)]
enum Direction {
    East,
    South,
    West,
    North
}

fn get_next_dir(p1: &(usize, usize), p2: &(usize, usize)) -> Direction {
    let (p1x, p1y) = *p1;
    let (p2x, p2y) = *p2;

    let dx = p2x as i32 - p1x as i32;
    let dy = p2y as i32 - p1y as i32;

    match (dx, dy) {
        (-1, 0) => {
            Direction::West
        },
        (1, 0) => {
            Direction::East
        },
        (0, 1) => {
            Direction::South
        },
        (0, -1) => {
            Direction::North
        },
        _ => panic!(),
    }
}

fn dfs(current_point: &(usize, usize), 
       target: &(usize, usize),
       map: &Vec<Vec<bool>>,
       visited: &mut HashSet<(usize, usize)>,
       paths: &mut Vec<Vec<(usize, usize)>>,
       current_score: usize,
       current_dir: Direction,
       best_score: &mut usize) {

    visited.insert(*current_point);
    //println!("Visiting: {:?} {}", current_point, score);

    if current_point == target {
        println!("Reached target! {}, {}", 
                 current_score, 
                 best_score);

        for (y, row) in map.iter().enumerate() {
            for (x, walkable) in row.iter().enumerate() {
                if visited.contains(&(x, y)) {
                    print!("X");
                } else if *walkable {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!();
        }

        paths.push(visited.clone().iter().map(|x| *x).collect());

        if current_score < *best_score {
            *best_score = current_score;
        }

        visited.remove(&current_point);
        return;
    }

    let (x, y) = current_point;
    let neighbours = [
        (*x as i32, *y as i32 - 1),
        (*x as i32 - 1, *y as i32),
        (*x as i32 + 1, *y as i32),
        (*x as i32, *y as i32 + 1),
    ];

    for (nx, ny) in neighbours.iter() {
        if *nx < 0 || *ny < 0 || 
            *nx as usize >= map[0].len() || *ny as usize >= map.len() {
            continue;
        }
        let nx = *nx as usize;
        let ny = *ny as usize;

        //println!("Neighbour: {:?}, {}", (nx, ny), map[ny][nx]);

        let new_dir = get_next_dir(current_point, &(nx, ny));
        let mut delta_score = 1;
        if new_dir != current_dir {
            delta_score += 1000;
        }
        let new_score = current_score + delta_score;
        if new_score > *best_score {
            continue;
        }

        if map[ny][nx] && !visited.contains(&(nx, ny)) {
            dfs(&(nx, ny),
                target,
                map,
                visited,
                paths,
                new_score,
                new_dir,
                best_score);
        }
    }
    visited.remove(&current_point);
}

fn find_paths(start_point: &(usize, usize),
              end_point: &(usize, usize),
              map: &Vec<Vec<bool>>,
              best_score: usize) -> Vec<Vec<(usize, usize)>> {
    let mut visited = HashSet::new();
    let mut paths = Vec::new();
    let mut best_score = best_score;

    dfs(start_point, end_point, map, &mut visited, &mut paths, 
        0,
        Direction::East,
        &mut best_score);

    println!("{} {}", paths.len(), best_score);

    paths
}

fn score_map(start_point: &(usize, usize),
             end_point: &(usize, usize),
             map: &Vec<Vec<bool>>) -> usize {

    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back((*start_point, 0, None));
    //let mut current_dir = None;

    let mut count = 0;
    while let Some((current_point, score, current_dir)) = queue.pop_front() {
        println!("Visited: {:?} {}", current_point, score);
        visited.insert(current_point, score);
        count += 1;

        if current_point == *end_point {
            continue;
        }

        let (x, y) = current_point;
        let neighbours = [
            (x as i32, y as i32 + 1),
            (x as i32 - 1, y as i32),
            (x as i32 + 1, y as i32),
            (x as i32, y as i32 - 1),
        ];

        for (nx, ny) in neighbours.iter() {
            if *nx < 0 || *ny < 0 || 
                *nx as usize >= map[0].len() || *ny as usize >= map.len() {
                continue;
            }
            let nx = *nx as usize;
            let ny = *ny as usize;

            //println!("Neighbour: {:?}, {}", (nx, ny), map[ny][nx]);

            let new_dir = Some(get_next_dir(&current_point, &(nx, ny)));
            let mut delta_score = 1;
            if current_dir.is_some() && new_dir != current_dir {
                delta_score += 1000;
            }
            let new_score = score + delta_score;
            //current_dir = new_dir;
            //let current_score = ; 

            match visited.get(&(nx, ny)) {
                None => {},
                Some(current_score) => {
                    if new_score > *current_score {
                        continue;
                    }
                }
            };

            if map[ny][nx] {
                queue.push_back(((nx, ny), new_score, new_dir));
            }
        }
    }

    println!("Visited: {:?}, {}", visited, count);

    *visited.get(end_point).unwrap()
}

fn main() {
    let mut map = Vec::new();
    let mut start_point = (0, 0);
    let mut end_point = (0, 0);
    include_str!("input.txt")
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            let row = line.chars().enumerate().map(|(x, c)| {
                match c {
                    '#' => {
                        false
                    },
                    '.' => {
                        true
                    },
                    'S' => {
                        start_point = (x, y);
                        true
                    },
                    'E' => {
                        end_point = (x, y);
                        true
                    }
                    _ => {panic!()}
                }
            })
            .collect::<Vec<bool>>();

            map.push(row);
        });

    // walk backwards, scoring each point based on turns/distance
    let best_score = score_map(&end_point, 
                               &start_point, 
                               &map);

    println!("Best score: {}", best_score);
}
