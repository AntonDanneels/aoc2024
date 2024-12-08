use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn main() {
    let mut initial_position = (-1, -1);
    let mut direction = Direction::Up;
    let map: Vec<Vec<bool>> = include_str!("input1.txt")
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '#' {
                        return true;
                    }
                    if c == '^' {
                        initial_position = (x as i32, y as i32);
                    }

                    false
                })
                .collect()
        })
        .collect();

    let mut visited: HashMap<(i32, i32), Direction> = HashMap::new();
    let mut position = initial_position;
    loop {
        visited.insert(position.clone(), direction.clone());

        let (next_position, next_direction) = match direction {
            Direction::Up => ((position.0, position.1 - 1), Direction::Right),
            Direction::Right => ((position.0 + 1, position.1), Direction::Down),
            Direction::Down => ((position.0, position.1 + 1), Direction::Left),
            Direction::Left => ((position.0 - 1, position.1), Direction::Up),
        };

        if next_position.0 < 0
            || next_position.0 >= map[0].len() as i32
            || next_position.1 < 0
            || next_position.1 >= map.len() as i32
        {
            break;
        }

        if map[next_position.1 as usize][next_position.0 as usize] {
            direction = next_direction;
        } else {
            position = next_position;
        }
    }

    let mut blocks = Vec::new();
    for (extra_obstacle, _) in visited.iter() {
        if *extra_obstacle == initial_position {
            continue;
        }
        let mut new_visited: HashMap<(i32, i32), Vec<Direction>> = HashMap::new();
        position = initial_position;
        direction = Direction::Up;
        loop {
            let mut found_block = false;
            let directions = new_visited.entry(position).or_insert(Vec::new());
            for old_direction in directions.iter() {
                if *old_direction == direction {
                    blocks.push(extra_obstacle.clone());
                    found_block = true;
                    break;
                }
            }
            if found_block {
                break;
            }
            directions.push(direction.clone());

            let (next_position, next_direction) = match direction {
                Direction::Up => ((position.0, position.1 - 1), Direction::Right),
                Direction::Right => ((position.0 + 1, position.1), Direction::Down),
                Direction::Down => ((position.0, position.1 + 1), Direction::Left),
                Direction::Left => ((position.0 - 1, position.1), Direction::Up),
            };

            if next_position.0 < 0
                || next_position.0 >= map[0].len() as i32
                || next_position.1 < 0
                || next_position.1 >= map.len() as i32
            {
                break;
            }

            if map[next_position.1 as usize][next_position.0 as usize] ||
                next_position.0 == extra_obstacle.0 && next_position.1 == extra_obstacle.1 {
                direction = next_direction;
            } else {
                position = next_position;
            }
        }
    }

    println!("{:?}", blocks.len());
}
