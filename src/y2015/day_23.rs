use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Register{A,B}
use Register::*;
fn register(r: char) -> Register{
    if r == 'a' {
        A
    }else {
        B
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction{
    HALF(Register),
    TRIPLE(Register),
    INC(Register),
    JMP(isize),
    JIE(Register, isize),
    JIO(Register, isize),
}
use Instruction::*;
use std::ops::{Index, IndexMut};

impl FromStr for Instruction{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.as_bytes();
        match chars[2] as char {
            'f' => Ok(HALF(register(chars[4] as char))),
            'l' => Ok(TRIPLE(register(chars[4] as char))),
            'c' => Ok(INC(register(chars[4] as char))),
            'p' => Ok(JMP(s.split_at(4).1.parse().unwrap())),
            'e' => Ok(JIE(register(chars[4] as char), s.split_at(7).1.parse().unwrap())),
            'o' => Ok(JIO(register(chars[4] as char), s.split_at(7).1.parse().unwrap())),
            _ => unreachable!("'{}' in {}", chars[2] as char, s)
        }
    }
}

#[derive(Debug)]
struct Machine{
    pub a : isize,
    pub b : isize,
    pub ip : usize,
    pub instructions : Vec<Instruction>,
}

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2015/23");
    let instructions : Vec<Instruction> = INPUT
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    let mut machine = Machine{ a: 0, b: 0, ip: 0, instructions };

    machine.execute_program();

    println!("[PART 1] Register b after program execution: {}", machine.b);

    machine.a = 1;
    machine.b = 0;
    machine.ip = 0;
    machine.execute_program();

    println!("[PART 2] Register b after program execution: {}", machine.b);

}

impl Machine{
    pub fn execute_program(&mut self){
        while self.ip < self.instructions.len() {
            self.step();
        }
    }
    pub fn step(&mut self){
        match self.instructions[self.ip]{
            HALF(r) => { self[r] /= 2; self.ip += 1;},
            TRIPLE(r) => { self[r] *= 3; self.ip += 1;},
            INC(r) => { self[r] += 1; self.ip += 1;},
            JMP(o) => self.jmp(o),
            JIE(r, o) => if self[r] % 2 == 0 { self.jmp(o) } else { self.ip += 1 },
            JIO(r, o) => if self[r] == 1 { self.jmp(o) } else { self.ip += 1 },
        }
    }
    fn jmp(&mut self, o: isize){
        if o < 0 {
            self.ip = self.ip.overflowing_sub((-o) as usize).0;
        }else {
            self.ip += o as usize;
        }
    }
}

impl Index<Register> for Machine{
    type Output = isize;

    fn index(&self, index: Register) -> &Self::Output {
        match index {
            A => &self.a,
            B => &self.b,
        }
    }
}
impl IndexMut<Register> for Machine{
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        match index {
            A => &mut self.a,
            B => &mut self.b,
        }
    }
}
