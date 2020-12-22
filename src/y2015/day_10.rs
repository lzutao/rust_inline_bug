use std::str::FromStr;

pub fn solve(){
    const INPUT : &str = include_str!("../../inputs/2015/10");
    if false {
        brute_force(INPUT) // really really really slow !
    }else {
        smart(INPUT)
    }
}
//[PART 1] Length of the result 360154
//[PART 2] Length of the result 5103798

fn smart(input : &str){
    let mut elements = elements_from_str(input);
    for _ in 0..40 {
        elements = look_ad_say_elements(&elements);
    }
    let len_40 = elements.iter().map(|e|e.len()).sum::<usize>();
    println!("[PART 1] Length of the result {}", len_40);
    for _ in 0..10 {
        elements = look_ad_say_elements(&elements);
    }
    let len_50 = elements.iter().map(|e|e.len()).sum::<usize>();
    println!("[PART 2] Length of the result {}", len_50);
}

fn brute_force(input : &str){
    println!("WARNING! this is kind of really slow (~2 hours on my PC) :(");
    let mut input = input.to_string();
    for _ in 0..40 {
        input = look_ad_say(&input);
    }
    println!("[PART 1] Length of the result {}", input.len());
    for _ in 0..10 {
        input = look_ad_say(&input);
    }
    println!("[PART 2] Length of the result {}", input.len());
}

fn look_ad_say(input: &str) -> String{
    let mut next = String::new();
    let mut last = '?';
    let mut count = 1;
    for char in input.chars() {
        if char == last {
            count += 1;
        }else if last != '?' {
            next = format!("{}{}{}", next, count, last);
            count = 1;
        }
        last = char;
    }
    format!("{}{}{}", next, count, last)
}

fn look_ad_say_elements(input: &[Element]) -> Vec<Element>{
    input.iter().flat_map(|e| e.next()).collect()
}

fn elements_from_str(s : &str) -> Vec<Element>{
    let e : Element = s.parse().unwrap();
    match e {
        Element::Compound(_) => {},
        e => { return vec![e]; }
    }
    split(s)
}

enum Element {
    Compound(String),
    H,
    He,
    Li,
    Be,
    B,
    C,
    N,
    O,
    F,
    Ne,
    Na,
    Mg,
    Al,
    Si,
    P,
    S,
    Cl,
    Ar,
    K,
    Ca,
    Sc,
    Ti,
    V,
    Cr,
    Mn,
    Fe,
    Co,
    Ni,
    Cu,
    Zn,
    Ga,
    Ge,
    As,
    Se,
    Br,
    Kr,
    Rb,
    Sr,
    Y,
    Zr,
    Nb,
    Mo,
    Tc,
    Ru,
    Rh,
    Pd,
    Ag,
    Cd,
    In,
    Sn,
    Sb,
    Te,
    I,
    Xe,
    Cs,
    Ba,
    La,
    Ce,
    Pr,
    Nd,
    Pm,
    Sm,
    Eu,
    Gd,
    Tb,
    Dy,
    Ho,
    Er,
    Tm,
    Yb,
    Lu,
    Hf,
    Ta,
    W,
    Re,
    Os,
    Ir,
    Pt,
    Au,
    Hg,
    Tl,
    Pb,
    Bi,
    Po,
    At,
    Rn,
    Fr,
    Ra,
    Ac,
    Th,
    Pa,
    U,
}

