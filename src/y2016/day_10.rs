
#[derive(Copy, Clone, Debug)]
enum T{
    Bot(usize),
    Output(usize),
}

impl PartialEq<usize> for T {
    fn eq(&self, other: &usize) -> bool {
        match self {
            T::Bot(id) => id.eq(other),
            T::Output(_) => false,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Bot{
    pub id : usize,
    pub low : Option<usize>,
    pub low_target : T,
    pub high : Option<usize>,
    pub high_target : T,
    pub sent: bool,
}
impl Bot{
    pub fn add_value(&mut self, value: usize) {
        assert!(self.high.is_none());
        if self.low.is_none() {
            self.low = Some(value);
        }else if self.low.unwrap() < value {
            self.high = Some(value);
        }else {
            std::mem::swap(&mut self.low, &mut self.high);
            self.low = Some(value);
        }
        if self.low == Some(17) && self.high == Some(61) {
            println!("[PART 1] Both handling 61 and 17 is {}", self.id);
        }
    }
    pub fn is_final(&self) -> bool {
        self.low.is_some() && self.high.is_some()
    }
}

#[derive(Copy, Clone)]
enum BV {
    Bot(Bot),
    Value(usize, usize)
}

impl BV {
    pub fn is_final(&self) -> bool {
        match self {
            BV::Bot(b) => b.is_final(),
            BV::Value(_, _) => true,
        }
    }
}

pub fn solve(){
    const INPUT : &str = include_str!("../../inputs/2016/10");
    let mut bots : Vec<BV> = INPUT
        .lines()
        .map(|l| {
            let mut s = l.split(" ");
            let bv = s.next().unwrap();
            if bv == "bot" {
                let b = s.next().unwrap().parse().unwrap();
                let mut s = s.skip(3);
                let lt = s.next().unwrap();
                let l = s.next().unwrap().parse().unwrap();
                let lt = if lt == "bot" {
                    T::Bot(l)
                } else {
                    T::Output(l)
                };
                let mut s = s.skip(3);
                let ht = s.next().unwrap();
                let h = s.next().unwrap().parse().unwrap();
                let ht = if ht == "bot" {
                    T::Bot(h)
                } else {
                    T::Output(h)
                };

                BV::Bot(Bot{ id:b, low:None, low_target:lt,
                    high:None, high_target:ht, sent: false })
            }else {
                let v = s.next().unwrap().parse().unwrap();
                let mut s = s.skip(3);
                let b = s.next().unwrap().parse().unwrap();
                BV::Value(v, b)
            }
        })
        .collect();

    for i in 0..bots.len() {
        let (v, t) = match bots[i] {
            BV::Bot(_) => continue,
            BV::Value(v, t) => (v, t)
        };
        for j in 0..bots.len() {
            match &mut bots[j] {
                BV::Bot(b) => {
                    if b.id != t {
                        continue;
                    }
                    b.add_value(v)
                },
                BV::Value(_, _) => continue,
            };
        }
    }

    loop {
        for i in 0..bots.len() {
            let sb = match bots[i] {
                BV::Bot(b) => {
                    if b.sent || !b.is_final(){
                        continue
                    }
                    b
                }
                BV::Value(_, _) => continue,
            };
            for j in 0..bots.len() {
                if bots[j].is_final() {
                    continue;
                }
                match &mut bots[j] {
                    BV::Bot(b) => {
                        if sb.low_target == b.id {
                            b.add_value(sb.low.unwrap())
                        }
                        if sb.high_target == b.id {
                            b.add_value(sb.high.unwrap())
                        }
                    }
                    BV::Value(_, _) => unreachable!()
                }
            }
            match &mut bots[i] {
                BV::Bot(b) => b.sent = true,
                BV::Value(_, _) => unreachable!(),
            }
        }
        if bots.iter().all(|b| b.is_final()) {
            break;
        }
    }

    let mut results = 1;

    for i in 0..bots.len() {
        match bots[i] {
            BV::Bot(b) => {
                match b.low_target {
                    T::Bot(_) => {}
                    T::Output(o) => {
                        if o < 3 {
                            results *= b.low.unwrap()
                        }
                    }
                }
                match b.high_target {
                    T::Bot(_) => {}
                    T::Output(o) => {
                        if o < 3 {
                            results *= b.high.unwrap()
                        }
                    }
                }
            },
            BV::Value(_, _) => continue,
        }
    }


    println!("[PART 2] Results: {}", results);
}
