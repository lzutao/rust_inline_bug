
pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2015/17");
    let mut sizes : Vec<usize> = INPUT
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();

    sizes.sort_by(|a, b| b.cmp(a));

    const TARGET : usize = 150;

    let fits = count_fits(&sizes, 0, TARGET);

    println!("[PART 1] Combinations of buckets that fits {}", fits);

    let min_containers = min_fits(&sizes, 0, 0, TARGET);
    let min_containers = count_fits_for(&sizes, 0,
                                        0, TARGET, min_containers);

    println!("[PART 2] Combinations using least containers {}", min_containers);
}

fn count_fits(sizes:&[usize], current: usize, target: usize) -> usize{
    let mut fits = 0;
    for (i, size) in sizes.iter().enumerate() {
        if current + size == target {
            fits += 1;
            continue;
        }
        fits += count_fits(&sizes[i+1..], current + size, target);
    }
    fits
}

fn count_fits_for(sizes:&[usize], current: usize, current_containers: usize,
                  target: usize, target_containers: usize) -> usize{
    let mut fits = 0;
    if current_containers >= target_containers {
        return 0;
    }
    for (i, size) in sizes.iter().enumerate() {
        if current + size == target {
            fits += 1;
            continue;
        }
        fits += count_fits_for(&sizes[i+1..], current + size,
                               current_containers +1, target, target_containers);
    }
    fits
}

fn min_fits(sizes:&[usize], current: usize, current_containers: usize, target: usize) -> usize {
    let mut min = usize::MAX;
    for (i, size) in sizes.iter().enumerate() {
        if current + size == target {
            return current_containers + 1;
        }
        let c = min_fits(&sizes[i+1..], current + size,
                         current_containers + 1, target);
        if c < min {
            min = c;
        }
    }
    min
}