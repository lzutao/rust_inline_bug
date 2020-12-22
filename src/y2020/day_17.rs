
pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2020/17");

    let mut input = Vec::new();
    for states in INPUT.lines() {
        let mut row = Vec::new();
        for state in states.chars() {
            match state {
                '#' => row.push(true),
                '.' => row.push(false),
                _ => unreachable!(),
            }
        }
        input.push(row)
    }

    let mut cubes = vec![input.clone()];
    //print(&cubes, 0);
    for _i in 0..6 {
        cubes = step(&cubes);
        //print(&cubes, _i + 1);
    }

    let active = cubes
        .iter()
        .map(|p|p.iter().map(|r|{
            r.iter().filter(|a|**a).count()
        }).sum::<usize>())
        .sum::<usize>();

    println!("[PART 1] Active cubes {} after 6 steps", active);


    let mut hypercubes = vec![vec![input]];
    //print4d(&hypercubes, 0);
    for _i in 0..6 {
        hypercubes = step4d(&hypercubes);
        //print4d(&hypercubes, _i + 1);
    }

    let active = hypercubes
        .iter()
        .map(|hc| hc.iter()
            .map(|p|p.iter().map(|r|{
            r.iter().filter(|a|**a).count()
        }).sum::<usize>()).sum::<usize>())
        .sum::<usize>();

    println!("[PART 2] Active hypercubes {} after 6 steps", active);
}

