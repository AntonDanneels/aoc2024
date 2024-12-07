fn main() {
    let data: Vec<Vec<char>> = include_str!("sample.txt")
        .lines()
        .map(|line| {
            line.chars().collect()
        })
        .collect();

    println!("{:?}", data);

    // part 1
    let mut count = 0;
    for (i, row) in data.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'X' {
                //check row backwards and forwards
                if j as i32 - 3 >= 0 && row[j - 1] == 'M' && row[j - 2] == 'A' && row[j - 3] == 'S' {
                    count += 1;
                }
                if j as i32 + 3 < row.len() as i32 && row[j + 1] == 'M' && row[j + 2] == 'A' && row[j + 3] == 'S' {
                    count += 1;
                }
                //check column backwards and forwards
                if i as i32 - 3 >= 0 && data[i - 1][j] == 'M' && data[i - 2][j] == 'A' && data[i - 3][j] == 'S' {
                    count += 1;
                }
                if i as i32 + 3 < data.len() as i32 && data[i + 1][j] == 'M' && data[i + 2][j] == 'A' && data[i + 3][j] == 'S' {
                    count += 1;
                }
                //check diagonal backwards and forwards
                if i as i32 - 3 >= 0 && j as i32 - 3 >= 0 && data[i - 1][j - 1] == 'M' && data[i - 2][j - 2] == 'A' && data[i - 3][j - 3] == 'S' {
                    count += 1;
                }
                if i as i32 - 3 >= 0 && j as i32 + 3 < row.len() as i32 && data[i - 1][j + 1] == 'M' && data[i - 2][j + 2] == 'A' && data[i - 3][j + 3] == 'S' {
                    count += 1;
                }
                if i as i32 + 3 < data.len() as i32 && j as i32 + 3 < row.len() as i32 && data[i + 1][j + 1] == 'M' && data[i + 2][j + 2] == 'A' && data[i + 3][j + 3] == 'S' {
                    count += 1;
                }
                if i as i32 + 3 < data.len() as i32 && j as i32 - 3 >= 0 as i32 && data[i + 1][j - 1] == 'M' && data[i + 2][j - 2] == 'A' && data[i + 3][j - 3] == 'S' {
                    count += 1;
                }
            }
        }
    }
    println!("{:?}", count);

    // part 2
    let mut count = 0;
    for (i, row) in data.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'A' {
                if i as i32 - 1 >= 0 && i + 1 < data.len() && j as i32 - 1 >= 0 && j + 1 < row.len() {
                    if ((data[i - 1][j - 1] == 'M' && data[i + 1][j + 1] == 'S') || (data[i - 1][j - 1] == 'S' && data[i + 1][j + 1] == 'M')) && 
                       ((data[i + 1][j - 1] == 'M' && data[i - 1][j + 1] == 'S') || (data[i + 1][j - 1] == 'S' && data[i - 1][j + 1] == 'M')) {
                        count += 1
                    }
                }
            }
        }
    }

    println!("{:?}", count);
}
