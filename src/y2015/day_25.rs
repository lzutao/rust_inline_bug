
pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2015/25");
    let mut s = INPUT.split(",");
    s.next().unwrap();
    let a = s
        .next().unwrap()
        .split(" ")
        .last().unwrap()
        .parse::<usize>().unwrap();
    let b  = s
        .next().unwrap()
        .split(" ")
        .last().unwrap()
        .split(".")
        .next().unwrap()
        .parse::<usize>().unwrap();

    println!("[PART 1] Code: {}", get_code(get_code_index(a, b)));
}

fn get_code(index: usize) -> usize {
    let mut code = 20151125;
    for _ in 1..index {
        code = (code * 252533) % 33554393
    }
    code
}

fn get_code_index(mut row: usize, mut col: usize) -> usize {
    let mut index = 1;
    loop {
        if row == 1 && col == 1 {
            return index;
        } else {
            if col > 1{
                row = row + 1;
                col = col - 1;
            }else{
                col = row - 1;
                row = 1;
            };
            index += 1;
        }
    }
}
