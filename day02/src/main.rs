fn check_validity(row: &Vec<i32>) -> bool {
    let mut sign = None;
    let good_cases: Vec<i32> = row.
        windows(2)
        .map(|window| window[1] - window[0])
        .filter(|delta| {
            let mut result = delta.abs() > 0 && delta.abs() <= 3;

            if let Some(x) = sign {
                result &= delta.signum() == x;
            } else {
                sign = Some(delta.signum());
            }

            result
        })
        .collect();

    good_cases.len() == row.len() - 1
}

fn main() {
    let result = include_str!("input1.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|part| part.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let tmp = result.iter()
        .filter(|row| {
            if check_validity(row) {
                return true;
            }

            for i in 0..row.len() {
                let tmp = row.iter()
                    .enumerate()
                    .filter(|(idx, _)| *idx != i)
                    .map(|(_, e)| *e)
                    .collect();
                if check_validity(&tmp) {
                    return true;
                }
            }

            false
        })
        .collect::<Vec<&Vec<i32>>>()
        .len();

    std::println!("{:?}", tmp);
}
