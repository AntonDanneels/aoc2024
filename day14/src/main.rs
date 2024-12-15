use std::str::FromStr;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
struct Vec {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Robot {
    pos: Vec,
    vel: Vec
}

fn write_pgm(name: &str, width: i32, height: i32, positions: &HashSet<(i32, i32)>) -> std::io::Result<()> {
    let mut file = File::create(format!("iteration_{}.pgm", name))?;

    writeln!(file, "P2")?;
    writeln!(file, "{} {}", width, height)?;
    writeln!(file, "1")?;

    for y in 0..height {
        for x in 0..width {
            if positions.contains(&(x, y)) {
                write!(file, "1 ")?;
            } else {
                write!(file, "0 ")?;
            }
        }
        writeln!(file)?;
    }

    Ok(())
}

fn main() {
    let robots = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let mut p1 = parts.next().unwrap().split("=")
                          .nth(1).unwrap().split(",");
            let x = i32::from_str(p1.next().unwrap()).unwrap();
            let y = i32::from_str(p1.next().unwrap()).unwrap();

            let mut p2 = parts.next().unwrap().split("=")
                          .nth(1).unwrap().split(",");
            let vx = i32::from_str(p2.next().unwrap()).unwrap();
            let vy = i32::from_str(p2.next().unwrap()).unwrap();

            Robot {
                pos: Vec { x, y },
                vel: Vec { x: vx, y: vy },
            }
        })
        .collect::<std::vec::Vec<Robot>>();


    // Part 1
    //println!("{:?}", robots);

    let w: i32 = 101;
    let h: i32 = 103;
    let hw: i32 = w / 2;
    let hh: i32 = h / 2;
    let seconds = 100;
    /*
    let end_pos = robots.iter().map(|robot| {
        let x = robot.pos.x + (seconds * robot.vel.x);
        let y = robot.pos.y + (seconds * robot.vel.y);

        (x.rem_euclid(w), y.rem_euclid(h))
    })
    .collect::<std::vec::Vec<(i32, i32)>>();

    let mut quadrants = [0, 0, 0, 0].to_vec();
    println!("{} {}", hw, hh);
    end_pos.iter().for_each(|(x, y)|{
        if *x < hw && *y < hh {
            quadrants[0] += 1;
        } else if *x > hw && *y < hh {
            quadrants[1] += 1;
        } else if *x < hw && *y > hh {
            quadrants[2] += 1;
        } else if *x > hw && *y > hh {
            quadrants[3] += 1;
        }

    });

    println!("{:?} {:?}", end_pos, quadrants);

    let total = quadrants.iter().fold(1, |product, val| { product * val });

    println!("{}", total);
    */

    //magic numbers: looked at the first 100 iterations,
    //noticed a pattern at 14 and 64 horizontal/vertical
    //then looked at repeats every width/height
    let mut i: i32 = 64;
    while i < w * h {
        let mut positions = HashSet::new();

        for robot in robots.iter() {
            let x = robot.pos.x + (i * robot.vel.x);
            let y = robot.pos.y + (i * robot.vel.y);

            positions.insert((x.rem_euclid(w), y.rem_euclid(h)));
        }

        write_pgm(format!("{}", i + 1).as_str(), w, h, &positions).expect("Failed to write image");

        i += h;
    }
}
