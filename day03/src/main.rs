#[derive(Debug, PartialEq)]
enum Token {
    Mul,
    Do,
    Dont,
    LeftParen,
    Number(u32),
    RightParen,
    Comma,
    Invalid,
}

fn main() {
    let data = include_str!("input1.txt");

    let mut iter = data.chars().into_iter().peekable();
    let mut tokens = Vec::new();

    while iter.peek().is_some() {
        let c = iter.next().unwrap();

        match c {
            'm' => {
                if iter.peek() != Some(&'u') {
                    continue;
                }
                iter.next();

                if iter.peek() != Some(&'l') {
                    continue;
                }
                iter.next();

                tokens.push(Token::Mul);
            }
            'd' => {
                if iter.peek() != Some(&'o') {
                    continue;
                }
                iter.next();

                if iter.peek() == Some(&'n') {
                    iter.next();

                    if iter.peek() != Some(&'\'') {
                        continue;
                    }
                    iter.next();

                    if iter.peek() != Some(&'t') {
                        continue;
                    }
                    iter.next();
                    tokens.push(Token::Dont);
                } else {
                    tokens.push(Token::Do);
                }
            }
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            ',' => tokens.push(Token::Comma),
            '0'..='9' => {
                let mut numbers = Vec::new();
                numbers.push(c.to_digit(10).unwrap());
                while let Some(n) = iter.peek() {
                    match n {
                        '0'..='9' => {
                            let x = iter.next().unwrap();
                            numbers.push(x.to_digit(10).unwrap());
                        }
                        _ => break,
                    }
                }
                let mut number = 0;
                for (i, n) in numbers.iter().rev().enumerate() {
                    number += *n * 10_u32.pow(i as u32);
                }
                tokens.push(Token::Number(number));
            }
            _ => {
                tokens.push(Token::Invalid);
            }
        }
    }

    std::println!("{:?}", tokens);

    let mut total = 0;
    let mut iter = tokens.iter().peekable();
    let mut do_multiply = true;
    while iter.peek().is_some() {
        let token = iter.next().unwrap();

        match token {
            Token::Mul => {
                let left;
                let right;
                if iter.peek() != Some(&&Token::LeftParen) {
                    continue;
                }
                iter.next();
                match iter.next() {
                    Some(Token::Number(x)) => {
                        left = *x;
                    }
                    _ => continue,
                }
                if iter.peek() != Some(&&Token::Comma) {
                    continue;
                }
                iter.next();
                match iter.next() {
                    Some(Token::Number(x)) => {
                        right = *x;
                    }
                    _ => continue,
                }
                if iter.peek() != Some(&&Token::RightParen) {
                    continue;
                }
                iter.next();

                if do_multiply {
                    total += left * right;
                }
            },
            Token::Do => do_multiply = true,
            Token::Dont => do_multiply = false,
            _ => {}
        }
    }

    println!("{:?}", total);
}
