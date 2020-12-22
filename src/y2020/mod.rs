use std::time::Instant;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;


pub fn solve(day: Option<u8>)
{
    match day{
        Some(day) => solve_day(day),
        None => for day in 1..=25 { solve_day(day) },
    }
}

fn solve_day(day: u8)
{
    assert!(day > 0);
    assert!(day <= 25);
    println!("day {}", day);
    let s = Instant::now();
    match day {
        1 => day_01::solve(),
        2 => day_02::solve(),
        3 => day_03::solve(),
        4 => day_04::solve(),
        5 => day_05::solve(),
        6 => day_06::solve(),
        7 => day_07::solve(),
        8 => day_08::solve(),
        9 => day_09::solve(),
        10 => day_10::solve(),
        11 => day_11::solve(),
        12 => day_12::solve(),
        13 => day_13::solve(),
        14 => day_14::solve(),
        15 => day_15::solve(),
        16 => day_16::solve(),
        17 => day_17::solve(),
        18 => day_18::solve(),
        19 => day_19::solve(),
        20 => day_20::solve(),
        21 => day_21::solve(),
        22 => day_22::solve(),
        /*23 => day_23::solve(),
        24 => day_24::solve(),
        25 => day_25::solve(),*/
        _ => unreachable!()
    }
    println!("[day {}] solved in: {:?}", day, s.elapsed());
}