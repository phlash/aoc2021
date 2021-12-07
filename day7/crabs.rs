use std::io::{self,BufRead};

fn main() {
    // hmmn: https://stackoverflow.com/questions/28528998/how-do-i-read-a-single-string-from-standard-input
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let vals : Vec<&str> = line.split(',').collect();
    let crabs : Vec<usize> = vals.iter().map(|v| usize::from_str_radix(v,10).unwrap()).collect();
    // find maximum..
    let mut max : usize = 0;
    for c in 0..crabs.len() { if crabs[c]>max { max=crabs[c]; } }
    println!("max: {}", max);
    // generate cost function map
    let mut costs : Vec<usize> = Vec::new();
    for p in 0..max+1 {
        let mut c : usize = 0;
        for d in 0..p+1 { c += d; }
        costs.push(c);
    }
    // cost each position to find minimum..
    let mut min : usize = usize::MAX;
    let mut mps : usize = 0;
    for p in 0..max {
        let mut cost : usize = 0;
        for c in 0..crabs.len() {
            let v = crabs[c];
            cost += if v<p { costs[p-v] } else { costs[v-p] };
        }
        println!("pos:{} cost:{}", p, cost);
        if cost<min { min=cost; mps=p; }
    }
    println!("min: {} @ {}", min, mps);
}