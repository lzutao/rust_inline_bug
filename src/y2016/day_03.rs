
pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2016/3");

    let triangles : Vec<(usize, usize, usize)> = INPUT
        .lines()
        .map(|l| {
            let mut t = l.split(" ")
                .filter(|t| !t.is_empty())
                .map(|s| s.parse().unwrap());
            (t.next().unwrap(), t.next().unwrap(), t.next().unwrap())
        })
        .collect();

    let valid = triangles
        .iter()
        .filter(|(a, b, c)| a + b > *c && a + c > *b && b + c > *a)
        .count();

    println!("[PART 1] Valid triangles {}", valid);

    let mut i = 0;
    let mut valid = 0;
    while i < triangles.len() {
        let (a1, a2, a3) = triangles[i + 0];
        let (b1, b2, b3) = triangles[i + 1];
        let (c1, c2, c3) = triangles[i + 2];
        if a1 + b1 > c1 && a1 + c1 > b1 && b1 + c1 > a1 {
            valid += 1;
        }
        if a2 + b2 > c2 && a2 + c2 > b2 && b2 + c2 > a2 {
            valid += 1;
        }
        if a3 + b3 > c3 && a3 + c3 > b3 && b3 + c3 > a3 {
            valid += 1;
        }
        i += 3;
    }

    println!("[PART 2] Valid triangles {}", valid);
}
