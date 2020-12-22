
pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2015/9");
    let edges : Vec<(&str, &str, usize)> = INPUT
        .lines()
        .map(get_edge)
        .collect();

    let mut shortest = usize::MAX;
    for edge in edges.iter() {
        let mut visited = Vec::new();
        visited.push(edge.0);
        visited.push(edge.1);
        let short = find_shortest(&edges, &mut visited, edge.2, shortest);
        if short < shortest {
            shortest = short;
        }
    }

    println!("[PART 1] Shortest path {}", shortest);

    let mut longest = usize::MIN;
    for edge in edges.iter() {
        let mut visited = Vec::new();
        visited.push(edge.0);
        visited.push(edge.1);
        let long = find_longest(&edges, &mut visited, edge.2, longest);
        if long > longest {
            longest = long;
        }
    }
    println!("[PART 2] Longest path {}", longest);
}

fn get_edge(line : &str) -> (&str, &str, usize){
    let parts : Vec<&str> = line.split(" ").collect();
    (parts[0], parts[2], parts[4].parse().unwrap())
}

fn find_shortest(edges : &[(&str, &str, usize)], visited : &mut Vec<&str>,
                 current_distance : usize, mut shortest : usize) -> usize{
    let current = *visited.last().unwrap();
    let possibilities : Vec<&(&str, &str, usize)> = edges
        .iter()
        .filter(|(f, t, _d)|
            *f == current && !visited.contains(t)
                || *t == current && !visited.contains(f))
        .collect();
    if possibilities.is_empty() {
        return current_distance;
    }
    for (f, t, d) in possibilities {
        let t = if *f == current {
            t
        }else {
            f
        };
        if current_distance + d > shortest {
            continue
        }
        let mut visited = visited.clone();
        visited.push(t);
        let short = find_shortest(edges, &mut visited,
                                  current_distance + d, shortest);
        if short < shortest {
            shortest = short;
        }
    }
    shortest
}

fn find_longest(edges : &[(&str, &str, usize)], visited : &mut Vec<&str>,
                 current_distance : usize, mut longest : usize) -> usize{
    let current = *visited.last().unwrap();
    let possibilities : Vec<&(&str, &str, usize)> = edges
        .iter()
        .filter(|(f, t, _d)|
            *f == current && !visited.contains(t)
                || *t == current && !visited.contains(f))
        .collect();
    if possibilities.is_empty() {
        return current_distance;
    }
    for (f, t, d) in possibilities {
        let t = if *f == current {
            t
        }else {
            f
        };
        let mut visited = visited.clone();
        visited.push(t);
        let long = find_longest(edges, &mut visited,
                                  current_distance + d, longest);
        if long > longest {
            longest = long;
        }
    }
    longest
}