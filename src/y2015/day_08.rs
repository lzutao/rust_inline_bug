use regex::Regex;

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2015/8");
    let visible_minus_bytes : usize = INPUT
        .lines()
        .map(visible_minus_bytes)
        .sum();
    println!("[PART 1] Visible - bytes {}", visible_minus_bytes);
    let encoded_minus_visible : usize = INPUT
        .lines()
        .map(encoded_minus_visible)
        .sum();
    println!("[PART 1] Encoded - visible {}", encoded_minus_visible);
}

fn visible_minus_bytes(string : &str) -> usize{
    let visible = string.len();
    let (_, string) = string.split_at(1);
    let (string, _) = string.split_at(string.len() - 1);
    let hex = Regex::new("(\\\\x..)|(\\\\\\\\)|(\\\\\")").unwrap();
    let string = hex.replace_all(&string, "?");
    visible - string.len()
}

fn encoded_minus_visible(string : &str) -> usize{
    let visible = string.len();
    let string = format!("{:?}", string);
    string.len() - visible
}