impl Element {
    pub fn next(&self) -> Vec<Self> {
        match self {
            Element::Compound(_) => {unimplemented!()}
            Element::H => vec![Element::H],
            Element::He => vec![Element::Hf, Element::Pa, Element::H, Element::Ca, Element::Li],
            Element::Li => vec![Element::He],
            Element::Be => vec![Element::Ge, Element::Ca, Element::Li],
            Element::B => vec![Element::Be],
            Element::C => vec![Element::B],
            Element::N => vec![Element::C],
            Element::O => vec![Element::N],
            Element::F => vec![Element::O],
            Element::Ne => vec![Element::F],
            Element::Na => vec![Element::Ne],
            Element::Mg => vec![Element::Pm, Element::Na],
            Element::Al => vec![Element::Mg],
            Element::Si => vec![Element::Al],
            Element::P => vec![Element::Ho, Element::Si],
            Element::S => vec![Element::P],
            Element::Cl => vec![Element::S],
            Element::Ar => vec![Element::Cl],
            Element::K => vec![Element::Ar],
            Element::Ca => vec![Element::K],
            Element::Sc => vec![Element::Ho, Element::Pa, Element::H, Element::Ca, Element::Co],
            Element::Ti => vec![Element::Sc],
            Element::V => vec![Element::Ti],
            Element::Cr => vec![Element::V],
            Element::Mn => vec![Element::Cr, Element::Si],
            Element::Fe => vec![Element::Mn],
            Element::Co => vec![Element::Fe],
            Element::Ni => vec![Element::Zn, Element::Co],
            Element::Cu => vec![Element::Ni],
            Element::Zn => vec![Element::Cu],
            Element::Ga => vec![Element::Eu, Element::Ca, Element::Ac, Element::H, Element::Ca, Element::Zn],
            Element::Ge => vec![Element::Ho, Element::Ga],
            Element::As => vec![Element::Ge, Element::Na],
            Element::Se => vec![Element::As],
            Element::Br => vec![Element::Se],
            Element::Kr => vec![Element::Br],
            Element::Rb => vec![Element::Kr],
            Element::Sr => vec![Element::Rb],
            Element::Y => vec![Element::Sr, Element::U],
            Element::Zr => vec![Element::Y, Element::H, Element::Ca, Element::Tc],
            Element::Nb => vec![Element::Er, Element::Zr],
            Element::Mo => vec![Element::Nb],
            Element::Tc => vec![Element::Mo],
            Element::Ru => vec![Element::Eu, Element::Ca, Element::Tc],
            Element::Rh => vec![Element::Ho, Element::Ru],
            Element::Pd => vec![Element::Rh],
            Element::Ag => vec![Element::Pd],
            Element::Cd => vec![Element::Ag],
            Element::In => vec![Element::Cd],
            Element::Sn => vec![Element::In],
            Element::Sb => vec![Element::Pm, Element::Sn],
            Element::Te => vec![Element::Eu, Element::Ca, Element::Sb],
            Element::I => vec![Element::Ho, Element::Te],
            Element::Xe => vec![Element::I],
            Element::Cs => vec![Element::Xe],
            Element::Ba => vec![Element::Cs],
            Element::La => vec![Element::Ba],
            Element::Ce => vec![Element::La, Element::H, Element::Ca, Element::Co],
            Element::Pr => vec![Element::Ce],
            Element::Nd => vec![Element::Pr],
            Element::Pm => vec![Element::Nd],
            Element::Sm => vec![Element::Pm, Element::Ca, Element::Zn],
            Element::Eu => vec![Element::Sm],
            Element::Gd => vec![Element::Eu, Element::Ca, Element::Co],
            Element::Tb => vec![Element::Ho, Element::Gd],
            Element::Dy => vec![Element::Tb],
            Element::Ho => vec![Element::Dy],
            Element::Er => vec![Element::Ho, Element::Pm],
            Element::Tm => vec![Element::Er, Element::Ca, Element::Co],
            Element::Yb => vec![Element::Tm],
            Element::Lu => vec![Element::Yb],
            Element::Hf => vec![Element::Lu],
            Element::Ta => vec![Element::Hf, Element::Pa, Element::H, Element::Ca, Element::W],
            Element::W => vec![Element::Ta],
            Element::Re => vec![Element::Ge, Element::Ca, Element::W],
            Element::Os => vec![Element::Re],
            Element::Ir => vec![Element::Os],
            Element::Pt => vec![Element::Ir],
            Element::Au => vec![Element::Pt],
            Element::Hg => vec![Element::Au],
            Element::Tl => vec![Element::Hg],
            Element::Pb => vec![Element::Tl],
            Element::Bi => vec![Element::Pm, Element::Pb],
            Element::Po => vec![Element::Bi],
            Element::At => vec![Element::Po],
            Element::Rn => vec![Element::Ho, Element::At],
            Element::Fr => vec![Element::Rn],
            Element::Ra => vec![Element::Fr],
            Element::Ac => vec![Element::Ra],
            Element::Th => vec![Element::Ac],
            Element::Pa => vec![Element::Th],
            Element::U => vec![Element::Pa],
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Element::Compound(s) => s.len(),
            Element::H => 2,
            Element::He => 32,
            Element::Li => 27,
            Element::Be => 42,
            Element::B => 34,
            Element::C => 28,
            Element::N => 28,
            Element::O => 18,
            Element::F => 14,
            Element::Ne => 12,
            Element::Na => 9,
            Element::Mg => 10,
            Element::Al => 10,
            Element::Si => 7,
            Element::P => 12,
            Element::S => 10,
            Element::Cl => 6,
            Element::Ar => 4,
            Element::K => 4,
            Element::Ca => 2,
            Element::Sc => 16,
            Element::Ti => 14,
            Element::V => 8,
            Element::Cr => 5,
            Element::Mn => 12,
            Element::Fe => 8,
            Element::Co => 5,
            Element::Ni => 8,
            Element::Cu => 6,
            Element::Zn => 3,
            Element::Ga => 17,
            Element::Ge => 23,
            Element::As => 26,
            Element::Se => 20,
            Element::Br => 16,
            Element::Kr => 14,
            Element::Rb => 10,
            Element::Sr => 7,
            Element::Y => 7,
            Element::Zr => 23,
            Element::Nb => 28,
            Element::Mo => 20,
            Element::Tc => 15,
            Element::Ru => 21,
            Element::Rh => 24,
            Element::Pd => 18,
            Element::Ag => 12,
            Element::Cd => 10,
            Element::In => 8,
            Element::Sn => 5,
            Element::Sb => 7,
            Element::Te => 13,
            Element::I => 18,
            Element::Xe => 14,
            Element::Cs => 8,
            Element::Ba => 6,
            Element::La => 5,
            Element::Ce => 10,
            Element::Pr => 8,
            Element::Nd => 6,
            Element::Pm => 3,
            Element::Sm => 6,
            Element::Eu => 7,
            Element::Gd => 11,
            Element::Tb => 16,
            Element::Dy => 12,
            Element::Ho => 7,
            Element::Er => 9,
            Element::Tm => 14,
            Element::Yb => 10,
            Element::Lu => 6,
            Element::Hf => 5,
            Element::Ta => 32,
            Element::W => 27,
            Element::Re => 42,
            Element::Os => 34,
            Element::Ir => 28,
            Element::Pt => 24,
            Element::Au => 18,
            Element::Hg => 14,
            Element::Tl => 12,
            Element::Pb => 9,
            Element::Bi => 10,
            Element::Po => 10,
            Element::At => 7,
            Element::Rn => 12,
            Element::Fr => 10,
            Element::Ra => 6,
            Element::Ac => 4,
            Element::Th => 4,
            Element::Pa => 13,
            Element::U => 1,
        }
    }
}

