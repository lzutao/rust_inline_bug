use std::collections::BTreeMap;

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2015/3");
    let mut x = 0;
    let mut y = 0;
    let mut s_x = 0;
    let mut s_y = 0;
    let mut r_x = 0;
    let mut r_y = 0;
    let mut houses_first_year = BTreeMap::new();
    let mut houses_second_year = BTreeMap::new();
    add_present(&mut houses_first_year, x, y);
    add_present(&mut houses_second_year, s_x, s_y);
    add_present(&mut houses_second_year, r_x, r_y);
    let mut robot_santa = false;
    for d in INPUT.chars().into_iter() {
        move_(d, &mut x, &mut y);
        if robot_santa {
            move_(d, &mut s_x, &mut s_y);
        }else {
            move_(d, &mut r_x, &mut r_y);
        }
        robot_santa = !robot_santa;
        add_present(&mut houses_first_year, x, y);
        add_present(&mut houses_second_year, s_x, s_y);
        add_present(&mut houses_second_year, r_x, r_y);
    }
    println!("[PART 1] Houses with at least one present {}", houses_first_year.len());
    println!("[PART 2] Houses with at least one present {}", houses_second_year.len());
}

fn move_(d: char, x : &mut i32, y : &mut i32)
{
    match d {
        '<' => *x -=1,
        '>' => *x += 1,
        'v' => *y -= 1,
        '^' => *y += 1,
        _ => unreachable!()
    }
}

fn add_present(houses: &mut BTreeMap<(i32, i32), usize>, x : i32, y : i32)
{
    let position = (x, y);
    let house = houses.entry(position).or_insert(0);
    *house += 1;
}