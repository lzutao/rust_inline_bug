use std::collections::BTreeSet;

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2020/7");
    let bags : Vec<(&str, Vec<(usize, &str)>)> = INPUT
        .lines()
        .map(baggy_bags)
        .collect();

    println!("[PART 1] Possible outer bags {}", contains_shiny_gold(&bags));

    println!("[PART 2] Inside bags {}", inside_bag(&bags, "shiny gold"));
}

fn inside_bag(bags : &Vec<(&str, Vec<(usize, &str)>)>, bag: &str) -> usize {
    let mut inside = 0;
    for (outer, inners) in bags {
        if *outer == bag {
            for (inner_count, inner) in inners {
                inside += inner_count + inner_count * inside_bag(bags, inner);
            }
        }
    }
    inside
}

fn contains_shiny_gold(bags : &Vec<(&str, Vec<(usize, &str)>)>) -> usize {
    let mut map = BTreeSet::new();
    for (outer, inners) in bags {
        if inners.iter().any(|(_, i)| *i == "shiny gold") {
            map.insert(outer);
        }
    }
    loop {
        let count = map.len();
        let prev_map = map.clone();
        for (outer, inners) in bags {
            if inners.iter().any(|(_, i)| prev_map.contains(i)) {
                map.insert(outer);
            }
        }
        let new_count = map.len();
        if new_count != count {
            continue
        }
        return count;
    }
}

fn baggy_bags(line: &str) -> (&str, Vec<(usize, &str)>) {
    let mut parts = line.split(" bags contain ");
    let outer = parts.next().unwrap();
    let mut inner = Vec::new();
    for mut i in parts.next().unwrap().split(" bag") {
        if i.len() < 3 {
            continue
        }
        let chars = i.as_bytes();
        let s = chars[0] as char == 's';
        if s || chars[0] as char == ',' {
            let mut chars = i.chars();
            if s {
                chars.next().unwrap();
            }
            chars.next().unwrap();
            chars.next().unwrap();
            i = chars.as_str();
        }
        if i == "no other" {
            continue
        }
        let s = i.find(' ').unwrap();
        let (count, bag) = i.split_at(s);
        let (_ ,bag) = bag.split_at(1);
        let count = count.parse().unwrap();
        inner.push((count, bag));
    }

    (outer, inner)
}
