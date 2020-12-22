
pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2020/2");
    let lines : Vec<(usize, usize, char, &str)> = INPUT
        .lines()
        .map(|l| {
            let mut s = l.split(" ");
            let mut range = s.next().unwrap().split("-");
            let min = range.next().unwrap().parse::<usize>().unwrap();
            let max = range.next().unwrap().parse::<usize>().unwrap();
            let char = s.next().unwrap().as_bytes()[0] as char;
            let pass = s.next().unwrap();
            (min, max, char, pass)
        })
        .collect();

    let valid = lines
        .iter()
        .filter(|(min, max, rc, pass)| {
            let count = pass.chars().filter(|c| *c == *rc).count();
            count >= *min && count <= *max
        })
        .count();

    println!("[PART 1] Valid passwords {}", valid);

    let valid = lines
        .iter()
        .filter(|(a, b, rc, pass)| {
            (pass.as_bytes()[*a-1] as char == *rc) ^ (pass.as_bytes()[*b-1] as char == *rc)
        })
        .count();

    println!("[PART 2] Valid passwords {}", valid);
}
