use std::collections::BTreeMap;


pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2015/13");
    let stats = INPUT
        .lines()
        .map(|l| {
            let (l, _) = l.split_at(l.len() - 1);
            let p : Vec<&str> = l.split(" ").collect();
            (p[0], p[2], p[3].parse::<i32>().unwrap(), p[10])
        })
        .map(|v| (v.0, v.3, if v.1 == "gain" {v.2}else{-v.2}));

    let mut people = BTreeMap::new();
    for stat in stats {
        let e = people.entry(stat.0).or_insert(BTreeMap::new());
        e.insert(stat.1, stat.2);
    }

    let happiness = best_order(&people);

    println!("[PART 1] Total happiness {}", happiness);
    let keys = people.keys().map(|k| *k).collect::<Vec<&str>>();
    const ME : &str = "me";
    for key in keys {
        let e = people.entry(ME).or_insert(BTreeMap::new());
        e.insert(key, 0);
        let e = people.entry(key).or_insert(BTreeMap::new());
        e.insert(ME, 0);
    }
    let happiness = best_order(&people);

    println!("[PART 2] Total happiness {}", happiness);
}

fn best_order(people: &BTreeMap<&str, BTreeMap<&str, i32>>) -> i32 {
    let keys = people.keys().map(|k| *k).collect::<Vec<&str>>();
    let orders = orders(Vec::new(), &keys);

    orders.iter().map(|o|{
        let len = o.len();
        let mut h = *people.get(o[len-1]).unwrap().get(o[0]).unwrap();
        h += *people.get(o[0]).unwrap().get(o[len-1]).unwrap();
        for i in 1..len {
            h += *people.get(o[i-1]).unwrap().get(o[i]).unwrap();
            h += *people.get(o[i]).unwrap().get(o[i-1]).unwrap();
        }
        h
    }).max().unwrap()
}

fn orders<'a>(current: Vec<&'a str>, people: &[&'a str]) -> Vec<Vec<&'a str>>{
    let other : Vec<&str> = people.iter()
        .filter(|p| !current.contains(p))
        .map(|p| *p)
        .collect();
    let mut possible_orders = Vec::new();
    if other.is_empty() {
        return vec![current];
    }
    for other in other {
        let mut c = current.clone();
        c.push(other);
        possible_orders.append(&mut orders(c, people));
    }
    possible_orders
}