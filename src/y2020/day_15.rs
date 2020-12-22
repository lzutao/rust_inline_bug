use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

pub fn solve(){
    println!("WARNING! this is kind of slow :(");
    const INPUT : &str =  include_str!("../../inputs/2020/15");

    let numbers = INPUT
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut last_seen: BTreeMap<_, _> = numbers[..numbers.len() - 1]
        .iter()
        .copied()
        .enumerate()
        .map(|(i, n)| (n, i))
        .collect();
    let _30000000th = (numbers.len()..30_000_000)
        .fold(*numbers.last().unwrap(), |n, i| {
        let n = match last_seen.entry(n) {
            Entry::Occupied(mut occ) => i - occ.insert(i - 1) - 1,
            Entry::Vacant(vac) => {
                vac.insert(i - 1);
                0
            }
        };
        if i == 2020 - 1 {
            println!("[PART 1] 2020th number is {}", n);
        }
        n
    });
    println!("[PART 2] 30000000th number is {}", _30000000th);
}
