
pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2020/13");
    let mut input = INPUT.lines();
    let time = input.next().unwrap().parse::<usize>().unwrap();
    let busses = input
        .next().unwrap()
        .split(",")
        .map(|b| b.parse().ok())
        .collect::<Vec<Option<usize>>>();
    let real_busses = busses
        .iter()
        .flat_map(|b| *b)
        .collect::<Vec<usize>>();

    let first = real_busses
        .iter()
        .map(|b| {
            ((((time - 1 + b) / b) * b)-time, b)
        })
        .min_by(|(n1, _), (n2, _)| n1.cmp(n2))
        .unwrap();

    println!("[PART 1] ID * time {}", first.1 * first.0);

    let min = crt(&busses
        .iter()
        .enumerate()
        .filter_map(|(i, b)| b.map(|b|(i, b)))
        .map(|(i, b)| {
            let i = i as isize;
            let mut i = -i;
            while i < 0 {
                i += b as isize
            }
            (i as u128, b as u128)
        })
        .collect());

    println!("[PART 2] min time {}", min);
}

fn crt(pairs: &Vec<(u128, u128)>) -> u128 {
    let mut m = 1;
    for (_, mx) in pairs {
        m *= mx
    }
    let mut total = 0;
    for (x, mx) in pairs {
        let b = m / mx;
        total += x * b * mod_pow(b, mx - 2, *mx);
        total %= m;
    }
    total
}

fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    if modulus == 1 { return 0 }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}
