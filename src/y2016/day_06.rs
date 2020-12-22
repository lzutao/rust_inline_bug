use std::collections::BTreeMap;

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2016/6");
    let mut chars = Vec::new();
    for _ in INPUT.lines().next().unwrap().chars() {
        chars.push(BTreeMap::new())
    }
    for line in INPUT.lines() {
        for (i, char) in line.chars().enumerate() {
            let v = chars[i].entry(char).or_insert(0);
            *v += 1;
        }
    }

    let message = chars
        .iter()
        .map(|chars|
            chars
                .iter()
                .max_by(|(_, c1), (_, c2)| c1.cmp(c2))
                .map(|(c, _)| *c))
        .map(|c| c.unwrap())
        .collect::<String>();

    println!("[PART 1] Message is {}", message);

    let message = chars
        .iter()
        .map(|chars|
            chars
                .iter()
                .min_by(|(_, c1), (_, c2)| c1.cmp(c2))
                .map(|(c, _)| *c))
        .map(|c| c.unwrap())
        .collect::<String>();

    println!("[PART 2] Message is {}", message);
}
