
#[derive(Copy, Clone, Eq, PartialEq)]
enum Seat{
    Occupied,
    Empty,
    Floor
}
use Seat::*;

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2020/11");

    let mut seats = Vec::with_capacity(100);
    for line in INPUT.lines() {
        let mut row = Vec::with_capacity(100);
        for seat in line.chars() {
            match seat {
                'L' => row.push(Occupied),
                '.' => row.push(Floor),
                _ => unreachable!(),
            }
        }
        seats.push(row)
    }

    let mut prev_seats = seats.clone();
    loop {
        let seats = step(&prev_seats, alive);
        if seats == prev_seats {
            break
        } else {
            prev_seats = seats;
        }
    }

    let occupied = prev_seats
        .iter()
        .map(|r|r.iter().filter(|a|**a == Occupied).count())
        .sum::<usize>();

    println!("[PART 1] Occupied seats {}", occupied);

    let mut prev_seats = seats.clone();
    loop {
        let seats = step(&prev_seats, alive2);
        let occupied = prev_seats
            .iter()
            .map(|r|r.iter().filter(|a|**a == Occupied).count())
            .sum::<usize>();
        println!("Occupied seats {}", occupied);
        if seats == prev_seats {
            break
        } else {
            prev_seats = seats;
        }
    }

    let occupied = prev_seats
        .iter()
        .map(|r|r.iter().filter(|a|**a == Occupied).count())
        .sum::<usize>();

    println!("[PART 2] Occupied seats {}", occupied);
}

fn step(state : &[Vec<Seat>], alive: fn(&[Vec<Seat>], usize, usize, Seat)-> Seat)
    -> Vec<Vec<Seat>>
{
    let mut next = Vec::with_capacity(state.len());
    for (r, row) in state.iter().enumerate() {
        next.push(Vec::with_capacity(row.len()));
        for (c, was_alive) in row.iter().enumerate() {
            if *was_alive == Floor {
                next[r].push(Floor)
            }else {
                next[r].push(alive(state, r, c, *was_alive))
            }
        }
    }

    next
}
fn alive(state : &[Vec<Seat>], r: usize, c: usize, was_alive: Seat) -> Seat{
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
    match was_alive{
        Occupied => if around >= 4 { Empty } else { Occupied },
        Empty => if around == 0 { Occupied } else { Empty },
        Floor => Floor,
    }
}
fn get(state : &[Vec<Seat>], r: isize, c: isize) -> bool{
    if r < 0 || c < 0 || r >= state.len() as isize || c >= state[r as usize].len() as isize {
        false
    }else {
        state[r as usize][c as usize] == Occupied
    }
}
fn get2(state : &[Vec<Seat>], mut r: isize, rd: isize, mut c: isize, cd: isize) -> bool{
    loop{
        r = r + rd;
        c = c + cd;
        let next = if r < 0 || c < 0 ||
            r >= state.len() as isize || c >= state[r as usize].len() as isize {
            return false;
        }else {
            state[r as usize][c as usize]
        };
        if next != Floor {
            return next == Occupied;
        }
    }
}
fn alive2(state : &[Vec<Seat>], r: usize, c: usize, was_alive: Seat) -> Seat{
    let r = r as isize;
    let c = c as isize;
    let around = vec![
        get2(state, r, - 1, c, - 1),
        get2(state, r, - 0, c, - 1),
        get2(state, r,  1, c, - 1),
        get2(state, r, - 1, c, - 0),
        get2(state, r,  1, c, - 0),
        get2(state, r, - 1, c,  1),
        get2(state, r, - 0, c,  1),
        get2(state, r,  1, c,  1),
    ];
    let around = around.iter().filter(|a|**a).count();
    match was_alive{
        Occupied => if around >= 5 { Empty } else { Occupied },
        Empty => if around == 0 { Occupied } else { Empty },
        Floor => Floor,
    }
}
