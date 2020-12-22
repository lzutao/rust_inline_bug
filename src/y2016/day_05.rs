use std::time::Instant;

pub fn solve(){
    let s = Instant::now();
    println!("WARNING! this is kind of slow :(");
    const INPUT : &str = include_str!("../../inputs/2016/5");
    let mut password = String::new();
    let mut pass_chars = [u8::MAX;8];
    for i in 0.. {
        let hash = md5::compute(format!("{}{}", INPUT, i));
        if hash.0[0] == 0 && hash.0[1] == 0 && hash.0[2] & 0xF0 == 0 {

            let six = hash.0[2] & 0xF;
            if password.len() < 8 {
                password = format!("{}{:x}", password, six);
            }

            if six < 8 && pass_chars[six as usize] == u8::MAX {
                pass_chars[six as usize] = hash.0[3] >> 4;
                if pass_chars.iter().all(|c| *c < u8::MAX) {
                    break;
                }
            }
        }
    }
    println!("[PART 1] Password is {}", password);
    println!("[PART 2] Password is {:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}",
             pass_chars[0], pass_chars[1], pass_chars[2], pass_chars[3],
             pass_chars[4], pass_chars[5], pass_chars[6], pass_chars[7]);
    println!("{:?}", s.elapsed())
}