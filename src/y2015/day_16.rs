use std::str::FromStr;

#[derive(Default)]
struct Clue{
    pub children: Option<usize>,
    pub cats: Option<usize>,
    pub samoyeds: Option<usize>,
    pub pomeranians: Option<usize>,
    pub akitas: Option<usize>,
    pub vizslas: Option<usize>,
    pub goldfish: Option<usize>,
    pub trees: Option<usize>,
    pub cars: Option<usize>,
    pub perfumes: Option<usize>,
}

impl FromStr for Clue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts : Vec<&str> = s.split(" ").collect();
        let mut parts = parts.iter();
        parts.next().unwrap();
        parts.next().unwrap();
        let mut clue = Clue::default();
        while let Some(p) = parts.next(){
            let val = parts.next().unwrap();
            let val = val.split(",").next().unwrap().parse().unwrap();
            if *p == "children:" {
                clue.children = Some(val);
            } else if *p == "cats:" {
                clue.cats = Some(val);
            } else if *p == "samoyeds:" {
                clue.samoyeds = Some(val);
            } else if *p == "pomeranians:" {
                clue.pomeranians = Some(val);
            } else if *p == "akitas:" {
                clue.akitas = Some(val);
            } else if *p == "vizslas:" {
                clue.vizslas = Some(val);
            } else if *p == "goldfish:" {
                clue.goldfish = Some(val);
            } else if *p == "trees:" {
                clue.trees = Some(val);
            } else if *p == "cars:" {
                clue.cars = Some(val);
            } else if *p == "perfumes:" {
                clue.perfumes = Some(val);
            }
        }
        Ok(clue)
    }
}

impl Clue {
    pub fn match_exact(&self, other: &Self) -> bool{
        other.children.map_or(true, |v| Some(v) == self.children)
            && other.cats.map_or(true, |v| Some(v) == self.cats)
            && other.samoyeds.map_or(true, |v| Some(v) == self.samoyeds)
            && other.pomeranians.map_or(true, |v| Some(v) == self.pomeranians)
            && other.akitas.map_or(true, |v| Some(v) == self.akitas)
            && other.vizslas.map_or(true, |v| Some(v) == self.vizslas)
            && other.goldfish.map_or(true, |v| Some(v) == self.goldfish)
            && other.trees.map_or(true, |v| Some(v) == self.trees)
            && other.cars.map_or(true, |v| Some(v) == self.cars)
            && other.perfumes.map_or(true, |v| Some(v) == self.perfumes)
    }
    pub fn match_advanced(&self, other: &Self) -> bool{
        other.children.map_or(true, |v| Some(v) == self.children)
            && other.cats.map_or(true, |v| v > self.cats.unwrap())
            && other.samoyeds.map_or(true, |v| Some(v) == self.samoyeds)
            && other.pomeranians.map_or(true, |v| v < self.pomeranians.unwrap())
            && other.akitas.map_or(true, |v| Some(v) == self.akitas)
            && other.vizslas.map_or(true, |v| Some(v) == self.vizslas)
            && other.goldfish.map_or(true, |v| v < self.goldfish.unwrap())
            && other.trees.map_or(true, |v| v > self.trees.unwrap())
            && other.cars.map_or(true, |v| Some(v) == self.cars)
            && other.perfumes.map_or(true, |v| Some(v) == self.perfumes)
    }
}


pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2015/16");
    let aunts : Vec<(usize, Clue)> = INPUT
        .lines()
        .map(|l| l.parse().unwrap())
        .enumerate()
        .collect();

    const CLUES : Clue = Clue{
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    let aunt = aunts
        .iter()
        .filter_map(|(a, c)| { if CLUES.match_exact(c) { Some(*a) }else { None }
        }).next().unwrap();

    println!("[PART 1] Aunt number {} send the gift", aunt + 1);

    let aunt = aunts
        .iter()
        .filter_map(|(a, c)| { if CLUES.match_advanced(c) { Some(*a) }else { None }
        }).next().unwrap();

    println!("[PART 2] Aunt number {} send the gift", aunt + 1);
}

#[allow(dead_code)]
pub fn solve_part_1(){
    const INPUT : &str =  include_str!("../../inputs/2015/16");
    let aunts : Vec<(usize, &str)> = INPUT
        .lines()
        .enumerate()
        .collect();

    const CLUES : &str =
"children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1";
    let clues : Vec<(&str, &str)> = CLUES
        .lines()
        .map(|l|{
            let clue = l.split(" ").next().unwrap();
            (clue, l)
        })
        .collect();

    let aunt = aunts
        .iter()
        .filter_map(|(a, i)|{
            for (clue, line) in &clues {
                if i.contains(clue) && !i.contains(line) {
                    return None;
                }
            }
            Some(*a)
        }).next().unwrap();

    println!("[PART 1] Aunt number {} send the gift", aunt + 1);
}