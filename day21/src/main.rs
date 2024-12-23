use std::collections::{HashSet, HashMap, VecDeque};
use std::str::FromStr;

#[derive(Debug)]
struct KeyPad {
    current_pos: (usize, usize),
    grid: Vec<Vec<Option<char>>>,

    memory: HashMap<(char, (usize, usize)), (usize, (usize, usize))>,

    parent: Option<Box<KeyPad>>
}

impl KeyPad {
    fn find_path(&mut self, code: char) -> Vec<char> {
        let mut result = Vec::new();

        let (mut px, mut py) = self.current_pos;

        let mut tx = px;
        let mut ty = py;
        for (y,row) in self.grid.iter().enumerate() {
            for (x,c) in row.iter().enumerate() {
                if Some(code) == *c {
                    tx = x;
                    ty = y;
                }
            }
        }

        let dx: i32 = if tx > px { 1 } else { -1 };
        let dy: i32 = if ty > py { 1 } else { -1 };

        let mx = if dx < 0 { '<' } else { '>' };
        let my = if dy < 0 { '^' } else { 'v' };

        let mut cx = px;
        let mut move_horizontal = true;
        while cx != tx {
            let nx = (cx as i32 + dx) as usize;
            if self.grid[py][nx].is_none() {
                move_horizontal = false;
                break;
            }
            cx = nx;
        }

        if move_horizontal {
            while px != tx {
                let nx = (px as i32 + dx) as usize;
                px = nx;
                result.push(mx);
            }
            while py != ty {
                let ny = (py as i32 + dy) as usize;
                py = ny;
                result.push(my);
            }
        } else {
            while py != ty {
                let ny = (py as i32 + dy) as usize;
                py = ny;
                result.push(my);
            }
            while px != tx {
                let nx = (px as i32 + dx) as usize;
                px = nx;
                result.push(mx);
            }
        }
        self.current_pos = (tx, ty);

        result.push('A');

        result
    }

    fn enter_digit(&mut self, code: char, depth: usize) -> usize {

        for _ in 0..depth {
            print!("\t");
        }
        println!("{}", code);

        if let Some((x, (px, py))) = self.memory.get(&(code, self.current_pos)) {
            self.current_pos = (*px, *py);

            for _ in 0..depth {
                print!("\t");
            }
            println!("memory: {:?}", x);

            return *x;
        }

        let start_pos = self.current_pos;
        let path = self.find_path(code);

        for _ in 0..depth {
            print!("\t");
        }
        println!("{:?}", path);

        if let Some(ref mut parent) = self.parent {
            let r = path.iter().map(|a| {
                let p = parent.enter_digit(*a, depth + 1);
                p
            }).sum();
            self.memory.insert((code, start_pos),
                          (r, self.current_pos));

            for _ in 0..depth {
                print!("\t");
            }
            println!("P: {}", r);

            return r;
        }

        self.memory.insert((code, start_pos),
                      (path.len(), self.current_pos));

        for _ in 0..depth {
            print!("\t");
        }
        println!("R: {}", path.len());

        path.len()
    }
}

fn main() {
    let mut previous_keypad = None;
    for _ in 0..25 {
        let keypad = KeyPad {
            current_pos: (2, 0),
            grid: [
                [None,      Some('^'), Some('A')].to_vec(),
                [Some('<'), Some('v'), Some('>')].to_vec(),
            ].to_vec(),
            parent: previous_keypad,
            memory: HashMap::new(),
        };

        previous_keypad = Some(Box::new(keypad));
    }

    let mut keypad1 = KeyPad {
        current_pos: (2, 3),
        grid: [
            [Some('7'), Some('8'), Some('9')].to_vec(),
            [Some('4'), Some('5'), Some('6')].to_vec(),
            [Some('1'), Some('2'), Some('3')].to_vec(),
            [None,      Some('0'), Some('A')].to_vec(),
        ].to_vec(),
        //parent: Some(Box::new(previous_keypad)),
        parent: previous_keypad,
        //parent: None,
        memory: HashMap::new(),
    };

    let entries = [
        "029A",
        "980A",
        "179A",
        "456A",
        "379A",
    ];

    let entries = [
        "964A",
        "140A",
        "413A",
        "670A",
        "593A",
    ];

    let mut sum = 0;
    for entry in entries.iter() {
        //let mut result = Vec::new();
        let mut result = 0;
        for d in entry.chars() {
            let mut r = keypad1.enter_digit(d, 0);
            //println!("{:?} {}", d,
            //     r.iter().collect::<String>());
            //result.append(&mut r);
            //result.push(' ');
            result += r;
            println!("{} -> {:?}", d, r);
        }
        println!("{} -> {:?}", entry, result);

        let multiplier =
            usize::from_str(&entry[0..entry.len() - 1])
            .unwrap();

        /*
        println!("{:?} {} x {}", result.len(),
                 result.iter().collect::<String>(),
                 multiplier);
        */

        sum += multiplier * result;
    }

    println!("{}", sum);
}
