use std::collections::BTreeMap;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Rule {
    pub name : &'static str,
    pub min1 : usize,
    pub max1 : usize,
    pub min2 : usize,
    pub max2 : usize,
}

impl Rule {
    fn from_str(s: &'static str) -> Self {
        let mut s = s.split(": ");
        let name = s.next().unwrap();
        let mut s = s.next().unwrap().split(" or ");
        let mut r = s.next().unwrap().split("-");
        let min1 = r.next().unwrap().parse().unwrap();
        let max1 = r.next().unwrap().parse().unwrap();
        let mut r = s.next().unwrap().split("-");
        let min2 = r.next().unwrap().parse().unwrap();
        let max2 = r.next().unwrap().parse().unwrap();
        Rule{name, min1, max1, min2, max2}
    }
}

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2020/16");

    let mut lines = INPUT.lines();
    let mut rules = Vec::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break
        }
        rules.push(Rule::from_str(line))
    }
    let line = lines.next().unwrap();
    assert_eq!(line, "your ticket:");
    let my_ticket = lines
        .next().unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();
    let line = lines.next().unwrap();
    assert!(line.is_empty());
    let line = lines.next().unwrap();
    assert_eq!(line, "nearby tickets:");
    let mut nearby_tickets = Vec::new();
    for line in lines {
        nearby_tickets.push(line
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect::<Vec<usize>>());
    }

    let error_rate = nearby_tickets
        .iter()
        .flat_map(|v| v)
        .filter_map(|v|{
            let v = *v;
            for rule in &rules {
                if (v >= rule.min1 && v <= rule.max1) || (v >= rule.min2 && v <= rule.max2) {
                    return None;
                }
            }
            Some(v)
        })
        .sum::<usize>();

    println!("[PART 1] Error rate is {}", error_rate);

    let mut possible_rules = Vec::new();
    for _ in 0..rules.len() {
        possible_rules.push(rules.clone());
    }

    let nearby_valid_tickets = nearby_tickets
        .iter()
        .filter_map(|v|{
            if v.iter().all(|v|{
                let v = *v;
                for rule in &rules {
                    if (v >= rule.min1 && v <= rule.max1) || (v >= rule.min2 && v <= rule.max2) {
                        return true;
                    }
                }
                false
            }) {
                Some(v.clone())
            }else {
                None
            }
        })
        .collect::<Vec<Vec<usize>>>();

    for ticket in &nearby_valid_tickets {
        for i in 0..ticket.len() {
            let v = ticket[i];

            possible_rules[i] = possible_rules[i]
                .iter()
                .filter_map(|rule|{
                    if (v >= rule.min1 && v <= rule.max1) || (v >= rule.min2 && v <= rule.max2) {
                        Some(rule.clone())
                    }else {
                        None
                    }
                })
                .collect();
        }
    }

    let mut map = BTreeMap::new();
    loop {
        let mut remove = None;
        for i in 0..possible_rules.len() {
            if possible_rules[i].len() == 1 {
                remove = Some(i);
                break;
            }
        }
        let i = remove.unwrap();
        let rule = possible_rules[i][0].clone();
        map.insert(i, rule.clone());
        for rules in &mut possible_rules {
            for r in 0..rules.len() {
                if rules[r] == rule {
                    rules.remove(r);
                    break;
                }
            }
        }
        if map.len() == possible_rules.len() {
            break;
        }
    }

    let departure = my_ticket
        .iter()
        .enumerate()
        .filter_map(|(i, v)|{
            let rule = map.get(&i).unwrap();
            if rule.name.contains("departure") {
                Some(*v)
            }else {
                None
            }
        })
        .product::<usize>();

    println!("[PART 2] Departure value is {}", departure);
}
