use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[cfg(debug_assertions)]
fn warning() { println!("WARNING! this is kind of slow :( try --release to speed it up"); }
#[cfg(not(debug_assertions))]
fn warning() { }

pub fn solve(){
    warning();
    const INPUT : &str =  include_str!("../../inputs/2020/22");

    let lines = INPUT.lines().collect::<Vec<&str>>();
    let midpoint = (lines.len() - 1) / 2;
    let p1 = &lines[1..midpoint];
    let p2 = &lines[midpoint + 2..];
    let p1 : VecDeque<usize> = p1.iter().map(|c| c.parse().unwrap()).collect();
    let p2 : VecDeque<usize> = p2.iter().map(|c| c.parse().unwrap()).collect();

    let winning_score = play_game(p1.clone(), p2.clone(), false);
    println!("[PART 1] Winning score {}", winning_score);

    let winning_score = play_game(p1, p2, true);
    println!("[PART 2] Winning score {}", winning_score);
}

fn play_game(p1: VecDeque<usize>, p2: VecDeque<usize>, recursive: bool) -> usize {
    let (_, winning_deck) = play_game_inner(p1, p2, recursive);
    winning_deck.iter().rev().enumerate().map(|(m, c)| (m + 1) * c).sum()
}

fn play_game_inner(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>, recursive: bool)
                   -> (bool, VecDeque<usize>){
    let mut history = Vec::new();
    loop {
        if p1.is_empty() {
            return (false, p2);
        }
        if p2.is_empty() {
            return (true, p1);
        }
        if recursive {
            let mut hasher = DefaultHasher::new();
            p1.hash(&mut hasher);
            p2.hash(&mut hasher);
            let h = hasher.finish();
            if history.contains(&h) {
                return (true, p1);
            }
            history.push(h);
        }

        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        let p1_win = if recursive {
            if c1 <= p1.len() && c2 <= p2.len() {
                let p1 = p1
                    .iter()
                    .copied()
                    .take(c1)
                    .collect::<VecDeque<usize>>();
                let p2 = p2
                    .iter()
                    .copied()
                    .take(c2)
                    .collect::<VecDeque<usize>>();
                play_game_inner(p1, p2, true).0
            }else {
                c1 > c2
            }
        } else {
            c1 > c2
        };
        if p1_win {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }
}
