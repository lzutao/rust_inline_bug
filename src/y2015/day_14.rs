
#[derive(Copy, Clone)]
struct Sob<'a>{
    pub name : &'a str,
    pub speed : usize,
    pub fly_time : usize,
    pub rest_time : usize,
}

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2015/14");
    let sobs : Vec<Sob> = INPUT
        .lines()
        .map(|l| {
            let p : Vec<&str> = l.split(" ").collect();
            Sob{
                name:p[0],
                speed: p[3].parse::<usize>().unwrap(),
                fly_time: p[6].parse::<usize>().unwrap(),
                rest_time: p[13].parse::<usize>().unwrap()
            }
        })
        .collect();

    const RACE_TIME : usize = 2503;
    let distance = sobs
        .iter()
        .map(|s| fly(s, RACE_TIME))
        .max().unwrap();

    println!("[PART 1] Winners distance {}", distance);
    let mut sobs : Vec<SobWithState> = sobs
        .iter()
        .map(|s| SobWithState::new(*s, State::Flying(0), 0, 0)).collect();
    for _ in 0..RACE_TIME {
        for sob in &mut sobs {
            fly_step(sob)
        }
        let mut max = usize::MIN;
        for sob in &mut sobs {
            if sob.pos > max {
                max = sob.pos;
            }
        }
        for sob in &mut sobs {
            if sob.pos == max {
                sob.points += 1;
            }
        }
    }

    let points = sobs
        .iter()
        .map(|s| s.points)
        .max().unwrap();
    println!("[PART 2] Winners points {}", points);
}

struct SobWithState<'a>{
    pub sob : Sob<'a>,
    pub state: State,
    pub pos: usize,
    pub points: usize,
}
impl<'a> SobWithState<'a>{
    pub fn new(sob : Sob<'a>, state: State, pos: usize, points: usize) -> SobWithState<'a>{
        SobWithState{ sob, state, pos, points }
    }
}

enum State{
    Flying(usize),
    Resting(usize),
}

fn fly(sob: &Sob, time: usize) -> usize{
    let mut sws = SobWithState::new(*sob, State::Flying(0), 0, 0);
    for _ in 0..time {
        fly_step(&mut sws);
    }
    sws.pos
}

fn fly_step(sob: &mut SobWithState){
    sob.state = match sob.state{
        State::Flying(t) => {
            sob.pos += sob.sob.speed;
            if t + 1 < sob.sob.fly_time {
                State::Flying(t+1)
            }else {
                State::Resting(0)
            }
        }
        State::Resting(t) => {
            if t + 1 < sob.sob.rest_time {
                State::Resting(t+1)
            }else {
                State::Flying(0)
            }
        }
    };
}