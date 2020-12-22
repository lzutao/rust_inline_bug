
pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2020/1");
    let input : Vec<usize> = INPUT.lines().map(|l| l.parse().unwrap()).collect();

    'outer1: for i in 0..input.len() - 1 {
        for j in i + 1..input.len() {
            if input[i] + input[j] == 2020 {
                println!("[PART 1] Result {}", input[i] * input[j]);
                break 'outer1;
            }
        }
    }

    'outer2: for i in 0..input.len() - 2 {
        for j in i + 1..input.len() - 1 {
            for k in j + 1..input.len() {
                if input[i] + input[j] + input[k] == 2020 {
                    println!("[PART 2] Result {}", input[i] * input[j] * input[k]);
                    break 'outer2;
                }
            }
        }
    }
}