use std::str::FromStr;

enum Instruction {
    Rect(usize, usize),
    Row(usize, usize),
    Col(usize, usize),
}
use Instruction::*;

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(" ");
        let i = s.next().unwrap();
        let v = s.next().unwrap();
        Ok(if i == "rect" {
            let mut s = v.split("x");
            let a = s.next().unwrap().parse().unwrap();
            let b = s.next().unwrap().parse().unwrap();
            Rect(a, b)
        } else {
            let a = s.next().unwrap().split_at(2).1.parse().unwrap();
            s.next().unwrap(); // by
            let b = s.next().unwrap().parse().unwrap();
            if v == "row" {
                Row(a, b)
            } else {
                Col(a, b)
            }
        })
    }
}

const SIZE_X : usize = 50;
const SIZE_Y : usize = 6;

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2016/8");

    let instructions : Vec<Instruction> = INPUT
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    let mut display = [[false;SIZE_Y];SIZE_X];

    for i in &instructions {
        match i {
            Rect(a, b) => {
                for x in 0..*a {
                    for y in 0..*b {
                        display[x][y] = true
                    }
                }
            }
            Row(a, b) => {
                for _ in 0..*b {
                    let carry = display[SIZE_X - 1][*a];
                    for x in (1..SIZE_X).rev() {
                        display[x][*a] = display[x - 1][*a];
                    }
                    display[0][*a] = carry;
                }
            }
            Col(a, b) => {
                let col = &mut display[*a];
                for _ in 0..*b {
                    let carry = col[SIZE_Y-1];
                    for y in (1..SIZE_Y).rev() {
                        col[y] = col[y - 1];
                    }
                    col[0] = carry;
                }
            }
        }
    }

    let lit : usize = display
        .iter()
        .map(|c| c.iter().filter(|p| **p).count())
        .sum();


    println!("[PART 1] Lit pixels: {}", lit);

    println!("[PART 2]");
    for y in 0..SIZE_Y {
        for x in 0..SIZE_X {
            if display[x][y] {
                print!("#")
            } else {
                print!(" ")
            }
        }
        println!()
    }
}
