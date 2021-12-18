// I failed to get anywhere all day with this, so took inspiration from Reddit megathread and
// https://github.com/InfinityByTen/AoC-2021/blob/main/day18/src/main.rs
use std::io::{self,BufRead};

#[derive(Debug, Clone)]
enum SnailNum {
    Leaf(usize),
    Pair { left : Box<SnailNum>, right : Box<SnailNum> },
}

fn parse(txt : &str) -> SnailNum {
    let mut mid : usize = 0;
    let mut depth : usize = 0;
    for (i,c) in txt.chars().enumerate() {
        match c {
            // digits are a SnailNum all by themselves..
            '0'..='9' => { if 0==depth { return SnailNum::Leaf(usize::from_str_radix(&c.to_string(),10).unwrap()); } },
            // more complex forms we find the separator..
            '[' => { depth += 1; },
            ']' => { depth -= 1; },
            ',' => { if 1==depth { mid = i; } }
            _ => {},
        }
    }
    // split at separator and recursively parse halves
    return SnailNum::Pair {
        left : Box::new(parse(&txt[1..mid])),
        right: Box::new(parse(&txt[mid+1..txt.len()-1])),
    };
}

fn add(a : &mut SnailNum, b : &SnailNum) {
    *a = SnailNum::Pair {
        left  : Box::new(a.clone()),
        right : Box::new(b.clone()),
    };
}

fn distribute(n : &mut SnailNum, v : usize, kl : bool) {
    match n {
        // regular number, add that value and stop
        SnailNum::Leaf(p) => { *p += v; },
        // pair, recurse down, keeping left or right
        SnailNum::Pair{ left: l, right: r } => {
            if kl { distribute(l, v, kl); }
            else  { distribute(r, v, kl); }
        },
    }
}

fn explode(n : &mut SnailNum, depth : usize) -> Option<(usize,usize)> {
    // walk the tree, if we reach depth>=4, explode..
    match n {
        SnailNum::Leaf(_) => {
            // cannot explode at a raw number
            return None;
        },
        SnailNum::Pair { left : l, right: r } => {
            // explode depth?
            if depth>=4 {
                //println!("explode@ [{:?},{:?}]", *l, *r);
                // parse leaf values, replace exploded pair..
                let vl = match **l {
                    SnailNum::Leaf(v) => { v },
                    _ => unreachable!(),
                };
                let vr = match **r {
                    SnailNum::Leaf(v) => { v },
                    _ => unreachable!(),
                };
                *n = SnailNum::Leaf(0);
                return Some((vl,vr));
            }
            if let Some((vl,vr)) = explode(l, depth+1) {
                // left side of pair exploded, distribute rightwards value, to leftmost digit to the right..
                distribute(r, vr, true);
                // return leftwards value for higher layer to process
                return Some((vl,0));
            }
            if let Some((vl,vr)) = explode(r, depth+1) {
                // as above but the other way..
                distribute(l, vl, false);
                return Some((0,vr));
            }
        },
    }
    return None;
}

fn split(n : &mut SnailNum) -> Option<()> {
    match n {
        SnailNum::Leaf(val) => {
            // needs a split?
            if *val>9 {
                //println!("split@ {}", *val);
                *n = SnailNum::Pair {
                    left  : Box::new(SnailNum::Leaf(*val/2)),
                    right : Box::new(SnailNum::Leaf((*val+1)/2)),
                };
                return Some(());
            }
        },
        SnailNum::Pair { left : l, right: r } => {
            // recurse in, left-first
            if split(l).is_some() || split(r).is_some() {
                return Some(());
            }
        },
    }
    return None;
}

fn reduce_step(n : &mut SnailNum) -> bool {
    // Look for an explode, then a split
    return explode(n, 0).is_some() || split(n).is_some();
}

fn magnitude(n : &SnailNum) -> usize {
    match n {
        SnailNum::Leaf(val) => { return *val; },
        SnailNum::Pair { left : l, right : r } => {
            return 3*magnitude(l) + 2*magnitude(r);
        },
    }
}

fn main() {
    let mut nums : Vec<SnailNum> = Vec::new();
    for line in io::stdin().lock().lines() {
        let txt = line.unwrap();
        nums.push(parse(&txt));
    }
    // Part 1: add #em all up
    let mut sum = nums[0].clone();
    for i in 1..nums.len() {
        add(&mut sum, &nums[i]);
        //println!("after add: {:?}", sum);
        while reduce_step(&mut sum) {
            //println!("reduced: {:?}", sum);
        }
    }
    println!("Part 1: magnitude: {}", magnitude(&sum));
    // Part 2: find combination of two numbers with the largest magnitude sum
    let mut max : usize = 0;
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if j!=i {
                let mut a = nums[i].clone();
                add(&mut a, &nums[j]);
                while reduce_step(&mut a) {}
                let mag = magnitude(&a);
                if mag>max {
                    max = mag;
                }
            }
        }
    }
    println!("Part 2: max: {}", max);
}