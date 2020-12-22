use std::collections::BTreeMap;

enum Rule {
    Char(char),
    One(usize),
    Two(usize, usize),
    OrOne(usize, usize),
    OrTwo((usize, usize), (usize, usize)),
    OneOrTwo(usize, (usize, usize)),
    TwoOrTree((usize, usize), (usize, usize, usize)),
}
use Rule::*;
impl Rule {
    pub fn matches(&self, rules : &BTreeMap<usize, Rule>, text : &'static str)
        -> Option<Vec<&'static str>> {
        match self {
            Char(c) => {
                if text.len() >= 1 && text.as_bytes()[0] as char == *c {
                    Some(vec![text.split_at(1).1])
                }else {
                    None
                }
            },
            One(r) => rules.get(r).unwrap().matches(rules, text),
            Two(a, b) => Rule::match_pair(rules, text, a, b),
            OrOne(a, b) => {
                let a = rules.get(a).unwrap()
                    .matches(rules, text);
                let b = rules.get(b).unwrap()
                    .matches(rules, text);
                match (a, b) {
                    (None, None) => None,
                    (None, Some(b)) => Some(b),
                    (Some(a), None) => Some(a),
                    (Some(mut a), Some(b)) => {
                        a.extend(b.iter());
                        Some(a)
                    }
                }
            }
            OrTwo((a, b), (c, d)) => {
                let ab = Rule::match_pair(rules, text, a, b);
                let cd = Rule::match_pair(rules, text, c, d);
                match (ab, cd) {
                    (None, None) => None,
                    (None, Some(cd)) => Some(cd),
                    (Some(ab), None) => Some(ab),
                    (Some(mut ab), Some(cd)) => {
                        ab.extend(cd.iter());
                        Some(ab)
                    }
                }
            }
            OneOrTwo(a, (b, c)) => {
                let bc = Rule::match_pair(rules, text, b, c);
                match (rules.get(a).unwrap().matches(rules, text), bc) {
                    (None, None) => None,
                    (None, Some(bc)) => Some(bc),
                    (Some(a), None) => Some(a),
                    (Some(mut a), Some(bc)) => {
                        a.extend(bc.iter());
                        Some(a)
                    }
                }
            }
            TwoOrTree((a, b), (c, d, e)) => {
                let ab = Rule::match_pair(rules, text, a, b);
                let cde = Rule::match_triple(rules, text, c, d, e);
                match (ab, cde) {
                    (None, None) => None,
                    (None, Some(cde)) => Some(cde),
                    (Some(ab), None) => Some(ab),
                    (Some(mut ab), Some(cde)) => {
                        ab.extend(cde.iter());
                        Some(ab)
                    }
                }
            }
        }
    }

    fn match_pair(rules: &BTreeMap<usize, Rule>, text: &'static str, a: &usize, b: &usize)
                  -> Option<Vec<&'static str>> {
        let b = rules.get(b).unwrap();
        match rules.get(a).unwrap()
            .matches(rules, text)
            .map(|v| v.iter()
                .filter_map(|t| b.matches(rules, t))
                .flatten()
                .collect::<Vec<&str>>()) {
            None => None,
            Some(v) => {
                if v.is_empty() {
                    None
                } else {
                    Some(v)
                }
            }
        }
    }

    fn match_triple(rules: &BTreeMap<usize, Rule>, text: &'static str, a: &usize, b: &usize, c: &usize)
                  -> Option<Vec<&'static str>> {
        let b = rules.get(b).unwrap();
        let c = rules.get(c).unwrap();
        match rules.get(a).unwrap()
            .matches(rules, text)
            .map(|v| v.iter()
                .filter_map(|t| b.matches(rules, t))
                .flatten()
                .filter_map(|t| c.matches(rules, t))
                .flatten()
                .collect::<Vec<&str>>()) {
            None => None,
            Some(v) => {
                if v.is_empty() {
                    None
                } else {
                    Some(v)
                }
            }
        }
    }
}

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2020/19");

    let mut rules = BTreeMap::new();
    let mut lines = INPUT.lines();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(": ");
        let id = parts.next().unwrap().parse::<usize>().unwrap();
        let rule = parts.next().unwrap();
        if rule.contains('|') {
            let mut parts = rule.split(" ");
            let p1 = parts.next().unwrap().parse().unwrap();
            let p2 = parts.next().unwrap();
            if p2 == "|" {
                let p2 = parts.next().unwrap().parse().unwrap();
                assert_eq!(parts.next(), None);
                rules.insert(id, OrOne(p1, p2));
            }else {
                let p2 = p2.parse().unwrap();
                parts.next().unwrap();
                let p3 = parts.next().unwrap().parse().unwrap();
                let p4 = parts.next().unwrap().parse().unwrap();
                assert_eq!(parts.next(), None);
                rules.insert(id, OrTwo((p1, p2), (p3, p4)));
            }
        } else if rule.as_bytes()[0] as char == '"' {
            rules.insert(id, Char(rule.as_bytes()[1] as char));
        } else {
            let mut parts = rule.split(" ");
            let p1 = parts.next().unwrap().parse().unwrap();
            match parts.next() {
                None => {
                    rules.insert(id, One(p1));
                }
                Some(p2) => {
                    assert_eq!(parts.next(), None);
                    rules.insert(id, Two(p1, p2.parse().unwrap()));
                }
            }
        }
    }

    let messages = lines.collect::<Vec<&str>>();

    let r0 = rules.get(&0).unwrap();
    let matches = messages
        .iter()
        .filter(|m| {
            match r0.matches(&rules, m) {
                None => false,
                Some(v) => {
                    v.iter().any(|r| r.is_empty())
                }
            }
        })
        .count();

    println!("[PART 1] Matches {}", matches);

    rules.insert(8, OneOrTwo(42, (42, 8))).unwrap();
    rules.insert(11, TwoOrTree((42, 31), (42, 11, 31))).unwrap();

    let r0 = rules.get(&0).unwrap();
    let matches = messages
        .iter()
        .filter(|m| {
            match r0.matches(&rules, m) {
                None => false,
                Some(v) => {
                    v.iter().any(|r| r.is_empty())
                }
            }
        })
        .count();

    println!("[PART 2] Matches {}", matches);
}

