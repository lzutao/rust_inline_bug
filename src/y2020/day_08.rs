use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Instruction{
    NOP(isize),
    ACC(isize),
    JMP(isize),
}
use Instruction::*;

impl FromStr for Instruction{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.as_bytes();
        match chars[0] as char {
            'a' => Ok(ACC(s.split_at(4).1.parse().unwrap())),
            'j' => Ok(JMP(s.split_at(4).1.parse().unwrap())),
            'n' => Ok(NOP(s.split_at(4).1.parse().unwrap())),
            _ => unreachable!("'{}' in {}", chars[0] as char, s)
        }
    }
}

#[derive(Debug)]
struct Machine{
    pub acc : isize,
    pub ip : usize,
    pub instructions : Vec<(Instruction, bool)>,
}

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2020/8");
    let instructions : Vec<(Instruction, bool)> = INPUT
        .lines()
        .map(|l| (l.parse().unwrap(), false))
        .collect();

    let mut machine = Machine{ acc: 0, ip: 0, instructions: instructions.clone() };

    machine.execute_program();

    println!("[PART 1] Accumulator after program execution: {}", machine.acc);

    for i in 0..instructions.len() {
        let new= match instructions[i].0 {
            ACC(_) => continue,
            NOP(v) => JMP(v),
            JMP(v) => NOP(v),
        };

        let mut instructions = instructions.clone();
        instructions[i] = (new, false);
        let mut machine = Machine{ acc: 0, ip: 0, instructions: instructions.clone() };
        machine.execute_program();
        let expected_ip = match (*instructions.last().unwrap()).0 {
            NOP(_) | ACC(_) => instructions.len() as isize,
            JMP(v) => instructions.len() as isize - 1 + v,
        } as usize;
        if machine.ip != expected_ip {
            continue
        }

        println!("[PART 2] Accumulator after program execution: {}", machine.acc);
        return;
    }

    println!("not found solution to PART 2")

}

impl Machine{
    pub fn execute_program(&mut self){
        while self.step() { }
    }
    pub fn step(&mut self) -> bool{
        let (i, already_executed) = &mut self.instructions[self.ip];
        if *already_executed {
            return false;
        }
        *already_executed = true;
        match *i {
            ACC(v) => { self.acc += v; self.ip += 1;},
            NOP(_) => self.ip += 1,
            JMP(o) => self.jmp(o),
        }
        self.ip < self.instructions.len()
    }
    fn jmp(&mut self, o: isize){
        if o < 0 {
            self.ip = self.ip.overflowing_sub((-o) as usize).0;
        }else {
            self.ip += o as usize;
        }
    }
}
