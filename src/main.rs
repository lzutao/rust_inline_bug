use std::time::Instant;

mod y2015;
mod y2016;
mod y2020;

fn main() {
    solve(Some(2015), Some(7));
}

fn solve(year : Option<u16>, day : Option<u8>){
    match year{
        None => for year in 2015..=2020 { solve(Some(year), day) },
        Some(year) => {
            println!("year {}", year);
            let s = Instant::now();
            match year {
                2015 => y2015::solve(day),
                2016 => y2016::solve(day),
                2020 => y2020::solve(day),
                _ => unreachable!()
            }
            println!("[year {}] solved in: {:?}", year, s.elapsed());
        },
    }
}
