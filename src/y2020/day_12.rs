
#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;

impl Direction {
    pub fn right(&self, mut degrees: isize) -> Self {
        let mut target = *self;
        while degrees > 0 {
            target = match target {
                North => East,
                East => South,
                South => West,
                West => North,
            };
            degrees -= 90;
        }
        target
    }
    pub fn left(&self, mut degrees: isize) -> Self {
        let mut target = *self;
        while degrees > 0 {
            target = match target {
                North => West,
                East => North,
                South => East,
                West => South,
            };
            degrees -= 90;
        }
        target
    }
}

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2020/12");
    let directions : Vec<(char, isize)> = INPUT
        .lines()
        .map(|d| {
            let (dir, distance) = d.split_at(1);
            (dir.chars().next().unwrap(), distance.parse().unwrap())
        })
        .collect();

    let mut x : isize = 0;
    let mut y : isize = 0;
    let mut dir = East;
    for (direction, dist) in &directions {
        match *direction {
            'N' => y += dist,
            'S' => y -= dist,
            'E' => x += dist,
            'W' => x -= dist,
            'L' => dir = dir.left(*dist),
            'R' => dir = dir.right(*dist),
            'F' => {
                match dir {
                    North => y += dist,
                    East => x += dist,
                    South => y -= dist,
                    West => x -= dist,
                }
            }
            _ => unreachable!(),
        }
    }

    println!("[PART 1] Distance is {}", x.abs() + y.abs());

    let mut x : isize = 0;
    let mut y : isize = 0;
    let mut wx : isize = 10;
    let mut wy : isize = 1;
    for (direction, dist) in &directions {
        match *direction {
            'N' => wy += dist,
            'S' => wy -= dist,
            'E' => wx += dist,
            'W' => wx -= dist,
            'L' => {
                let mut degrees = *dist;
                while degrees > 0 {
                    std::mem::swap(&mut wx, &mut wy);
                    wx = -wx;
                    degrees -= 90;
                }
            },
            'R' => {
                let mut degrees = *dist;
                while degrees > 0 {
                    std::mem::swap(&mut wx, &mut wy);
                    wy = -wy;
                    degrees -= 90;
                }
            },
            'F' => {
                x += wx * dist;
                y += wy * dist;
            }
            _ => unreachable!(),
        }
    }

    println!("[PART 2] Distance is {}", x.abs() + y.abs());
}