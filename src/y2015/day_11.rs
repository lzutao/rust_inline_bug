use std::collections::BTreeMap;

pub fn solve(){
    const INPUT : &str = include_str!("../../inputs/2015/11");
    let next = next_pass(INPUT);
    println!("[PART 1] Next password will be {}", next);
    let next = next_pass(&next);
    println!("[PART 2] Next password will be {}", next);
}

fn next_pass(old : &str) -> String {
    let mut chars : Vec<char> = old.chars().collect();
    let last = chars.len() - 1;
    loop {
        inc(&mut chars, last);

        let next = chars.iter().collect::<String>();
        if is_valid(&next){
            return next;
        }
    }
}

fn inc(chars : &mut Vec<char>, index : usize){
    chars[index] = (chars[index] as u8 + 1) as char;
    if chars[index] > 'z' {
        chars[index] = 'a';
        inc(chars, index - 1);
    }

}

fn is_valid(pass : &str) -> bool{
    let mut double = BTreeMap::new();
    let mut parsed_double = false;
    let mut seq = false;
    let mut last2 = None;
    let mut last = None;
    for char in pass.chars() {
        match char {
            'i' | 'o' | 'l' => {
                return false;
            }
            _ => {}
        }
        if parsed_double {
            parsed_double = false;
        } else if last == Some(char) {
            let e = double.entry(char).or_insert(0);
            *e += 1;
            parsed_double = true;
        }
        if last2 == Some((char as u8 -2) as char) && last == Some((char as u8 -1) as char){
            seq = true;
        }
        last2 = last;
        last = Some(char);
    }
    seq && double.values().sum::<usize>() > 1
}