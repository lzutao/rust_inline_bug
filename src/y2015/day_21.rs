use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
struct Character{
    pub hp : u8,
    pub dmg : u8,
    pub def : u8,
}

#[derive(Debug, Copy, Clone)]
struct Item{
    pub cost : u16,
    pub dmg : u8,
    pub def : u8,
}

impl FromStr for Character{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let hp = lines.next().unwrap().split_at(12).1.parse().unwrap();
        let dmg = lines.next().unwrap().split_at(8).1.parse().unwrap();
        let def = lines.next().unwrap().split_at(7).1.parse().unwrap();
        Ok(Character{hp, dmg, def})
    }
}

const WEAPONS : [Item;5] = [
    Item{cost: 8, dmg: 4, def: 0},
    Item{cost: 10, dmg: 5, def: 0},
    Item{cost: 25, dmg: 6, def: 0},
    Item{cost: 40, dmg: 7, def: 0},
    Item{cost: 74, dmg: 8, def: 0},
];
const ARMORS : [Item;6] = [
    Item{cost: 0, dmg: 0, def: 0},  // added no armor
    Item{cost: 13, dmg: 0, def: 1},
    Item{cost: 31, dmg: 0, def: 2},
    Item{cost: 53, dmg: 0, def: 3},
    Item{cost: 75, dmg: 0, def: 4},
    Item{cost: 102, dmg: 0, def: 5},
];
const RINGS : [Item;8] = [
    Item{cost: 0, dmg: 0, def: 0}, // added no ring left
    Item{cost: 0, dmg: 0, def: 0}, // added no ring right
    Item{cost: 25, dmg: 1, def: 0},
    Item{cost: 50, dmg: 2, def: 0},
    Item{cost: 100, dmg: 3, def: 0},
    Item{cost: 20, dmg: 0, def: 1},
    Item{cost: 40, dmg: 0, def: 2},
    Item{cost: 80, dmg: 0, def: 3},
];

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2015/21");
    let boos : Character = INPUT.parse().unwrap();

    let min_cost = pick_gear_and_win_fight(boos);

    println!("[PART 1] Minimum gold spend to win: {}", min_cost);

    let max_cost = pick_gear_and_lose_fight(boos.clone());

    println!("[PART 2] Maximum gold spend while still losing: {}", max_cost);

}

fn pick_gear_and_win_fight(boos: Character) -> u16{
    let mut min_price = u16::MAX;
    for weapon in WEAPONS.iter() {
        for armor in ARMORS.iter() {
            for (ri, ring1) in RINGS.iter().enumerate() {
                for r2 in 0..RINGS.len() {
                    if r2 == ri {
                        continue
                    }
                    let ring2 = &RINGS[r2];
                    let mut player = Character{
                        hp: 100,
                        dmg: weapon.dmg + armor.dmg + ring1.dmg + ring2.dmg,
                        def: weapon.def + armor.def + ring1.def + ring2.def,
                    };
                    let mut boos = boos.clone();
                    if fight(&mut player, &mut boos) {
                        let cost = weapon.cost + armor.cost + ring1.cost + ring2.cost;
                        //println!("c:{} w:{:?} a:{:?} r1:{:?} r2:{:?}", cost, weapon, armor, ring1, ring2);
                        if cost < min_price {
                            min_price = cost;
                        }
                    }
                }
            }
        }
    }
    min_price
}

fn pick_gear_and_lose_fight(boos: Character) -> u16{
    let mut max_price = u16::MIN;
    for weapon in WEAPONS.iter() {
        for armor in ARMORS.iter() {
            for (ri, ring1) in RINGS.iter().enumerate() {
                for r2 in 0..RINGS.len() {
                    if r2 == ri {
                        continue
                    }
                    let ring2 = &RINGS[r2];
                    let mut player = Character{
                        hp: 100,
                        dmg: weapon.dmg + armor.dmg + ring1.dmg + ring2.dmg,
                        def: weapon.def + armor.def + ring1.def + ring2.def,
                    };
                    let mut boos = boos.clone();
                    if !fight(&mut player, &mut boos) {
                        let cost = weapon.cost + armor.cost + ring1.cost + ring2.cost;
                        //println!("c:{} w:{:?} a:{:?} r1:{:?} r2:{:?}", cost, weapon, armor, ring1, ring2);
                        if cost > max_price {
                            max_price = cost;
                        }
                    }
                }
            }
        }
    }
    max_price
}

fn fight(player : &mut Character, boos : &mut Character) -> bool {
    loop {
        let attack = player.dmg.saturating_sub(boos.def).max(1);
        boos.hp = boos.hp.saturating_sub(attack);
        if boos.hp == 0 {
            return true;
        }
        let attack = boos.dmg.saturating_sub(player.def).max(1);
        player.hp = player.hp.saturating_sub(attack);
        if player.hp == 0 {
            return false;
        }
    }
}