impl FromStr for Element {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let e = if s == "3" { Element::U }
        else if s == "13" { Element::Pa }
        else if s == "1113" { Element::Th }
        else if s == "3113" { Element::Ac }
        else if s == "132113" { Element::Ra }
        else if s == "1113122113" { Element::Fr }
        else if s == "311311222113" { Element::Rn }
        else if s == "1322113" { Element::At }
        else if s == "1113222113" { Element::Po }
        else if s == "3113322113" { Element::Bi }
        else if s == "123222113" { Element::Pb }
        else if s == "111213322113" { Element::Tl }
        else if s == "31121123222113" { Element::Hg }
        else if s == "132112211213322113" { Element::Au }
        else if s == "111312212221121123222113" { Element::Pt }
        else if s == "3113112211322112211213322113" { Element::Ir }
        else if s == "1321132122211322212221121123222113" { Element::Os }
        else if s == "111312211312113221133211322112211213322113" { Element::Re }
        else if s == "312211322212221121123222113" { Element::W }
        else if s == "13112221133211322112211213322113" { Element::Ta }
        else if s == "11132" { Element::Hf }
        else if s == "311312" { Element::Lu }
        else if s == "1321131112" { Element::Yb }
        else if s == "11131221133112" { Element::Tm }
        else if s == "311311222" { Element::Er }
        else if s == "1321132" { Element::Ho }
        else if s == "111312211312" { Element::Dy }
        else if s == "3113112221131112" { Element::Tb }
        else if s == "13221133112" { Element::Gd }
        else if s == "1113222" { Element::Eu }
        else if s == "311332" { Element::Sm }
        else if s == "132" { Element::Pm }
        else if s == "111312" { Element::Nd }
        else if s == "31131112" { Element::Pr }
        else if s == "1321133112" { Element::Ce }
        else if s == "11131" { Element::La }
        else if s == "311311" { Element::Ba }
        else if s == "13211321" { Element::Cs }
        else if s == "11131221131211" { Element::Xe }
        else if s == "311311222113111221" { Element::I }
        else if s == "1322113312211" { Element::Te }
        else if s == "3112221" { Element::Sb }
        else if s == "13211" { Element::Sn }
        else if s == "11131221" { Element::In }
        else if s == "3113112211" { Element::Cd }
        else if s == "132113212221" { Element::Ag }
        else if s == "111312211312113211" { Element::Pd }
        else if s == "311311222113111221131221" { Element::Rh }
        else if s == "132211331222113112211" { Element::Ru }
        else if s == "311322113212221" { Element::Tc }
        else if s == "13211322211312113211" { Element::Mo }
        else if s == "1113122113322113111221131221" { Element::Nb }
        else if s == "12322211331222113112211" { Element::Zr }
        else if s == "1112133" { Element::Y }
        else if s == "3112112" { Element::Sr }
        else if s == "1321122112" { Element::Rb }
        else if s == "11131221222112" { Element::Kr }
        else if s == "3113112211322112" { Element::Br }
        else if s == "13211321222113222112" { Element::Se }
        else if s == "11131221131211322113322112" { Element::As }
        else if s == "31131122211311122113222" { Element::Ge }
        else if s == "13221133122211332" { Element::Ga }
        else if s == "312" { Element::Zn }
        else if s == "131112" { Element::Cu }
        else if s == "11133112" { Element::Ni }
        else if s == "32112" { Element::Co }
        else if s == "13122112" { Element::Fe }
        else if s == "111311222112" { Element::Mn }
        else if s == "31132" { Element::Cr }
        else if s == "13211312" { Element::V }
        else if s == "11131221131112" { Element::Ti }
        else if s == "3113112221133112" { Element::Sc }
        else if s == "12" { Element::Ca }
        else if s == "1112" { Element::K }
        else if s == "3112" { Element::Ar }
        else if s == "132112" { Element::Cl }
        else if s == "1113122112" { Element::S }
        else if s == "311311222112" { Element::P }
        else if s == "1322112" { Element::Si }
        else if s == "1113222112" { Element::Al }
        else if s == "3113322112" { Element::Mg }
        else if s == "123222112" { Element::Na }
        else if s == "111213322112" { Element::Ne }
        else if s == "31121123222112" { Element::F }
        else if s == "132112211213322112" { Element::O }
        else if s == "111312212221121123222112" { Element::N }
        else if s == "3113112211322112211213322112" { Element::C }
        else if s == "1321132122211322212221121123222112" { Element::B }
        else if s == "111312211312113221133211322112211213322112" { Element::Be }
        else if s == "312211322212221121123222112" { Element::Li }
        else if s == "13112221133211322112211213322112" { Element::He }
        else if s == "22" { Element::H }
        else {
            Element::Compound(s.to_string())
        };
        Ok(e)
    }
}