#[allow(dead_code)]
fn print(state : &[Vec<Vec<bool>>], i : usize) {
    println!();
    println!("cycle: {}", i);
    for (z, p) in state.iter().enumerate() {
        println!();
        println!("z= {}", z as isize - i as isize);
        for r in p.iter() {
            for c in r.iter() {
                if *c {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn step(state : &[Vec<Vec<bool>>]) -> Vec<Vec<Vec<bool>>>{
    let mut next = Vec::with_capacity(state.len());
    for p in 0..state.len() + 2 {
        let pi = p > 0 && p < state.len() + 1;
        next.push(Vec::with_capacity(state[0].len()));
        for r in 0..state[0].len() + 2 {
            let ri = r > 0 && r < state[0].len() + 1;
            next[p].push(Vec::with_capacity(state[0][0].len()));
            for c in 0..state[0][0].len() + 2 {
                let ci = c > 0 && c < state[0][0].len() + 1;
                let was_alive = if pi && ri && ci {
                    state[p-1][r-1][c-1]
                }else {
                    false
                };
                let ip = p as isize - 1;
                let ir = r as isize - 1;
                let ic = c as isize - 1;
                next[p][r].push(alive(state, ip, ir, ic, was_alive));
            }
        }
    }

    let fp = next[0].iter().all(|r|r.iter().all(|a|!*a));
    let lp = next[next.len() - 1].iter().all(|r|r.iter().all(|a|!*a));
    let mut fr = true;
    let mut lr = true;
    let mut fc = true;
    let mut lc = true;
    for plane in &next {
        fr &= plane[0].iter().all(|a|!*a);
        lr &= plane[plane.len() - 1].iter().all(|a|!*a);
        for row in plane {
            fc &= !row[0];
            lc &= !row[row.len() - 1];
        }
    }
    if lp {
        next.remove(next.len() - 1);
    }
    if fp {
        next.remove(0);
    }
    if lr || fr || fc || lc{
        for plane in &mut next {
            if lr {
                plane.remove(plane.len() - 1);
            }
            if fr {
                plane.remove(0);
            }
            if fc || lc {
                for row in plane {
                    if lc {
                        row.remove(row.len() - 1);
                    }
                    if fc {
                        row.remove(0);
                    }
                }
            }
        }
    }

    next
}
fn alive(state : &[Vec<Vec<bool>>], p: isize, r: isize, c: isize, was_alive: bool) -> bool{
    let around = vec![
        get(state, p - 1, r - 1, c - 1),
        get(state, p - 1, r - 0, c - 1),
        get(state, p - 1, r + 1, c - 1),
        get(state, p - 1, r - 1, c - 0),
        get(state, p - 1, r - 0, c - 0),
        get(state, p - 1, r + 1, c - 0),
        get(state, p - 1, r - 1, c + 1),
        get(state, p - 1, r - 0, c + 1),
        get(state, p - 1, r + 1, c + 1),

        get(state, p - 0, r - 1, c - 1),
        get(state, p - 0, r - 0, c - 1),
        get(state, p - 0, r + 1, c - 1),
        get(state, p - 0, r - 1, c - 0),
        get(state, p - 0, r + 1, c - 0),
        get(state, p - 0, r - 1, c + 1),
        get(state, p - 0, r - 0, c + 1),
        get(state, p - 0, r + 1, c + 1),

        get(state, p + 1, r - 1, c - 1),
        get(state, p + 1, r - 0, c - 1),
        get(state, p + 1, r + 1, c - 1),
        get(state, p + 1, r - 1, c - 0),
        get(state, p + 1, r - 0, c - 0),
        get(state, p + 1, r + 1, c - 0),
        get(state, p + 1, r - 1, c + 1),
        get(state, p + 1, r - 0, c + 1),
        get(state, p + 1, r + 1, c + 1),
    ];
    let around = around.iter().filter(|a|**a).count();
    if was_alive {
        around == 2 || around == 3
    }else {
        around == 3
    }
}
fn get(state : &[Vec<Vec<bool>>], p: isize, r: isize, c: isize) -> bool{
    if (p as usize) < state.len()
        && (r as usize) < state[p as usize].len()
        && (c as usize) < state[p as usize][r as usize].len() {
        state[p as usize][r as usize][c as usize]
    } else {
        false
    }
}

#[allow(dead_code)]
fn print4d(state : &[Vec<Vec<Vec<bool>>>], i : usize) {
    println!();
    println!("cycle: {}", i);
    for (w, hc) in state.iter().enumerate() {
        for (z, p) in hc.iter().enumerate() {
            println!();
            println!("z= {}, w= {}", z as isize - i as isize, w as isize - i as isize);
            for r in p.iter() {
                for c in r.iter() {
                    if *c {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
        }
    }
}

fn step4d(state : &[Vec<Vec<Vec<bool>>>]) -> Vec<Vec<Vec<Vec<bool>>>>{
    let mut next = Vec::with_capacity(state.len());
    for w in 0..state.len() + 2 {
        let wi = w > 0 && w < state[0].len() + 1;
        next.push(Vec::with_capacity(state[0].len()));
        for p in 0..state[0].len() + 2 {
            let pi = p > 0 && p < state[0].len() + 1;
            next[w].push(Vec::with_capacity(state[0][0].len()));
            for r in 0..state[0][0].len() + 2 {
                let ri = r > 0 && r < state[0][0].len() + 1;
                next[w][p].push(Vec::with_capacity(state[0][0][0].len()));
                for c in 0..state[0][0][0].len() + 2 {
                    let ci = c > 0 && c < state[0][0][0].len() + 1;
                    let was_alive = if wi && pi && ri && ci {
                        state[w - 1][p - 1][r - 1][c - 1]
                    } else {
                        false
                    };
                    let iw = w as isize - 1;
                    let ip = p as isize - 1;
                    let ir = r as isize - 1;
                    let ic = c as isize - 1;
                    next[w][p][r].push(alive4d(state, iw, ip, ir, ic, was_alive));
                }
            }
        }
    }

    while next[0].iter()
        .all(|p| p.iter().all(|r|r.iter().all(|a|!*a))) {
        next.remove(0);
    }
    while next[next.len()-1].iter()
        .all(|p| p.iter().all(|r|r.iter().all(|a|!*a))) {
        next.remove(next.len()-1);
    }
    while next.iter()
        .all(|hc| hc[0].iter()
            .all(|r|r.iter().all(|a|!*a))) {
        for hc in &mut next {
            hc.remove(0);
        }
    }
    while next.iter()
        .all(|hc| hc[hc.len() - 1].iter()
            .all(|r|r.iter().all(|a|!*a))) {
        for hc in &mut next {
            hc.remove(hc.len() - 1);
        }
    }
    while next.iter()
        .all(|hc| hc.iter()
            .all(|p|p[0].iter().all(|a|!*a))) {
        for hc in &mut next {
            for p in hc {
                p.remove(0);
            }
        }
    }
    while next.iter()
        .all(|hc| hc.iter()
            .all(|p|p[p.len() - 1].iter().all(|a|!*a))) {
        for hc in &mut next {
            for p in hc {
                p.remove(p.len() - 1);
            }
        }
    }
    while next.iter()
        .all(|hc| hc.iter()
            .all(|p|p.iter().all(|r| !r[0]))) {
        for hc in &mut next {
            for p in hc {
                for r in p {
                    r.remove(0);
                }
            }
        }
    }
    while next.iter()
        .all(|hc| hc.iter()
            .all(|p|p.iter().all(|r| !r[r.len() - 1]))) {
        for hc in &mut next {
            for p in hc {
                for r in p {
                    r.remove(r.len() - 1);
                }
            }
        }
    }


    next
}
fn alive4d(state : &[Vec<Vec<Vec<bool>>>], w: isize, p: isize, r: isize, c: isize, was_alive: bool) -> bool{
    let mut around = 0;
    for iw in w-1..=w+1 {
        for ip in p-1..=p+1 {
            for ir in r-1..=r+1 {
                for ic in c-1..=c+1 {
                    if iw != w || ip != p || ir != r || ic != c {
                        if get4d(state, iw, ip, ir, ic) {
                            around += 1;
                        }
                    }
                }
            }
        }
    }

    if was_alive {
        around == 2 || around == 3
    }else {
        around == 3
    }
}
fn get4d(state : &[Vec<Vec<Vec<bool>>>], w: isize, p: isize, r: isize, c: isize) -> bool{
    if (w as usize) < state.len()
        && (p as usize) < state[w as usize].len()
        && (r as usize) < state[w as usize][p as usize].len()
        && (c as usize) < state[w as usize][p as usize][r as usize].len() {
        state[w as usize][p as usize][r as usize][c as usize]
    } else {
        false
    }
}
