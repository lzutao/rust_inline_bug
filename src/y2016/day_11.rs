use std::collections::BTreeSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fmt;

pub use self::Thing::*;

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2016/11");

    println!("WARNING! this is kind of very slow :(");
    let floors : Vec<BTreeSet<Thing>> = INPUT
        .lines()
        .map(|l|{
            let mut s = l.split(" ");
            s.next().unwrap(); // The
            s.next().unwrap(); // X-th
            s.next().unwrap(); // floor
            s.next().unwrap(); // contains
            s.next().unwrap(); // a / nothing
            let mut objects = BTreeSet::new();
            loop {
                let o = s.next().unwrap();
                if o == "relevant." {
                    break
                }
                let t = s.next().unwrap();
                let dot = t.as_bytes()[t.len()-1] as char == '.';
                let t = t.as_bytes()[0] as char;
                if t == 'm' {
                    objects.insert(Microchip(o.split_at(1).0));
                } else if t == 'g' {
                    objects.insert(Generator(o.split_at(1).0));
                }
                if dot {
                    break
                }
                let a = s.next().unwrap(); // a / and
                if a == "and" {
                    s.next().unwrap(); // a
                }
            }
            objects
        })
        .collect();
    
    let mut state = State::new(floors);
    println!("[PART 1] Minimum number of steps: {}", state.min_steps());
    state.floors[0].insert(Generator("El"));
    state.floors[0].insert(Microchip("El"));
    state.floors[0].insert(Generator("Di"));
    state.floors[0].insert(Microchip("Di"));
    println!("[PART 2] Minimum number of steps: {}", state.min_steps());
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub enum Thing {
    Generator(&'static str),
    Microchip(&'static str),
}


#[derive(PartialEq, Eq, Clone)]
pub struct State {
    floors: Vec<BTreeSet<Thing>>,
    elevator: usize,
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("{")?;
        for (floor, things) in self.floors.iter().enumerate() {
            if self.elevator == floor {
                f.write_str("*")?
            }
            floor.fmt(f)?;
            f.write_str(":")?;
            for (i, thing) in things.iter().enumerate() {
                if i > 0 {
                    f.write_str(",")?
                }
                f.write_fmt(format_args!("{:?}", thing))?;
            }
            f.write_str(" ")?;
        }
        f.write_str("}")?;
        Ok(())
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (floor, things) in self.floors.iter().enumerate() {
            floor.hash(state);
            let mut things: Vec<Thing> = things.iter().cloned().collect();
            things.sort();
            things.hash(state);
        }
        self.elevator.hash(state);
    }
}

impl State {
    fn new(floors: Vec<BTreeSet<Thing>>) -> State {
        State { floors, elevator: 0 }
    }

    fn is_valid(&self) -> bool {
        // on each floor above first..
        for things in &self.floors[1..] {
            for thing in things {
                // if there is a microchip..
                if let &Microchip(e) = thing {
                    // without protection (a matching generator)..
                    if !things.iter().any(|t| t == &Generator(e)) &&
                        // exposed to a (non-matching) generator..
                        things.iter().any(|t| match t { &Generator(ee) if ee != e => true, _ => false })
                    {
                        // ..it'll fry (making this state invalid)
                        return false;
                    }
                }
            }
        }
        // valid only, if elevator position is on an existing floor
        self.elevator < self.floors.len()
    }

    fn is_done(&self) -> bool {
        for things in &self.floors[..self.floors.len()-1] {
            if things.len() > 0 {
                return false;
            }
        }
        true
    }

    fn next_states<'a>(&'a self) -> impl Iterator<Item=State> + 'a
    {
        let floor = &self.floors[self.elevator];
        [false, true]
            .iter()
            .filter_map(move |&down|{
                let state = self.clone();
                if down && state.elevator == 0
                    || !down && state.elevator + 1 >= state.floors.len() { return None; }
                let new_elevator = if down { state.elevator - 1 } else { state.elevator + 1 };
                let next_states = floor
                    .iter()
                    .flat_map(move |t1| {
                        [None].iter()
                            .cloned()
                            .chain(floor.iter().clone().map(|t| Some(t.clone())) )
                            .map(move|t2|(*t1, t2))
                    })
                    .filter_map(move |(thing1, thing2)|{
                        let mut new_floors = state.floors.clone();
                        new_floors[state.elevator].remove(&thing1);
                        new_floors[new_elevator].insert(thing1.clone());
                        if let Some(thing2) = thing2 {
                            new_floors[state.elevator].remove(&thing2);
                            new_floors[new_elevator].insert(thing2.clone());
                        }
                        let new_state = State { floors: new_floors, elevator: new_elevator };
                        if new_state.is_valid() {
                            Some(new_state)
                        }else {
                            None
                        }
                    });
                Some(next_states)
            })
            .flatten()
    }

    fn min_steps(&self) -> usize {
        let mut states = vec![self.clone()];
        let mut depth = 1;
        let mut seen = BTreeSet::new();
        loop {
            let mut new_states = Vec::new();
            let mut done = false;
            for state in &states {
                for new_state in state.next_states() {
                    if !done {
                        if new_state.is_done() {
                            done = true;
                        }
                        let mut hasher = DefaultHasher::new();
                        new_state.hash(&mut hasher);
                        let new_state_hash = hasher.finish();
                        if !seen.contains(&new_state_hash) {
                            seen.insert(new_state_hash);
                            new_states.push(new_state);
                        }
                    }
                }
            }
            if done { return depth; }
            states = new_states;
            depth += 1;
        }
    }
}
