use std::collections::BTreeMap;

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2020/14");

    let mut mem = BTreeMap::new();
    let mut and_mask = u64::MAX;
    let mut or_mask = u64::MIN;
    for line in INPUT.lines() {
        let mut s = line.split(" ");
        let t = s.next().unwrap();
        s.next().unwrap();
        let v = s.next().unwrap();
        if t == "mask" {
            and_mask = u64::MAX;
            or_mask = u64::MIN;
            for (i, char) in v.chars().rev().enumerate() {
                match char {
                    'X' => continue,
                    '1' => {
                        or_mask |= 1 << i;
                    }
                    '0' => {
                        and_mask &= !(1 << i);
                    }
                    _ => unreachable!()
                }
            }
        } else {
            let a : u64 = t.split_at(t.len() - 1).0.split_at(4).1.parse().unwrap();
            let v : u64 = v.parse().unwrap();
            let v = (v & and_mask) | or_mask;
            let _old = mem.insert(a, v);
        }
    }

    println!("[PART 1] Sum {}", mem.values().sum::<u64>());


    let mut mem = BTreeMap::new();
    let mut mask = "";
    for line in INPUT.lines() {
        let mut s = line.split(" ");
        let t = s.next().unwrap();
        s.next().unwrap();
        let v = s.next().unwrap();
        if t == "mask" {
            mask = v;
        } else {
            let a : u64 = t.split_at(t.len() - 1).0.split_at(4).1.parse().unwrap();
            let v : u64 = v.parse().unwrap();
            write_all(&mut mem, v, a, mask);
        }
    }

    println!("[PART 2] Sum {}", mem.values().sum::<u64>());
}

fn write_all(mem: &mut BTreeMap<u64, u64>, value: u64, address: u64, mask: &str) {
    let mut bits = Vec::new();
    for (i, char) in mask.chars().rev().enumerate() {
        match char {
            'X' => bits.push(0..=1),
            '1' => bits.push(1..=1),
            '0' => {
                let a = (address >> i) & 1;
                bits.push(a..=a)
            }
            _ => unreachable!()
        }
    }
    for b0 in bits[0].clone() {
        for b1 in bits[1].clone() {
            for b2 in bits[2].clone() {
                for b3 in bits[3].clone() {
                    for b4 in bits[4].clone() {
                        for b5 in bits[5].clone() {
                            for b6 in bits[6].clone() {
                                for b7 in bits[7].clone() {
                                    for b8 in bits[8].clone() {
                                        for b9 in bits[9].clone() {
                                            for b10 in bits[10].clone() {
                                                for b11 in bits[11].clone() {
                                                    for b12 in bits[12].clone() {
                                                        for b13 in bits[13].clone() {
                                                            for b14 in bits[14].clone() {
                                                                for b15 in bits[15].clone() {
                                                                    for b16 in bits[16].clone() {
                                                                        for b17 in bits[17].clone() {
                                                                            for b18 in bits[18].clone() {
                                                                                for b19 in bits[19].clone() {
                                                                                    for b20 in bits[20].clone() {
                                                                                        for b21 in bits[21].clone() {
                                                                                            for b22 in bits[22].clone() {
                                                                                                for b23 in bits[23].clone() {
                                                                                                    for b24 in bits[24].clone() {
                                                                                                        for b25 in bits[25].clone() {
                                                                                                            for b26 in bits[26].clone() {
                                                                                                                for b27 in bits[27].clone() {
                                                                                                                    for b28 in bits[28].clone() {
                                                                                                                        for b29 in bits[29].clone() {
                                                                                                                            for b30 in bits[30].clone() {
                                                                                                                                for b31 in bits[31].clone() {
                                                                                                                                    for b32 in bits[32].clone() {
                                                                                                                                        for b33 in bits[33].clone() {
                                                                                                                                            for b34 in bits[34].clone() {
                                                                                                                                                for b35 in bits[35].clone() {
let a = b0 | b1 << 1 | b2 << 2 | b3 << 3 | b4 << 4 | b5 << 5 | b6 << 6 | b7 << 7 | b8 << 8
    | b9 << 9 | b10 << 10 | b11 << 11 | b12 << 12 | b13 << 13 | b14 << 14 | b15 << 15 | b16 << 16
    | b17 << 17 | b18 << 18 | b19 << 19 | b20 << 20 | b21 << 21 | b22 << 22 | b23 << 23
    | b24 << 24 | b25 << 25 | b26 << 26 | b27 << 27 | b28 << 28 | b29 << 29 | b30 << 30
    | b31 << 31 | b32 << 32 | b33 << 33 | b34 << 34 | b35 << 35;
mem.insert(a, value);
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
