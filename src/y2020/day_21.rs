use std::collections::BTreeMap;

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2020/21");

    let foods = INPUT
        .lines()
        .map(|l|{
            let mut pats = l.split(" (contains ");
            let ingredients = pats
                .next()
                .unwrap()
                .split(" ")
                .collect();
            let allergens = pats.next().unwrap();
            let allergens = allergens.split_at(allergens.len() - 1).0;
            let allergens = allergens.split(", ").collect();
            (ingredients, allergens)
        })
        .collect::<Vec<(Vec<&'static str>,Vec<&'static str>)>>();
    //println!("{:#?}", foods);
    let mut possible_allergens_in = BTreeMap::new();
    for (i, a) in &foods {
        for &a in a {
            match possible_allergens_in.get_mut(a) {
                None => {possible_allergens_in.insert(a, i.clone());}
                Some(pi) => {
                    *pi = pi
                        .iter()
                        .filter(|pi| i.contains(pi))
                        .map(|pi| *pi)
                        .collect();
                }
            }
        }
    }
    let mut allergens_in = BTreeMap::new();
    loop {
        if possible_allergens_in.is_empty() {
            break;
        }
        let confirmed = possible_allergens_in
            .iter()
            .filter_map(|(a, i)| {
                if i.len() == 1 {
                    Some((*a, i[0]))
                }else {
                    None
                }
            })
            .next().unwrap();
        possible_allergens_in.remove(&confirmed.0);
        for (_a, i) in &mut possible_allergens_in {
            let mut remove = None;
            for (index, i) in i.iter().enumerate() {
                if *i == confirmed.1 {
                    remove = Some(index);
                    break;
                }
            }
            match remove {
                None => {}
                Some(r) => {
                    i.remove(r);
                }
            }
        }
        allergens_in.insert(confirmed.0, confirmed.1);
    }
    let ingredients_with_allergens = allergens_in
        .values()
        .map(|&v|v)
        .collect::<Vec<&str>>();
    let non_allergic_count = foods
        .iter()
        .map(|(i, _)| i.iter()
            .filter(|i| !ingredients_with_allergens.contains(i))
            .count())
        .sum::<usize>();//.sum::<usize>();

    println!("[PART 1] Non allergic ingredients {}", non_allergic_count);

    let mut allergens_in = allergens_in
        .iter()
        .collect::<Vec<(&&str, &&str)>>();
    allergens_in.sort_by(|(a1, _), (a2, _)| a1.cmp(a2));
    let dangerous_list = allergens_in
        .iter()
        .map(|(_, i)| *i)
        .fold(String::new(), | a, b| a + b + ",");
    let dangerous_list = dangerous_list.split_at(dangerous_list.len() - 1).0;

    println!("[PART 2] Dangerous list '{}'", dangerous_list);
}
