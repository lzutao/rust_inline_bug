use std::str::FromStr;

#[derive(Debug)]
enum Op { Off, On, Toggle }
struct Action{
    pub operation: Op,
    pub start_position: (usize, usize),
    pub end_position: (usize, usize),
}
impl FromStr for Action{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.bytes().nth(6).unwrap() as char == 'n' {
            let (_, s) = s.split_at(8);
            Ok(parse_action(Op::On, s))
        }else if s.bytes().nth(6).unwrap() as char == 'f' {
            let (_, s) = s.split_at(9);
            Ok(parse_action(Op::Off, s))
        }else {
            let (_, s) = s.split_at(7);
            Ok(parse_action(Op::Toggle, s))
        }
    }
}
fn parse_action(operation : Op, s : &str) -> Action{
    let (start, end) = s.split_at(s.find(' ').unwrap());
    let (_, end) = end.split_at(9);
    let start : Vec<usize> = start
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    let end : Vec<usize> = end
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    Action{
        operation,
        start_position: (start[0], start[1]),
        end_position: (end[0], end[1]),
    }
}


const SIZE : usize = 1000;

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2015/6");
    let actions : Vec<Action> = INPUT
        .lines()
        .map(|d| d.parse::<Action>().unwrap())
        .collect();
    // separated to two functions to avoid having too big stack
    on_of_lights(&actions);
    brightness_lights(actions);
}

fn brightness_lights(actions: Vec<Action>) {
    let mut brightness = [0u8; SIZE * SIZE];
    for a in actions.iter() {
        for x in a.start_position.0..=a.end_position.0 {
            for y in a.start_position.1..=a.end_position.1 {
                match a.operation {
                    Op::On => brightness[x + y * SIZE] += 1,
                    Op::Off => brightness[x + y * SIZE] = brightness[x + y * SIZE].saturating_sub(1),
                    Op::Toggle => brightness[x + y * SIZE] += 2,
                }
            }
        }
    }
    let total_brightness: usize = brightness.iter().map(|b| *b as usize).sum();
    println!("[PART 2] Total brightness {}", total_brightness);
}

fn on_of_lights(actions: &Vec<Action>) {
    {
        let mut lights = [false; SIZE * SIZE];
        for a in actions.iter() {
            for x in a.start_position.0..=a.end_position.0 {
                for y in a.start_position.1..=a.end_position.1 {
                    match a.operation {
                        Op::On => lights[x + y * SIZE] = true,
                        Op::Off => lights[x + y * SIZE] = false,
                        Op::Toggle => lights[x + y * SIZE] = !lights[x + y * SIZE],
                    }
                }
            }
        }
        let light_on = lights.iter().filter(|l| **l).count();
        println!("[PART 1] Lights On {}", light_on);
    }
}