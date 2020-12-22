
pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2015/1");
    let mut floor = 0;
    let mut was_in_the_basement = false;
    for (position, d) in INPUT.chars().into_iter().enumerate() {
        match d {
            '(' => floor +=1,
            ')' => {
                floor -= 1;
                if !was_in_the_basement && floor < 0 {
                    was_in_the_basement = true;
                    println!("[PART 2] First time in basement at position {}", position + 1); // 1 based index
                }
            }
            _ => unreachable!()
        }
    }
    println!("[PART 1] Target floor {}", floor);
}