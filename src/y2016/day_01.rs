
#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;
use std::collections::HashSet;

impl Direction {
    pub fn right(&self) -> Self {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
    pub fn left(&self) -> Self {
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }
}

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2016/1");
    let directions : Vec<(char, isize)> = INPUT
        .lines()
        .next()
        .unwrap()
        .split(", ")
        .map(|d| {
            let (dir, distance) = d.split_at(1);
            (dir.chars().next().unwrap(), distance.parse().unwrap())
        })
        .collect();

    let mut explored = HashSet::new();
    let mut x : isize = 0;
    let mut y : isize = 0;
    explored.insert((x, y));
    let mut dir = North;
    let mut twice = false;
    for (direction, dist) in &directions {
        match *direction {
            'R' => dir = dir.right(),
            'L' => dir = dir.left(),
            _ => unreachable!(),
        }
        if !twice{
            for _ in 0..*dist {
                match dir {
                    North => y += 1,
                    East => x += 1,
                    South => y -= 1,
                    West => x -= 1,
                }

                if !explored.insert((x, y)) {
                    if !twice {
                        twice = true;
                        println!("[PART 2] Distance to first location visited twice is {}",
                                 x.abs() + y.abs());
                    }
                }
            }
        }else {
            match dir {
                North => y += dist,
                East => x += dist,
                South => y -= dist,
                West => x -= dist,
            }
        }
    }

    println!("[PART 1] Distance is {}", x.abs() + y.abs());
}