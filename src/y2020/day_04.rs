
pub fn solve(){
    const INPUT : &str = include_str!("../../inputs/2020/4");
    const EMPTY: [&str;1] = [""];

    let mut byr = false;
    let mut iyr = false;
    let mut eyr = false;
    let mut hgt = false;
    let mut hcl = false;
    let mut ecl = false;
    let mut pid = false;
    let mut byr_valid = false;
    let mut iyr_valid = false;
    let mut eyr_valid = false;
    let mut hgt_valid = false;
    let mut hcl_valid = false;
    let mut ecl_valid = false;
    let mut pid_valid = false;
    let mut valid = 0;
    let mut really_valid = 0;
    for line in INPUT.lines().chain(EMPTY.iter().map(|s|*s)) {

        if line.is_empty() {
            if byr && iyr && eyr && hgt && hcl && ecl && pid {
                valid += 1;
            }
            byr = false;
            iyr = false;
            eyr = false;
            hgt = false;
            hcl = false;
            ecl = false;
            pid = false;

            if byr_valid && iyr_valid && eyr_valid && hgt_valid
                && hcl_valid && ecl_valid && pid_valid {
                really_valid += 1;
            }
            byr_valid = false;
            iyr_valid = false;
            eyr_valid = false;
            hgt_valid = false;
            hcl_valid = false;
            ecl_valid = false;
            pid_valid = false;

            continue;
        }

        let parts = line.split(" ");
        for part in parts {
            let mut i_v = part.split(":");
            let i = i_v.next().unwrap();
            let v = i_v.next().unwrap();
            if i == "byr" {
                byr = true;
                v.parse::<i32>().map(|v|{
                    if v >= 1920 && v <= 2002 {
                        byr_valid = true;
                    }
                }).unwrap_or(());
            }
            if i == "iyr" {
                iyr = true;
                v.parse::<i32>().map(|v|{
                    if v >= 2010 && v <= 2020 {
                        iyr_valid = true;
                    }
                }).unwrap_or(());
            }
            if i == "eyr" {
                eyr = true;
                v.parse::<i32>().map(|v|{
                    if v >= 2020 && v <= 2030 {
                        eyr_valid = true;
                    }
                }).unwrap_or(());
            }
            if i == "hgt" {
                hgt = true;
                let (v, u) = v.split_at(v.len()-2);
                let v = v.parse().unwrap_or(0);
                if u == "cm" && v >= 150 && v <= 193 {
                    hgt_valid = true;
                } else if u == "in" && v >= 59 && v <= 76 {
                    hgt_valid = true;
                }
            }
            if i == "hcl" {
                hcl = true;
                if v.as_bytes()[0] as char == '#' {
                    let v = v.split_at(1).1;
                    if v.len() == 6
                        && v.chars().filter(|c| c.is_ascii_hexdigit()).count() == 6 {
                        hcl_valid = true;
                    }
                }
            }
            if i == "ecl" {
                ecl = true;
                if v == "amb" || v == "blu" || v == "brn" || v == "gry"
                    || v == "grn" || v == "hzl" || v == "oth" {
                    ecl_valid = true;
                }
            }
            if i == "pid" {
                pid = true;
                if v.len() == 9
                    && v.chars().filter(|c| c.is_ascii_digit()).count() == 9 {
                    pid_valid = true;
                }
            }
        }
    }

    println!("[PART 1] There is {} 'valid' passports", valid);
    println!("[PART 2] There is {} 'more valid' passports", really_valid);
}