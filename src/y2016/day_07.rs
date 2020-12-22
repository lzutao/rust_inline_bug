
pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2016/7");

    let tls_supported : Vec<&str> = INPUT
        .lines()
        .filter(tls_support)
        .collect();

    println!("[PART 1] IP's with TLS support {}", tls_supported.len());

    let ssl_supported : Vec<&str> = INPUT
        .lines()
        .filter(ssl_support)
        .collect();

    println!("[PART 2] IP's with SSL support {}", ssl_supported.len());
}

fn tls_support(s: &&str) -> bool {
    let mut inside = false;
    let mut contains = false;
    let mut checking = false;
    let mut p1 = '?';
    let mut p2 = '?';
    let mut p3 = '?';
    for char in s.chars() {
        if char == '[' {
            inside = true;
            checking = false;
        } else if char == ']' {
            inside = false;
            checking = false;
        } else if checking {
            if char == p3 {
                if inside {
                    return false;
                } else {
                    contains = true;
                }
            }
            checking = false;
        } else {
            if char == p1 && p1 != p2{
                checking = true;
            }
        }
        p3 = p2;
        p2 = p1;
        p1 = char;
    }
    contains
}

fn ssl_support(s: &&str) -> bool {
    let mut inside = false;
    let mut p1 = '?';
    let mut p2 = '?';
    let mut ab = Vec::new();
    let mut ba = Vec::new();
    for char in s.chars() {
        if char == '[' {
            inside = true;
        } else if char == ']' {
            inside = false;
        } else if char == p2 && char != p1 && p1 != '[' && p1 != ']' {
            if inside {
                ba.push((p2, p1))
            } else {
                ab.push((p2, p1))
            }
        }
        p2 = p1;
        p1 = char;
    }

    ab
        .iter()
        .any(|(a, b)|{
            ba.contains(&(*b, *a))
        })
}