fn split(s : &str) -> Vec<Element>{
    let sp = split2(s);
    if sp.len() != 0 {
        return sp;
    }
    let sp = split3(s);
    if sp.len() != 0 {
        return sp;
    }
    let sp = split4(s);
    if sp.len() != 0 {
        return sp;
    }
    let sp = split5(s);
    if sp.len() != 0 {
        return sp;
    }
    let sp = split6(s);
    if sp.len() != 0 {
        return sp;
    }
    vec![s.parse().unwrap()]
}

fn split2(s : &str) -> Vec<Element>{
    for i in 1..s.len()-1 {
        let (p1, p2) = s.split_at(i);
        let p1 = p1.parse().unwrap();
        let p2 = p2.parse().unwrap();
        match p1 { Element::Compound(_) => continue, _ => {} }
        match p2 { Element::Compound(_) => continue, _ => {} }
        return vec![p1, p2];
    }
    Vec::new()
}

fn split3(s : &str) -> Vec<Element>{
    for i in 1..s.len()-2 {
        let (p1, p2) = s.split_at(i);
        for j in 1..p2.len()-1 {
            let (p2, p3) = p2.split_at(j);
            let p1 = p1.parse().unwrap();
            let p2 = p2.parse().unwrap();
            let p3 = p3.parse().unwrap();
            match p1 { Element::Compound(_) => continue, _ => {} }
            match p2 { Element::Compound(_) => continue, _ => {} }
            match p3 { Element::Compound(_) => continue, _ => {} }
            return vec![p1, p2, p3];
        }
    }
    Vec::new()
}

