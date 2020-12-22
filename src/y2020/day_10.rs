
pub fn solve(){
    const INPUT : &str = include_str!("../../inputs/2020/10");
    let mut adapters : Vec<usize> = INPUT
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    adapters.sort();

    let mut differences_of_1_jolt = 0;
    let mut differences_of_3_jolt = 1;
    let mut last = 0;
    for a in &adapters {
        let d = *a - last;
        last = *a;
        if d == 1 {
            differences_of_1_jolt += 1;
        }
        else if d == 3 {
            differences_of_3_jolt += 1;
        }
    }

    println!("[PART 1] differences multiplied {}", differences_of_1_jolt * differences_of_3_jolt);

    let mut arrangements : usize = 1;
    let lookup = [1, 1, 2, 4, 7, 13, 24, 44];
    let mut prev = 0;
    let mut i = 0;
    let len = adapters.len() - 1;
    if adapters[0] == 3 { // first one starting with 3
        i = 1;
        prev = 1;
    } else { // we need to account for 0 -> adapters[i]
        loop {
            if adapters[i+1] - adapters[i] == 3 {
                arrangements *= lookup[i + 1 - prev];
                i += 1;
                prev = i;
                break;
            }
            i += 1;
            if i >= len {
                break;
            }
        }
    }

    loop {
        if adapters[i+1] - adapters[i] == 3 {
            arrangements *= lookup[i - prev];
            prev = i + 1;
        }
        i += 1;
        if i >= len {
            break;
        }
    }
    arrangements *= lookup[i - prev];

    println!("[PART 2] Possible arrangements {}", arrangements);
}
