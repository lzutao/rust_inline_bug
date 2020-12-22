use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
struct Character{
    pub hp : u8,
    pub dmg : u8,
    pub def : u8,
    pub mana : u32,
    pub shield_effect: Option<u8>,
    pub poison_effect: Option<u8>,
    pub recharge_effect: Option<u8>,
    pub total_spend_mana : u32,
}

#[derive(Debug, Copy, Clone)]
enum Spell{
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}
impl From<u8> for Spell{
    fn from(s: u8) -> Self {
        match s {
            3 => Spell::MagicMissile,
            4 => Spell::Drain,
            2 => Spell::Shield,
            0 => Spell::Poison,
            1 => Spell::Recharge,
            _ => unreachable!()
        }
    }
}

impl FromStr for Character{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let hp = lines.next().unwrap().split_at(12).1.parse().unwrap();
        let dmg = lines.next().unwrap().split_at(8).1.parse().unwrap();
        Ok(Character{hp, dmg, def: 0, mana: 0,
            shield_effect: None, poison_effect: None, recharge_effect: None,
            total_spend_mana : 0
        })
    }
}

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2015/22");
    let boos : Character = INPUT.parse().unwrap();

    let player = Character{
        hp: 50,
        dmg: 0,
        def: 0,
        mana: 500,
        shield_effect: None,
        poison_effect: None,
        recharge_effect: None,
        total_spend_mana: 0
    };

    let mut tmp = u32::MAX;
    let min_cost = fights(player, boos, false, &mut tmp);

    println!("[PART 1] Minimum mana spend to win: {}", min_cost);

    let mut tmp = u32::MAX;
    let min_cost = fights(player, boos, true, &mut tmp);
    println!("[PART 2] Minimum mana spend to win on hard: {}", min_cost);

}

fn fights(player : Character, boos : Character, hard : bool, least_mana: &mut u32) -> u32 {
    for s in 0..5 {
        let mut player = player.clone();
        let mut boos = boos.clone();
        if hard {
            player.hp = player.hp.saturating_sub(1);
        }
        player.apply_effects();
        boos.apply_effects();
        if player.is_dead() {
            continue;
        }
        if boos.is_dead() {
            if *least_mana > player.total_spend_mana {
                *least_mana = player.total_spend_mana;
            }
            continue;
        }
        if player.mana < 53 {
            continue;
        }
        let spell = s.into();
        if !player.can_cast_spell(&mut boos, spell) {
            continue;
        }
        player.cast_spell(&mut boos, spell);
        if boos.is_dead() {
            if *least_mana > player.total_spend_mana {
                *least_mana = player.total_spend_mana;
            }
            continue;
        }
        if player.total_spend_mana >= *least_mana {
            continue;
        }

        player.apply_effects();
        boos.apply_effects();
        if player.is_dead() {
            continue
        }
        if boos.is_dead() {
            if *least_mana > player.total_spend_mana {
                *least_mana = player.total_spend_mana;
            }
            continue;
        }
        let attack = boos.dmg.saturating_sub(player.def).max(1);
        player.hp = player.hp.saturating_sub(attack);
        if player.is_dead() {
            continue
        }
        fights(player, boos, hard, least_mana);
    }
    *least_mana
}

impl Character {
    pub fn can_cast_spell(&self, target : &mut Character, spell: Spell) -> bool{
        match spell {
            Spell::MagicMissile => self.mana > 53,
            Spell::Drain => self.mana >= 73,
            Spell::Shield => self.mana > 113 && self.shield_effect.is_none(),
            Spell::Poison => self.mana > 173 && target.poison_effect.is_none(),
            Spell::Recharge => self.mana > 229 && self.recharge_effect.is_none(),
        }
    }
    pub fn cast_spell(&mut self, target : &mut Character, spell: Spell){
        match spell {
            Spell::MagicMissile => {
                self.mana -= 53;
                self.total_spend_mana += 53;
                target.hp = target.hp.saturating_sub(4);
            }
            Spell::Drain => {
                self.mana -= 73;
                self.total_spend_mana += 73;
                self.hp += 2;
                target.hp = target.hp.saturating_sub(2);
            }
            Spell::Shield => {
                self.mana -= 113;
                self.total_spend_mana += 113;
                self.shield_effect = Some(6);
                self.def += 7;
            }
            Spell::Poison => {
                self.mana -= 173;
                self.total_spend_mana += 173;
                target.poison_effect = Some(6);
            }
            Spell::Recharge => {
                self.mana -= 229;
                self.total_spend_mana += 229;
                self.recharge_effect = Some(5);
            }
        }
    }
    pub fn is_dead(&self) -> bool {
        self.hp == 0
    }
    pub fn apply_effects(&mut self){
        match &mut self.shield_effect {
            None => {}
            Some(duration) => {
                if *duration == 1 {
                    self.def -= 7;
                    self.shield_effect = None
                }else {
                    self.shield_effect = Some(*duration - 1);
                }
            }
        }
        match &mut self.poison_effect {
            None => {}
            Some(duration) => {
                self.hp = self.hp.saturating_sub(3);
                if *duration == 1 {
                    self.poison_effect = None
                }else {
                    self.poison_effect = Some(*duration - 1);
                }
            }
        }
        match &mut self.recharge_effect {
            None => {}
            Some(duration) => {
                self.mana += 101;
                if *duration == 1 {
                    self.recharge_effect = None
                }else {
                    self.recharge_effect = Some(*duration - 1);
                }
            }
        }
    }
}
