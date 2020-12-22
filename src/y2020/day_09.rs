
pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2020/9");
    let numbers : Vec<usize> = INPUT
        .lines()
        .map(|n| n.parse().unwrap())
        .collect();

    let mut target = usize::MAX;
    for i in 25..numbers.len() {
        let n = numbers[i];
        let mut ok = false;
        'outer: for j in i - 25..i-1 {
            let nj = numbers[j];
            for k in j+1..i {
                if nj + numbers[k] == n {
                    ok = true;
                    break 'outer;
                }
            }
        }
        if !ok {
            target = n;
            break;
        }
    }
    println!("[PART 1] Target number {}", target);

    for i in 0..numbers.len() {
        let n = numbers[i];
        let mut sum = numbers[i];
        for j in i + 1..numbers.len() {
            sum += numbers[j];
            if sum == target {
                println!("[PART 2] Weakness in encryption {}", n + numbers[j]);
            } else if sum > target {
                break;
            }
        }
    }
}
