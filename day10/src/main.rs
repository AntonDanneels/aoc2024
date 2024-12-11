use std::collections::HashSet;

fn find_paths(grid: &Vec<Vec<u32>>, current_position: &(usize, usize), visited: &mut HashSet<(usize, usize)>, sum_trailheads: &mut usize) {
    let (x, y) = current_position;
    let current_value = grid[*y][*x];

    visited.insert((*x, *y));

    if current_value == 9 {
        println!("Found path");
        *sum_trailheads += 1;
        return;
    }

    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let nx = *x as i32 + dx; 
        let ny = *y as i32 + dy; 

        if nx < 0 || ny < 0 || nx >= grid[0].len() as i32 || ny >= grid.len() as i32 {
            continue;
        }

        /*
        if visited.contains(&(nx as usize, ny as usize)) {
            continue;
        }
        */

        let neighbour = grid[ny as usize][nx as usize];

        if neighbour == current_value + 1 {
            find_paths(grid, &(nx as usize, ny as usize), visited, sum_trailheads);
        }
    }
}


fn main() {
    let mut starting_points = Vec::new();
    let grid = include_str!("input.txt")
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let d = c.to_digit(10).unwrap();
                    if d == 0 {
                        starting_points.push((x, y));
                    }
                    d
                })
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();

    println!("{:?}", starting_points);
    println!("{:?}", grid);

    let mut sum_trailheads: usize = 0;
    for starting_point in starting_points.iter() {
        let mut visited = HashSet::new();
        find_paths(&grid, starting_point, &mut visited, &mut sum_trailheads);
    }

    println!("{:?}", sum_trailheads);
}
