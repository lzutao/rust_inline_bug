use regex::Regex;
use substring::Substring;
use std::collections::BTreeMap;


pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2015/7");
    let gates : Vec<&str> = INPUT
        .lines()
        .collect();
    let mut solved_gates = BTreeMap::new();

    let output_to_a = find_gate(&gates, "a");
    let a = solve_gate(&gates, &mut solved_gates, &output_to_a);
    println!("[PART 1] Value on 'a' {}", a);
    solved_gates.clear();
    solved_gates.insert("b", a);
    let a = solve_gate(&gates, &mut solved_gates, &output_to_a);
    println!("[PART 2] Value on 'a' {}", a);
}

fn find_gate<'a>(gates: &[&'a str], gate: &'a str) -> &'a str {
    println!("before regex");
    println!("regex `{}`", &format!(r"(?i).+\s->\s{}$", gate));
    let regex = Regex::new(&format!(r"(?i).+\s->\s{}$", gate));
    println!("regex {:?}", regex);
    let output_to_gate = regex.unwrap();
    println!("after regex");

    let output_to_gate = gates
        .iter()
        .find(|line| output_to_gate.is_match(line))
        .unwrap();
    output_to_gate.substring(0, output_to_gate.len() - 5)
}

fn solve_gate<'a>(gates: &[&'a str], solved_gates : &mut BTreeMap<&'a str, u16>, gate: &'a str) -> u16{
    //println!("solving gate {}", gate);
    if gate.contains(' ') {
        let parts : Vec<&str> = gate.split(' ').collect();
        if parts[0] == r"NOT" {
            return !solve_gate(gates, solved_gates, parts[1]);
        }
        let left = solve_gate(gates, solved_gates, parts[0]);
        let right = solve_gate(gates, solved_gates, parts[2]);
        if parts[1] == r"OR" {
            left | right
        } else if parts[1] == r"AND" {
            left & right
        } else if parts[1] == r"LSHIFT" {
            left.overflowing_shl(right as u32).0
        } else if parts[1] == r"RSHIFT" {
            left.overflowing_shr(right as u32).0
        } else {
            unimplemented!("{}", parts[1])
        }
    }else {
        if let Ok(val) = gate.parse(){
            val
        }else {
            if let Some(result) = solved_gates.get(gate) {
                *result
            }else {
                let result = solve_gate(gates, solved_gates, find_gate(gates, gate));
                solved_gates.insert(gate, result);
                result
            }
        }
    }
}
