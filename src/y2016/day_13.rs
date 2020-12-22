use std::collections::BTreeMap;

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2016/13");
    let fav = INPUT.parse::<usize>().unwrap();

    /*for y in 0..50 {
        for x in 0..50 {
            let v = x*x + 3*x + 2*x*y + y + y*y + fav;
            let v = if v.count_ones() & 1 == 0 {'.'} else {'#'};
            print!("{}", v);
        }
        println!();
    }*/

    let shortest = find_shortest_path(1, 1, fav, 31, 39);

    println!("[PART 1] Shortest path is {}", shortest);

    let reachable = reachable_in_50_steps(1, 1, fav);

    println!("[PART 2] You can reach {} locations in 50 steps", reachable);
}

#[allow(dead_code)]
fn print(fav : usize, mark: &BTreeMap<Pos, usize>){
    for y in 0..25 {
        for x in 0..28 {
            let v = x*x + 3*x + 2*x*y + y + y*y + fav;
            let v = if mark.contains_key(&Pos{x, y}) {'o'} else {
                if v.count_ones() & 1 == 0 {'.'} else {'#'}
            };
            print!("{}", v);
        }
        println!();
    }
}

fn is_wall(x : usize, y : usize, fav : usize) -> bool {
    let v = x*x + 3*x + 2*x*y + y + y*y + fav;
    v.count_ones() & 1 == 1
}

fn find_shortest_path(x : usize, y : usize, fav : usize,
                      tx : usize, ty : usize) -> usize {
    let mut explored = BTreeMap::new();
    let mut paths = Vec::new();
    let target = Pos{x: tx, y : ty};
    paths.push(Path::new(x, y, 0, &target));
    loop {
        paths.sort_by(|p1, p2| p2.distance_hint.cmp(&p1.distance_hint));
        let p = paths.remove(paths.len() - 1);
        //println!("{} {} {} {}", p.distance_hint, p.distance, p.pos.x, p.pos.y);
        explored.insert(p.pos, p.distance);
        let next = p.next(fav, &target);
        for n in next.iter() {
            if let Some(n) = n {
                if n.distance_hint == n.distance {
                    return n.distance;
                } else {
                    match explored.get(&n.pos) {
                        None => paths.push(*n),
                        Some(e) => {
                            if *e > n.distance {
                                paths.push(*n);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn reachable_in_50_steps(x : usize, y : usize, fav : usize) -> usize {
    let mut explored = BTreeMap::new();
    let mut paths = Vec::new();
    let target = Pos{x, y};
    paths.push(Path::new(x, y, 0, &target));
    loop {
        if paths.len() == 0 {
            //print(fav, &explored);
            return explored.len();
        }
        let p = paths.remove(paths.len() - 1);
        explored.insert(p.pos, p.distance);
        let next = p.next(fav, &target);
        for n in next.iter() {
            if let Some(n) = n {
                if n.distance > 50 {
                    continue
                } else {
                    match explored.get(&n.pos) {
                        None => paths.push(*n),
                        Some(e) => {
                            if *e > n.distance {
                                paths.push(*n);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Pos {
    pub x : usize,
    pub y : usize,
}
impl Pos {
    pub fn dist(&self, target : &Pos) -> usize {
        self.x.max(target.x) - self.x.min(target.x)
            + self.y.max(target.y) - self.y.min(target.y)
    }
}

#[derive(Copy, Clone)]
struct Path{
    pub pos : Pos,
    pub distance_hint : usize,
    pub distance : usize,
}
impl Path {
    pub fn new(x : usize, y : usize, distance : usize, target: &Pos) -> Path {
        let pos = Pos{x, y};
        let distance_hint = distance + pos.dist(target);
        Path{ pos, distance_hint, distance }
    }
    pub fn next(self, fav : usize, target: &Pos) -> [Option<Path>;4] {
        let a = if self.pos.y == 0 || is_wall(self.pos.x, self.pos.y - 1, fav) {
            None
        }else {
            Some(Path::new(self.pos.x, self.pos.y - 1, self.distance + 1, target))
        };
        let b = if self.pos.x == 0 || is_wall(self.pos.x - 1, self.pos.y, fav) {
            None
        }else {
            Some(Path::new(self.pos.x - 1, self.pos.y, self.distance + 1, target))
        };
        let c = if is_wall(self.pos.x, self.pos.y + 1, fav) {
            None
        }else {
            Some(Path::new(self.pos.x, self.pos.y + 1, self.distance + 1, target))
        };
        let d = if is_wall(self.pos.x + 1, self.pos.y, fav) {
            None
        }else {
            Some(Path::new(self.pos.x + 1, self.pos.y, self.distance + 1, target))
        };

        [a,b,c,d]
    }
}
