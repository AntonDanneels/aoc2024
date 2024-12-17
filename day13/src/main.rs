use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
struct Fraction {
    num: i128,
    denom: i128,
}

impl Fraction {
    fn simplify(&mut self) {
        let mut a = self.num;
        let mut b = self.denom;
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }

        //println!("{} {} / {}", self.num, self.denom, a);

        self.num /= a;
        self.denom /= a;
    }

    fn from(num: i128) -> Fraction {
        Fraction {
            num: num,
            denom: 1,
        }
    }

    fn new(num: i128, denom: i128) -> Fraction {
        Fraction {
            num,
            denom
        }
    }
}

impl std::ops::Mul for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: Fraction) -> Fraction { 
        //println!("{:?} * {:?}", self, rhs);
        let mut r = Fraction {
            num: self.num * rhs.num,
            denom: self.denom * rhs.denom,
        };
        r.simplify();

        r
    }
}

impl std::ops::Sub for Fraction {
    type Output = Fraction;

    fn sub(self, rhs: Fraction) -> Fraction {
        //println!("{:?} - {:?}", self, rhs);
        let mut r = Fraction {
            num: (self.num * rhs.denom) - (rhs.num * self.denom),
            denom: self.denom * rhs.denom,
        };
        r.simplify();

        r
    }
}

impl std::ops::Div for Fraction {
    type Output = Fraction;

    fn div(self, rhs: Fraction) -> Fraction {
        //println!("{:?} / {:?}", self, rhs);
        let mut r = Fraction {
            num: self.num * rhs.denom,
            denom: self.denom * rhs.num,
        };
        r.simplify();

        r
    }
}

#[derive(Debug)]
struct Equation {
    a: Fraction,
    b: Fraction,
    total: Fraction,
}

fn solve(eq_a: &Equation, eq_b: &Equation) -> Option<(Fraction, Fraction)> {

    let multiplier = Fraction::new(eq_a.a.num, 
                                   eq_b.b.num * eq_a.a.num - eq_a.b.num * eq_b.a.num);
    let mut b = (eq_b.total - eq_b.a / eq_a.a * eq_a.total) * multiplier;
    let mut a = (eq_a.total - eq_a.b * b) / eq_a.a;

    /*
    println!("{:?} {:?} | {:?} {:?}",
             eq_a, eq_b, 
             a, b,
             );
             */

    /*
    if a < Fraction::from(0) || b < Fraction::from(0) {
        return None;
    }
    */

    /*
    if (a - a.round()).abs() > 0.1 || (b - b.round()).abs() > 0.1 {
        return None;
    }
    */

    /*
    if a > 100.0 || b > 100.0 {
        return None;
    }
    */

    //println!("Old {:?} {:?}", a, b);

    a.simplify();
    b.simplify();

    //println!("New {:?} {:?}", a, b);

    if a.denom != 1 || b.denom != 1 {
        return None;
    }

    Some((a, b))
}

impl Equation {
    fn new() -> Equation {
        Equation {
            a: Fraction::from(0),
            b: Fraction::from(0),
            total: Fraction::from(0),
        }
    }
}

fn main() {

    let mut problems = Vec::new();

    let mut eq_a = Equation::new();
    let mut eq_b = Equation::new();
    include_str!("input.txt")
        .lines()
        .for_each(|line| {
            if line.contains("Button") {
                let eq = line.split(":").nth(1).unwrap()
                    .split(",")
                    .map(|part| {
                        i128::from_str(part.trim()
                                          .split("+")
                                          .nth(1)
                                          .unwrap())
                            .unwrap()
                    })
                    .collect::<Vec<i128>>();

                if line.contains("Button A") {
                    eq_a.a = Fraction::from(eq[0]);
                    eq_b.a = Fraction::from(eq[1]);
                } else if line.contains("Button B") {
                    eq_a.b = Fraction::from(eq[0]);
                    eq_b.b = Fraction::from(eq[1]);
                }
            } else if line.contains("Prize") {
                let eq = line.split(":").nth(1).unwrap()
                    .split(",")
                    .map(|part| {
                        i128::from_str(part.trim()
                                          .split("=")
                                          .nth(1)
                                          .unwrap())
                            .unwrap()
                    })
                    .collect::<Vec<i128>>();

                eq_a.total = Fraction::from(eq[0] + 10000000000000);
                eq_b.total = Fraction::from(eq[1] + 10000000000000);

                let mut neq_a = Equation::new();
                let mut neq_b = Equation::new();

                std::mem::swap(&mut neq_a, &mut eq_a);
                std::mem::swap(&mut neq_b, &mut eq_b);

                problems.push((neq_a, neq_b));

            }
        });

    let mut sum = 0;
    for (eq_a, eq_b) in problems.iter() {
        let solution = solve(eq_a, eq_b);
        if let Some((a, b)) = solution {
            sum += a.num as u64 * 3 + b.num as u64;
        }
    }

    println!("{:?}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fraction_simplifies() {
        let mut f = Fraction { num: 16, denom: 2 };
        f.simplify();

        assert_eq!(f.num, 8);
        assert_eq!(f.denom, 1);

        let mut f = Fraction { num: 3, denom: 1 };
        f.simplify();

        assert_eq!(f.num, 3);
        assert_eq!(f.denom, 1);
    }

    #[test]
    fn operator_div() {
        let mut a = Fraction { num: 16, denom: 2 };
        let mut b = Fraction { num: 3, denom: 1 };

        let r = a / b;

        assert_eq!(r.num, 8);
        assert_eq!(r.denom, 3);
    }

    #[test]
    fn operator_mul() {
        let mut a = Fraction { num: 16, denom: 2 };
        let mut b = Fraction { num: 3, denom: 1 };

        let r = a * b;

        assert_eq!(r.num, 24);
        assert_eq!(r.denom, 1);
    }

    #[test]
    fn operator_sub() {
        let mut a = Fraction { num: 16, denom: 2 };
        let mut b = Fraction { num: 4, denom: 2 };

        let r = a - b;

        assert_eq!(r.num, 6);
        assert_eq!(r.denom, 1);
    }

    #[test]
    fn operator_sequence() {
        let mut a = Fraction { num: 16, denom: 2 };
        let mut b = Fraction { num: 4, denom: 2 };
        let mut c = Fraction { num: 3, denom: 1 };

        let r = a - b / c;

        assert_eq!(r.num, 22);
        assert_eq!(r.denom, 3);
    }
}
