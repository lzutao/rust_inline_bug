
struct Ingredient{
    pub capacity : isize,
    pub durability : isize,
    pub flavor : isize,
    pub texture : isize,
    pub calories : isize,
}

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2015/15");
    let ingredients : Vec<Ingredient> = INPUT
        .lines()
        .map(|l| {
            let p : Vec<&str> = l.split(" ").collect();
            Ingredient{
                capacity: p[2].split(",").next().unwrap().parse().unwrap(),
                durability: p[4].split(",").next().unwrap().parse().unwrap(),
                flavor: p[6].split(",").next().unwrap().parse().unwrap(),
                texture: p[8].split(",").next().unwrap().parse().unwrap(),
                calories: p[10].parse().unwrap(),
            }
        })
        .collect();
    const MAX_SPOONS : isize = 100;
    let none = Ingredient{capacity:0, durability: 0, flavor: 0, texture: 0, calories: 0};
    let best = brute_force_cook(&ingredients, MAX_SPOONS,
                                none, None);

    println!("[PART 1] Best possible score is {}", best);
    let none = Ingredient{capacity:0, durability: 0, flavor: 0, texture: 0, calories: 0};
    let best = brute_force_cook(&ingredients, MAX_SPOONS,
                                none, Some(500));

    println!("[PART 2] Best possible score with 500 calories is {}", best);
}

fn brute_force_cook(ingredients: &[Ingredient], max_spoons: isize, current: Ingredient,
                required_calories: Option<isize>) -> isize{
    if ingredients.len() == 1 {
        let i = Ingredient{
            capacity: current.capacity + ingredients[0].capacity * max_spoons,
            durability: current.durability + ingredients[0].durability * max_spoons,
            flavor: current.flavor + ingredients[0].flavor * max_spoons,
            texture: current.texture + ingredients[0].texture * max_spoons,
            calories: current.calories + ingredients[0].calories * max_spoons,
        };
        if i.capacity <= 0 || i.durability <= 0 || i.flavor <= 0 || i.texture <= 0 {
            return 0;
        }
        match required_calories {
            None => {}
            Some(c) => {
                if i.calories != c {
                    return 0;
                }
            }
        }
        return i.capacity * i.durability * i.flavor * i.texture;
    }
    let mut best = 0;
    for s in 0..max_spoons {
        let i = Ingredient{
            capacity: current.capacity + ingredients[0].capacity * s,
            durability: current.durability + ingredients[0].durability * s,
            flavor: current.flavor + ingredients[0].flavor * s,
            texture: current.texture + ingredients[0].texture * s,
            calories: current.calories + ingredients[0].calories * s,
        };
        let b = brute_force_cook(
            &ingredients[1..], max_spoons - s, i, required_calories);
        if b > best {
            best = b;
        }
    }

    best
}