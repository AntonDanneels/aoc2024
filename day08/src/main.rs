use std::collections::HashMap;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn invert(&self) -> Point {
        Point {x: self.x * -1, y: self.y * -1}
    }

    fn add(&self, other: &Point) -> Point {
        Point {x: self.x + other.x, y: self.y + other.y}
    }

    fn sub(&self, other: &Point) -> Point {
        Point {x: other.x - self.x, y: other.y - self.y}
    }
}

fn is_in_bounds(point: &Point, width: u32, height: u32) -> bool {
    point.x >= 0 && point.x < width as i32 &&
    point.y >= 0 && point.y < height as i32
}

fn main() {
    let data = include_str!("input1.txt").lines();
    let mut antennas = HashMap::<char, Vec<Point>>::new();
    let mut width = 0;
    let mut height = 0;
    data
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c != '.' {
                    let points = antennas.entry(c).or_insert(Vec::new());
                    points.push(Point {x: x as i32, y: y as i32});
                }

                width = line.len();
            });

            height = y + 1;
        });

    println!("{:?} {} {}", antennas, width, height);

    let mut antipoles = HashMap::<Point, bool>::new();
    let mut map_antennas = HashMap::<Point, char>::new();

    for (antenna, positions) in antennas.iter() {
        for (i, pos) in positions.iter().enumerate() {
            map_antennas.insert(pos.clone(), *antenna);
            if i + 1 == positions.len() { continue }
            for other in positions[i + 1..].iter() {
                let delta = pos.sub(other);
                let idelta = delta.invert();

                antipoles.insert(pos.clone(), true);
                antipoles.insert(other.clone(), true);

                let mut point = pos.clone();
                loop {
                    let new_point = point.add(&idelta);

                    if !is_in_bounds(&new_point, width as u32, height as u32) {
                        break;
                    }
                    antipoles.insert(new_point.clone(), true);
                    point = new_point;
                }

                point = pos.clone();
                loop {
                    let new_point = point.add(&delta);

                    if !is_in_bounds(&new_point, width as u32, height as u32) {
                        break;
                    }
                    antipoles.insert(new_point.clone(), true);
                    point = new_point;
                }
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            if antipoles.contains_key(&Point {x: x as i32, y: y as i32}) {
                print!("#");
            } else if let Some(x) = map_antennas.get(&Point {x: x as i32, y: y as i32}) {
                print!("{}", x);
            } else {
                print!(".");
            }
        }
        println!();
    }

    let antipoles_in_bound: u32 = antipoles.iter().filter(|(antipole, _)| {
        antipole.x >= 0 && antipole.x < width as i32 &&
            antipole.y >= 0 && antipole.y < height as i32
    }).map(|_| 1).sum();

    println!("{:?}", antipoles_in_bound);
}
