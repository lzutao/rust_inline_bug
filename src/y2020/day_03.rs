
pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2020/3");

    let map : Vec<Vec<bool>> = INPUT
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect();

    let mut x3 = 0;
    let mut trees3 = 0;
    for line in map.iter() {
        trees3 += if line[x3] { 1 } else { 0 };
        x3 = (x3 + 3) % line.len();
    }

    println!("[PART 1] Encountered trees {}", trees3);

    let mut x1 = 0;
    let mut x5 = 0;
    let mut x7 = 0;
    let mut x1y2 = 0;
    let mut y2 = 0;
    let mut trees1 = 0;
    let mut trees5 = 0;
    let mut trees7 = 0;
    let mut trees2 = 0;
    for (y, line) in map.iter().enumerate() {
        trees1 += if line[x1] { 1 } else { 0 };
        x1 = (x1 + 1) % line.len();
        trees5 += if line[x5] { 1 } else { 0 };
        x5 = (x5 + 5) % line.len();
        trees7 += if line[x7] { 1 } else { 0 };
        x7 = (x7 + 7) % line.len();
        if y == y2 {
            y2 += 2;
            trees2 += if line[x1y2] { 1 } else { 0 };
            x1y2 = (x1y2 + 1) % line.len();
        }
    }

    let trees = trees3 * trees1 * trees5 * trees7 * trees2;
    println!("[PART 2] Encountered trees {}", trees);
}