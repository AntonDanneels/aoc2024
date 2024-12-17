
#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq)]
enum Tile {
    Wall,
    Box,
    Air,
}

fn print_map(map: &mut Vec<Vec<Tile>>, robot_pos: &mut (i32, i32)) {
    let mut score = 0;
    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, m)| {
            if x == robot_pos.0 as usize && y == robot_pos.1 as usize {
                print!("@");
                return;
            }
            match m {
                Tile::Air => print!("."),
                Tile::Box => {
                    score += y * 100 + x;
                    print!("O")
                },
                Tile::Wall => print!("#"),
            }
        });
        println!();
    });
    println!("Score: {}", score);
}

fn main() {
    let mut map = Vec::new();
    let mut moves = Vec::new();
    let mut robot_pos: (i32, i32) = (0,0);
    include_str!("input.txt")
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            if line.contains("#") {
                let row = line.chars()
                    .enumerate().map(|(x, c)| {
                    match c {
                        '#' => Tile::Wall,
                        'O' => Tile::Box,
                        '.' => Tile::Air,
                        '@' => {
                            robot_pos = (x as i32, y as i32);
                            Tile::Air
                        },
                        _ => panic!(""),
                    }
                }).collect::<Vec<Tile>>();
                map.push(row);
            } else {
                line.chars().for_each(|c| {
                    let dir = match c {
                        '>' => Direction::Right,
                        '<' => Direction::Left,
                        '^' => Direction::Up,
                        'v' => Direction::Down,
                        _ => panic!(""),
                    };

                    moves.push(dir);
                });
            }
        });

    println!("{:?}", map);
    println!("{:?}", moves);

    let width = map[0].len() as i32;
    let height = map.len() as i32;

    print_map(&mut map, &mut robot_pos);
    moves.iter().for_each(|m| {
        let (next_pos, ex, ey) = match m {
            Direction::Up => {
                ((robot_pos.0, robot_pos.1 - 1), robot_pos.0, 0)
            },
            Direction::Right => {
                ((robot_pos.0 + 1, robot_pos.1), width - 1, robot_pos.1)
            },
            Direction::Left => {
                ((robot_pos.0 - 1, robot_pos.1), 0, robot_pos.1)
            },
            Direction::Down => {
                ((robot_pos.0, robot_pos.1 + 1), robot_pos.0, height - 1)
            },
        };

        if next_pos.0 < 0 || next_pos.0 >= width &&
           next_pos.1 < 0 || next_pos.1 >= height {
            return;
        }

        match map[next_pos.1 as usize][next_pos.0 as usize] {
            Tile::Air => {
                robot_pos = next_pos;
            }
            Tile::Wall => {}
            Tile::Box => {
                let rx  = robot_pos.0 as usize;
                let ry  = robot_pos.1 as usize;

                match m {
                    Direction::Up => {
                        let mut y = robot_pos.1 - 2;
                        while y >= 0 {
                            if map[y as usize][rx] == Tile::Wall {
                                break;
                            }
                            if map[y as usize][rx] == Tile::Air {
                                map[y as usize][rx] = Tile::Box;
                                map[next_pos.1 as usize][next_pos.0 as usize] = Tile::Air;
                                robot_pos = next_pos;
                                break;
                            }
                            y -= 1;
                        }
                    },
                    Direction::Down => {
                        let mut y = robot_pos.1 + 2;
                        while y < height {
                            if map[y as usize][rx] == Tile::Wall {
                                break;
                            }
                            if map[y as usize][rx] == Tile::Air {
                                map[y as usize][rx] = Tile::Box;
                                map[next_pos.1 as usize][next_pos.0 as usize] = Tile::Air;
                                robot_pos = next_pos;
                                break;
                            }
                            y += 1;
                        }
                    },
                    Direction::Left => {
                        let mut x = robot_pos.0 - 2;
                        while x >= 0 {
                            if map[ry][x as usize] == Tile::Wall {
                                break;
                            }
                            if map[ry][x as usize] == Tile::Air {
                                map[ry][x as usize] = Tile::Box;
                                map[next_pos.1 as usize][next_pos.0 as usize] = Tile::Air;
                                robot_pos = next_pos;
                                break;
                            }
                            x -= 1;
                        }
                    },
                    Direction::Right => {
                        let mut x = robot_pos.0 + 2;
                        while x < width {
                            if map[ry][x as usize] == Tile::Wall {
                                break;
                            }
                            if map[ry][x as usize] == Tile::Air {
                                map[ry][x as usize] = Tile::Box;
                                map[next_pos.1 as usize][next_pos.0 as usize] = Tile::Air;
                                robot_pos = next_pos;
                                break;
                            }
                            x += 1;
                        }
                    },
                    _ => {}
                }

            }
        };

        print_map(&mut map, &mut robot_pos);
    });
}
