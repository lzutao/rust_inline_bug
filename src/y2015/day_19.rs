use std::collections::BTreeSet;

const FIRST_GUESS : bool = false;

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2015/19");

    let mut lines = INPUT
        .lines()
        .rev();

    let medicine = lines.next().unwrap();
    lines.next().unwrap();
    let mut lines = lines.collect::<Vec<&str>>();
    lines.sort_by(|l1, l2| l2.len().cmp(&l1.len()));
    let replacements : Vec<(&str, &str)> = lines
        .iter()
        .map(|r| {
            let mut r = r.split(" => ");
            (r.next().unwrap(), r.next().unwrap())
        })
        .collect();

    let possibilities = count_possibilities(medicine, &replacements);

    println!("[PART 1] Can create up to {} with single steps", possibilities);

    if !FIRST_GUESS {
        println!("WARNING! this is very slow :(");
    }
    let required_steps = build_molecule("e", medicine,
                    &replacements, 0, usize::MAX);

    println!("[PART 2] To create medicine {} steps is required", required_steps);
}

fn count_possibilities(start_molecule: &str, replacements: &[(&str, &str)]) -> usize{
    let mut possibilities = BTreeSet::new();

    for (f, t) in replacements {
        let start_molecule = start_molecule.to_string();
        for (m, _) in start_molecule.match_indices(*f) {
            let mut new_molecule = start_molecule.clone();
            new_molecule.replace_range(m..m+f.len(), t);
            possibilities.insert(new_molecule);
        }
    }

    possibilities.len()
}

fn build_molecule(start_molecule: &str, target_molecule: &str,
                  replacements: &[(&str, &str)], step: usize, mut known_min: usize) -> usize{
    for (f, t) in replacements {
        let target_molecule = target_molecule.to_string();
        let try_step = step + target_molecule.match_indices(*t).count();
        if try_step == step || try_step >= known_min{
            continue;
        }
        let prev_molecule = target_molecule.clone().replace(t, f);
        if prev_molecule == start_molecule {
            return try_step;
        }
        let m = build_molecule(start_molecule, &prev_molecule,
                               replacements, try_step, known_min);
        if m < known_min {
            known_min = m;
            if FIRST_GUESS{
                println!("guessing! {}", m);
                // Not works for all inputs, but thanks to sorting it should work for most
                return known_min;
            }
        }
    }

    known_min
}