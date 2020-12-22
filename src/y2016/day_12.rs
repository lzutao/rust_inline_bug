use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Register{A,B, C, D}
use Register::*;
fn register(r: char) -> Register{
    match r {
        'a' => A,
        'b' => B,
        'c' => C,
        'd' => D,
        _ => unreachable!()
    }
}

#[derive(Debug, Copy, Clone)]
enum RV {R(Register), V(isize)}
fn rv(s: &str) -> RV {
    match s.parse() {
        Ok(v) => RV::V(v),
        Err(_) => RV::R(register(s.as_bytes()[0] as char))
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction{
    CPY(RV, Register),
    INC(Register),
    DEC(Register),
    JNZ(RV, isize),
}
use Instruction::*;
use std::ops::{Index, IndexMut};

impl FromStr for Instruction{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.as_bytes();
        match chars[0] as char {
            'c' => Ok(CPY(
                rv(s.split_at(s.len()-2).0.split_at(4).1),
                register(chars[chars.len()-1] as char))),
            'i' => Ok(INC(register(chars[4] as char))),
            'd' => Ok(DEC(register(chars[4] as char))),
            'j' => {
                let mut s = s.split(" ");
                let _i = s.next().unwrap();
                Ok(JNZ(rv(s.next().unwrap()), s.next().unwrap().parse().unwrap()))
            },
            _ => unreachable!("'{}' in {}", chars[2] as char, s)
        }
    }
}

#[derive(Debug)]
struct Machine{
    pub a : isize,
    pub b : isize,
    pub c : isize,
    pub d : isize,
    pub ip : usize,
    pub instructions : Vec<Instruction>,
}

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2016/12");
    let instructions : Vec<Instruction> = INPUT
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    let mut machine = Machine{ a: 0, b: 0, c: 0, d: 0, ip: 0, instructions };

    machine.execute_program();

    println!("[PART 1] Register a after program execution: {}", machine.a);

    machine.a = 0;
    machine.b = 0;
    machine.c = 1;
    machine.d = 0;
    machine.ip = 0;
    machine.execute_program();

    println!("[PART 2] Register a after program execution: {}", machine.a);

}

impl Machine{
    pub fn execute_program(&mut self){
        while self.ip < self.instructions.len() {
            self.step();
        }
    }
    pub fn step(&mut self){
        //println!("{:?}", self);
        match self.instructions[self.ip]{
            CPY(s, r) => {
                self[r] = self.rv(s); self.ip += 1;
            },
            INC(r) => { self[r] += 1; self.ip += 1;},
            DEC(r) => { self[r] -= 1; self.ip += 1;},
            JNZ(rv, o) => {
                if self.rv(rv) == 0 {
                    self.ip += 1;
                }else {
                    self.jmp(o);
                }
            },
        }
    }
    fn jmp(&mut self, o: isize){
        if o < 0 {
            self.ip = self.ip.overflowing_sub((-o) as usize).0;
        }else {
            self.ip += o as usize;
        }
    }

    fn rv(&self, rv: RV) -> isize {
        match rv {
            RV::R(r) => self[r],
            RV::V(v) => v,
        }
    }
}

impl Index<Register> for Machine{
    type Output = isize;

    fn index(&self, index: Register) -> &Self::Output {
        match index {
            A => &self.a,
            B => &self.b,
            C => &self.c,
            D => &self.d,
        }
    }
}
impl IndexMut<Register> for Machine{
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        match index {
            A => &mut self.a,
            B => &mut self.b,
            C => &mut self.c,
            D => &mut self.d,
        }
    }
}