fn split4(s : &str) -> Vec<Element>{
    for i in 1..s.len()-3 {
        let (p1, p2) = s.split_at(i);
        for j in 1..p2.len()-2 {
            let (p2, p3) = p2.split_at(j);
            for k in 1..p3.len()-1 {
                let (p3, p4) = p3.split_at(k);
                let p1 = p1.parse().unwrap();
                let p2 = p2.parse().unwrap();
                let p3 = p3.parse().unwrap();
                let p4 = p4.parse().unwrap();
                match p1 { Element::Compound(_) => continue, _ => {} }
                match p2 { Element::Compound(_) => continue, _ => {} }
                match p3 { Element::Compound(_) => continue, _ => {} }
                match p4 { Element::Compound(_) => continue, _ => {} }
                return vec![p1, p2, p3, p4];
            }
        }
    }
    Vec::new()
}

fn split5(s : &str) -> Vec<Element>{
    for i in 1..s.len()-4 {
        let (p1, p2) = s.split_at(i);
        for j in 1..p2.len()-3 {
            let (p2, p3) = p2.split_at(j);
            for k in 1..p3.len()-2 {
                let (p3, p4) = p3.split_at(k);
                for l in 1..p4.len()-1 {
                    let (p4, p5) = p4.split_at(l);
                    let p1 = p1.parse().unwrap();
                    let p2 = p2.parse().unwrap();
                    let p3 = p3.parse().unwrap();
                    let p4 = p4.parse().unwrap();
                    let p5 = p5.parse().unwrap();
                    match p1 { Element::Compound(_) => continue, _ => {} }
                    match p2 { Element::Compound(_) => continue, _ => {} }
                    match p3 { Element::Compound(_) => continue, _ => {} }
                    match p4 { Element::Compound(_) => continue, _ => {} }
                    match p5 { Element::Compound(_) => continue, _ => {} }
                    return vec![p1, p2, p3, p4, p5];
                }
            }
        }
    }
    Vec::new()
}

fn split6(s : &str) -> Vec<Element>{
    for i in 1..s.len()-5 {
        let (p1, p2) = s.split_at(i);
        for j in 1..p2.len()-4 {
            let (p2, p3) = p2.split_at(j);
            for k in 1..p3.len()-3 {
                let (p3, p4) = p3.split_at(k);
                for l in 1..p4.len()-2 {
                    let (p4, p5) = p4.split_at(l);
                    for m in 1..p5.len()-2 {
                        let (p5, p6) = p5.split_at(m);
                        let p1 = p1.parse().unwrap();
                        let p2 = p2.parse().unwrap();
                        let p3 = p3.parse().unwrap();
                        let p4 = p4.parse().unwrap();
                        let p5 = p5.parse().unwrap();
                        let p6 = p6.parse().unwrap();
                        match p1 { Element::Compound(_) => continue, _ => {} }
                        match p2 { Element::Compound(_) => continue, _ => {} }
                        match p3 { Element::Compound(_) => continue, _ => {} }
                        match p4 { Element::Compound(_) => continue, _ => {} }
                        match p5 { Element::Compound(_) => continue, _ => {} }
                        match p6 { Element::Compound(_) => continue, _ => {} }
                        return vec![p1, p2, p3, p4, p5, p6];
                    }
                }
            }
        }
    }
    Vec::new()
}
