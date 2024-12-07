use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn main() {
    let mut position = (-1, -1);
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
                        position = (x as i32, y as i32);
                    }

                    false
                })
                .collect()
        })
        .collect();

    let mut visited: HashMap<(i32, i32), bool> = HashMap::new();
    let mut previous_points = Vec::new();
    let mut loops = 0;
    let mut loop_points: HashMap<(i32, i32), bool> = HashMap::new();
    loop {
        visited.insert(position.clone(), true);

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
            previous_points.push(next_position);
        } else {

            // use all but the last three positions as we need to form a square
            let selected_points = previous_points.iter()
                                                 .enumerate()
                                                 .filter(|(i, _)| (*i as i32) < previous_points.len() as i32 - 2)
                                                 .map(|(_, e)| e);

            // if we have a direct path to a previous node from the current position, we have a
            // potential loop
            for previous_point in selected_points {
                // match next direction as we need to turn towards the point
                match next_direction {
                    Direction::Up => {
                        if previous_point.0 != position.0 || previous_point.1 >= position.1 {
                            continue;
                        }

                        let mut direct_line = true;
                        for y in (previous_point.1 + 1)..(position.1) {
                            if map[y as usize][position.0 as usize] {
                                direct_line = false;
                                break;
                            }
                        };

                        if direct_line {
                            loops += 1;
                            loop_points.insert((position.0 - 1, position.1), true);
                        }
                    },
                    Direction::Right => {
                        if previous_point.1 != position.1 || previous_point.0 <= position.0 {
                            continue;
                        }

                        let mut direct_line = true;
                        for x in (position.0)..(previous_point.0) {
                            if map[position.1 as usize][x as usize] {
                                direct_line = false;
                                break;
                            }
                        };

                        if direct_line {
                            loops += 1;
                            loop_points.insert((position.0, position.1 - 1), true);
                        }
                    },
                    Direction::Down => {
                        if previous_point.0 != position.0 || previous_point.1 <= position.1 {
                            continue;
                        }

                        let mut direct_line = true;
                        for y in (position.1)..(previous_point.1) {
                            if map[y as usize][position.0 as usize] {
                                direct_line = false;
                                break;
                            }
                        };

                        if direct_line {
                            loops += 1;
                            loop_points.insert((position.0 + 1, position.1), true);
                        }
                    },
                    Direction::Left => {
                        if previous_point.1 != position.1 || previous_point.0 >= position.0 {
                            continue;
                        }

                        let mut direct_line = true;
                        for x in (previous_point.0 + 1)..(position.0) {
                            if map[position.1 as usize][x as usize] {
                                direct_line = false;
                                break;
                            }
                        };

                        if direct_line {
                            loops += 1;
                            loop_points.insert((position.0, position.1 + 1), true);
                        }
                    },
                }
            }
            position = next_position;
        }
    }
    for (y, row) in map.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if map[y][x] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!();

    for (y, row) in map.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if loop_points.contains_key(&(x as i32, y as i32)) {
                print!("O");
            } else if map[y][x] {
                print!("#");
            } else if visited.contains_key(&(x as i32, y as i32)) {
                print!("x");
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!("{:?}", visited.len());
    println!("{:?}", loops);
}
