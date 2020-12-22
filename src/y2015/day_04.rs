
#[cfg(debug_assertions)]
fn warning() { println!("WARNING! this is kind of slow :( try --release to speed it up"); }
#[cfg(not(debug_assertions))]
fn warning() { }

pub fn solve(){
    warning();
    const INPUT : &str = include_str!("../../inputs/2015/4");
    let mut five_zeros_found = false;
    for i in 0.. {
        let hash = md5::compute(format!("{}{}", INPUT, i));
        if hash.0[0] == 0 && hash.0[1] == 0 && hash.0[2] & 0xF0 == 0 {
            if !five_zeros_found {
                five_zeros_found = true;
                println!("[PART 1] Magic number for 5 zeros is {}", i);
            }
            if hash.0[2] == 0 {
                println!("[PART 2] Magic number for 6 zeros is {}", i);
                return;
            }
        }
    }
}