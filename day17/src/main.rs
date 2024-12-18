use std::collections::HashMap;

fn run_program(program: &Vec<u64>, reg_a: u64, reg_b: u64, reg_c: u64) -> Vec<u64> {
    let mut output = Vec::new();

    let mut reg_a: u64 = reg_a;
    let mut reg_b: u64 = reg_b;
    let mut reg_c: u64 = reg_c;

    //println!("{:?}", program);

    let mut i = 0;
    while i < program.len() {
        //println!("CPU: a:{} b:{} c:{} i:{} | {} {}", reg_a, reg_b, reg_c, i, program[i], program[i + 1]);

        let combo: u64 = match program[i + 1] {
            0..=3 => {
                program[i + 1]
            },
            4 => reg_a,
            5 => reg_b,
            6 => reg_c,
            _ => panic!()
        };

        let literal: u64 = program[i + 1];

        match program[i] {
            // adv
            0 => {
                //println!("ADV: reg_a >> {}", combo);
                reg_a >>= combo;
            },
            // bxl
            1 => {
                //println!("BXL: reg_b ^ {}", literal);
                reg_b ^= literal;
            },
            // bst
            2 => {
                //println!("BST: reg_b = reg_a % 8 {}", literal);
                //println!("CPU: a:{} b:{} c:{} i:{} | {} {}", reg_a, reg_b, reg_c, i, program[i], program[i + 1]);
                reg_b = combo & 0b111;
            },
            // jnz
            3 => {
                if reg_a != 0 {
                    //println!("JMP");
                    i = literal as usize;
                    continue;
                }
            },
            // bxc
            4 => {
                //println!("BXC: reg_b ^ reg_c");
                reg_b = reg_b ^ reg_c;
            },
            // out
            5 => {
                //println!("OUT: {}, ", combo % 8);
                output.push(combo & 0b111);
            },
            // bdv
            6 => {
                //println!("BDV: reg_b >> {}", combo);
                reg_b = reg_a >> combo;
            },
            // cdv
            7 => {
                //println!("CDV: reg_c >> {}", combo);
                reg_c = reg_a >> combo;
            }
            _ => {}
        }

        i += 2;
    }

    output
}

fn find_solution(program: &Vec<u64>, index: i32, assembled_a: u64) -> Option<u64> {
    if index < 0 {
        return Some(assembled_a);
    }

    let digit = program[index as usize];
    println!("Testing for: {}", digit);

    for i in 0..0b1000 {
        let attempt_reg_a = (assembled_a << 3_u64) | i;
        println!("Attempt with: {:#032b} from {:#032b}", attempt_reg_a, assembled_a);
        let output = run_program(&program.to_vec(), attempt_reg_a, 0, 0);

        println!("{:?}", output);

        if output[0] == digit {
            println!("Found: {} for {}", i, digit);
            if let Some(x) = find_solution(program, index - 1, attempt_reg_a) {
                return Some(x)
            }
        }
    }

    None
}

fn main() {
    //let mut reg_a: i64 = 729;
    let mut reg_a: u64 = 22571680;
    let mut reg_b: u64 = 0;
    let mut reg_c: u64 = 0;

    //let program = [0, 1, 5, 4, 3, 0];
    let program = [2,4, 1,3, 7,5, 0,3, 4,3, 1,5, 5,5, 3,0];
    //let program = [4, 0];

    let output = run_program(&program.to_vec(), reg_a, reg_b, reg_c);
    println!("{:?}", output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));

    let result = find_solution(&program.to_vec(), (program.len() - 1) as i32, 0);

    println!("{:?}", result);
}
