
pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2015/2");
    let sizes : Vec<Vec<usize>> = INPUT
        .lines()
        .map(|l| l
            .split("x")
            .map(|d| d.parse::<usize>().unwrap())
            .collect())
        .collect();
    let mut area = 0;
    let mut length = 0;
    for (a, l) in sizes.iter().map(required_area) {
        area += a;
        length += l;
    }
    println!("[PART 1] Required area {}", area);
    println!("[PART 2] Required length {}", length);
}

fn required_area(sizes: &Vec<usize>) -> (usize, usize){
    let l = sizes[0];
    let w = sizes[1];
    let h = sizes[2];
    let s1 = l * w;
    let s2 = w * h;
    let s3 = h * l;
    let extra = s1.min(s2.min(s3));
    let r = if l < w {
        l + w.min(h)
    }else {
        w + l.min(h)
    };
    (2 * (s1 + s2 + s3) + extra, 2 * r + l * w * h)
}