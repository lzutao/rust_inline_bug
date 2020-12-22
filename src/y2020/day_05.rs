
pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2020/5");
    let mut  numbers = INPUT
        .lines()
        .map(bsp_to_number)
        .collect::<Vec<u16>>();

    println!("[PART 1] Max seat number {}", numbers.iter().max().unwrap());
    numbers.sort();
    let mut seat = numbers[0];
    for n in &numbers[1..] {
        if *n == seat + 2 {
            seat = seat + 1;
            println!("[PART 2] My seat is {}", seat);
            return;
        }
        seat = *n;
    }
}

fn bsp_to_number(bsp: &str) -> u16{
    let mut row_number = 0;
    let (row, col) = bsp.split_at(7);
    for char in row.chars() {
        row_number = row_number << 1;
        if char == 'B' {
            row_number |= 1;
        }
    }
    let mut col_number = 0;
    for char in col.chars() {
        col_number = col_number << 1;
        if char == 'R' {
            col_number |= 1;
        }
    }

    //println!("{}:{}, {}:{}, {}", row, row_number, col, col_number, row_number * 44 + col_number);
    row_number * 8 + col_number
}