
const SIZE : usize = 100;
const ISIZE : isize = SIZE as isize;

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2015/18");

    let mut light = Vec::with_capacity(SIZE);
    for states in INPUT.lines() {
        let mut row = Vec::with_capacity(SIZE);
        for state in states.chars() {
            match state {
                '#' => row.push(true),
                '.' => row.push(false),
                _ => unreachable!(),
            }
        }
        light.push(row)
    }

    let mut normal_lights = light.clone();
    for _ in 0..100 {
        normal_lights = step(&normal_lights);
    }

    let light_on = normal_lights
        .iter()
        .map(|r|r.iter().filter(|a|**a).count())
        .sum::<usize>();

    println!("[PART 1] Lights On {} after 100 steps", light_on);

    set_corners_on(&mut light);
    for _ in 0..100 {
        light = step(&light);
        set_corners_on(&mut light);
    }

    let light_on = light
        .iter()
        .map(|r|r.iter().filter(|a|**a).count())
        .sum::<usize>();

    println!("[PART 2] Lights On {} after 100 steps", light_on);
}

fn set_corners_on(state : &mut Vec<Vec<bool>>){
    state[0][0] = true;
    state[0][SIZE-1] = true;
    state[SIZE-1][0] = true;
    state[SIZE-1][SIZE-1] = true;
}

fn step(state : &[Vec<bool>]) -> Vec<Vec<bool>>{
    let mut next = Vec::with_capacity(state.len());
    for (r, row) in state.iter().enumerate() {
        next.push(Vec::with_capacity(row.len()));
        for (c, was_alive) in row.iter().enumerate() {
            next[r].push(alive(state, r, c, *was_alive))
        }
    }

    next
}
fn alive(state : &[Vec<bool>], r: usize, c: usize, was_alive: bool) -> bool{
    let r = r as isize;
    let c = c as isize;
    let around = vec![
        get(state, r - 1, c - 1),
        get(state, r - 0, c - 1),
        get(state, r + 1, c - 1),
        get(state, r - 1, c - 0),
        get(state, r + 1, c - 0),
        get(state, r - 1, c + 1),
        get(state, r - 0, c + 1),
        get(state, r + 1, c + 1),
    ];
    let around = around.iter().filter(|a|**a).count();
    if was_alive {
        around == 2 || around == 3
    }else {
        around == 3
    }
}
fn get(state : &[Vec<bool>], r: isize, c: isize) -> bool{
    if r < 0 || c < 0 || r >= ISIZE || c >= ISIZE {
        false
    }else {
        state[r as usize][c as usize]
    }
}
