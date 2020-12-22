use std::collections::BTreeMap;

pub fn solve(){
    const INPUT : &str = include_str!("../../inputs/2016/4");

    let rooms : Vec<(&str, usize, &str)> = INPUT
        .lines()
        .map(|l|{
            let (name, id_checksum) = l.split_at(l.rfind('-').unwrap());
            let (id, checksum) = id_checksum
                .split_at(1).1
                .split_at(id_checksum.find('[').unwrap() - 1);
            let (checksum, _) = checksum
                .split_at(1).1
                .split_at(checksum.len() - 2);
            (name, id.parse().unwrap(), checksum)
        })
        .collect();

    let mut sum_of_sectors = 0;
    'outer: for (name, sector, checksum) in &rooms {
        let mut chars = BTreeMap::new();
        for char in name.chars() {
            if char == '-' {
                continue
            }
            if !chars.contains_key(&char) {
                chars.insert(char, name.chars().filter(|c| *c == char).count());
            }
        }
        for char in checksum.chars() {
            let count = match chars.get(&char) {
                Some(c) => c,
                None => continue 'outer,
            };
            for (_, c) in &chars {
                if *c > *count {
                    continue 'outer;
                }
            }
            chars.remove(&char);
        }
        sum_of_sectors += sector;
        let decrypted : String = name.chars().map(|c| decrypt_char(c, *sector)).collect();
        // println!("{} {}", decrypted, sector);
        if decrypted.contains("northpole") {
            println!("[PART 1] Sector ID of '{}' is {}", decrypted, sector);
        }
    }

    println!("[PART 1] Sum of sectors {}", sum_of_sectors);
}

fn decrypt_char(mut c : char, key : usize) -> char {
    if c == '-' {
        return ' ';
    }
    for _ in 0..key {
        if c == 'z' {
            c = 'a'
        }else {
            c = (c as u8 + 1) as char;
        }
    }

    c
}
