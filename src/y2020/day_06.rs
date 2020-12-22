use std::collections::BTreeSet;

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2020/6");
    const EMPTY: [&str;1] = [""];

    let mut groups = Vec::new();
    groups.push((BTreeSet::new(), Vec::new()));
    for line in INPUT.lines().chain(EMPTY.iter().map(|s|*s)) {
        if line.is_empty() {
            groups.push((BTreeSet::new(), Vec::new()));
            continue;
        }
        let (anyone, all) = groups.last_mut().unwrap();
        let first = anyone.is_empty();
        for char in line.chars() {
            anyone.insert(char);
            if first {
                all.push(char);
            }
        }
        all.retain(|c| line.contains(*c));
    }
    let sum = groups
        .iter()
        .map(|(anyone, _)| anyone.len())
        .sum::<usize>();

    println!("[PART 1] The sum is {}", sum);

    let sum = groups
        .iter()
        .map(|(_, all)| all.len())
        .sum::<usize>();

    println!("[PART 2] The sum is {}", sum);
}