
enum Token {
    Raw(usize),
    Multiple(usize, Vec<Token>),
}
use Token::*;
impl Token {
    pub fn len(&self) -> usize {
        match self {
            Raw(v) => *v,
            Multiple(m, v) => {
                *m * v.iter().map(|t| t.len()).sum::<usize>()
            }
        }
    }
}

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2016/9");

    let decompressed = decompress(INPUT.lines().next().unwrap());

    println!("[PART 1] Decompressed length {}", decompressed.len());

    let decompressed = decompress_tokens(INPUT.lines().next().unwrap());
    let decompressed_len : usize = decompressed
        .iter()
        .map(|t| t.len())
        .sum();

    println!("[PART 2] Decompressed length {}", decompressed_len);
}

fn decompress_tokens(s: &str) -> Vec<Token> {
    let mut chars = s.chars();
    let mut decompressed = Vec::with_capacity(s.len());
    decompressed.push(Raw(0));
    loop {
        let c = chars.next();
        if c.is_none() {
            break;
        }
        let c = c.unwrap();
        if c == '(' {
            let pat: String = chars.clone().take_while(|c| *c != ')').collect();
            for _ in 0..=pat.len() {
                chars.next().unwrap();
            }
            let mut pat = pat.split("x");
            let i: usize = pat.next().unwrap().parse().unwrap();
            let o: usize = pat.next().unwrap().parse().unwrap();
            let it: String = chars.clone().take(i).collect();
            for _ in 0..i {
                chars.next().unwrap();
            }
            decompressed.push(Multiple(o, decompress_tokens(&it)));
            decompressed.push(Raw(0));
        } else {
            match decompressed.last_mut().unwrap(){
                Raw(v) => *v += 1,
                Multiple(_, _) => unreachable!(),
            }
        }
    }

    decompressed
}

fn decompress(s: &str) -> String {
    let mut chars = s.chars();
    let mut decompressed = String::with_capacity(s.len() * 2);
    loop {
        let c = chars.next();
        if c.is_none() {
            break;
        }
        let c = c.unwrap();
        if c == '(' {
            let pat: String = chars.clone().take_while(|c| *c != ')').collect();
            for _ in 0..=pat.len() {
                chars.next().unwrap();
            }
            let mut pat = pat.split("x");
            let i: usize = pat.next().unwrap().parse().unwrap();
            let o: usize = pat.next().unwrap().parse().unwrap();
            let it: String = chars.clone().take(i).collect();
            for _ in 0..i {
                chars.next().unwrap();
            }
            for _ in 0..o {
                decompressed.push_str(&it)
            }
        } else {
            decompressed.push(c);
        }
    }
    decompressed
}
