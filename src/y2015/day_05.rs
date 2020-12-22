use std::option::Option::Some;

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2015/5");
    let mut old_nice = 0;
    let mut new_nice = 0;
    for string in INPUT.lines() {
        if is_old_nice(string) {
            old_nice += 1;
        }
        if is_new_nice(string) {
            new_nice += 1;
        }
    }
    println!("[PART 1] Nice strings {}", old_nice);
    println!("[PART 2] Nice strings {}", new_nice);
}

fn is_old_nice(string : &str) -> bool{
    let mut vowels  = 0;
    let mut double = false;
    let mut last = None;
    for char in string.chars() {
        match char {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowels += 1;
            }
            'b' => if last == Some('a') { return false; }
            'd' => if last == Some('c') { return false; }
            'q' => if last == Some('p') { return false; }
            'y' => if last == Some('x') { return false; }
            _ => {}
        }
        if last == Some(char) {
            double = true;
        }
        last = Some(char);
    }

    vowels >= 3 && double
}

fn is_new_nice(string : &str) -> bool{
    let mut chars = string.chars();
    let mut last = None;
    let mut pair = false;
    let mut repeat = false;
    while let Some(char) = chars.next() {
        if chars.clone().next() == last {
            repeat = true;
        }
        last.map(|last|{
            let chars = chars.clone();
            let mut can_be = false;
            for next in chars {
                if can_be && next == char{
                    pair = true;
                    return;
                }else if next == last {
                    can_be = true
                }else {
                    can_be = false
                }
            }
        });

        last = Some(char);
    }
    pair && repeat
}