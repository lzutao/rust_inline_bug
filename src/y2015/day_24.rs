
pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2015/24");
    let mut weights : Vec<usize> = INPUT
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
    weights.sort_by(|a, b| b.cmp(a));

    let g1 = find_best_first_group(&weights, 3);
    let qe = g1.iter().product::<usize>();

    println!("[PART 1] Quantum entanglement of the first group: {}", qe);

    let g1 = find_best_first_group(&weights, 4);
    let qe = g1.iter().product::<usize>();

    println!("[PART 2] Quantum entanglement of the first group: {}", qe);
}

fn find_best_first_group(weights: &[usize], groups: usize) -> Vec<usize> {
    let target_weight = weights.iter().sum::<usize>() / groups;
    let mut best_size = usize::MAX;
    let g1s = find_smallest_group(weights, target_weight,
                                 0, Vec::new(), 0,
                                  &mut best_size);
    let mut best_qe = usize::MAX;
    let mut best_g1 = Vec::new();
    for g1 in g1s {
        let g1 = weights
            .iter()
            .enumerate()
            .filter(|(i, _v)| g1.contains(i))
            .map(|(_i, v)|*v)
            .collect::<Vec<usize>>();
        let qe = g1.iter().product::<usize>();
        if qe < best_qe {
            best_qe = qe;
            best_g1 = g1;
        }
    }


    best_g1
}

fn find_smallest_group(weights: &[usize], target_weight: usize,
                       current_weight: usize, indexes: Vec<usize>,
                       current: usize, best_size: &mut usize) -> Vec<Vec<usize> >{
    let mut results = Vec::new();
    for i in current..weights.len() {
        let sum = current_weight + weights[i];
        if sum > target_weight {
            continue
        }
        let mut indexes = indexes.clone();
        indexes.push(i);
        if sum == target_weight {
            *best_size = indexes.len();
            results.push(indexes);
            continue
        }
        if indexes.len() == *best_size {
            continue;
        }
        results.extend_from_slice(
            &find_smallest_group(weights, target_weight, sum,
                                 indexes, i + 1, best_size));
    }
    results
}
