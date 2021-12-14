use std::io::{self,BufRead};
use std::collections::HashMap;

struct Poly {
    pcnts : HashMap<String, u64>,
    lcnts : HashMap<String, u64>,
}

fn step(poly : &mut Poly, rules : &HashMap<String,String>) {
    // for all rules, if pairs exist in the poly, count them
    let mut hits : HashMap<String,u64> = HashMap::new();
    for r in rules.keys() {
        if poly.pcnts.contains_key(r) {
            hits.entry(r.to_string()).or_insert(poly.pcnts[r]);
        }
    }
    // now adjust poly counts, and create new pairs if required
    for h in hits.keys() {
        let c = &rules[h];
        let n = hits[h];
        // letter count increments for newly inserted element
        let v = poly.lcnts.entry(c.to_string()).or_insert(0);
        *v += n;
        // pair count drops for all pairs that are split
        let p = poly.pcnts.entry(h.to_string()).or_insert(0);
        *p -= n;
        // two (new) pairs formed around inserted element
        let mut p1 = h[0..1].to_string().clone();
        p1.push_str(&c);
        let mut p2 : String = c.clone();
        p2.push_str(&h[1..2]);
        let c1 = poly.pcnts.entry(p1).or_insert(0);
        *c1 += n;
        let c2 = poly.pcnts.entry(p2).or_insert(0);
        *c2 += n;
    }
}

fn main() {
    let mut rules : HashMap<String,String> = HashMap::new();
    let mut poly = Poly {
        pcnts : HashMap::new(),
        lcnts : HashMap::new(),
    };
    let mut init : String = "".to_string();
    for (idx,line) in io::stdin().lock().lines().enumerate() {
        let txt = line.unwrap();
        if 0==idx {
            // initial polymer chain
            init = txt.clone();
            for c in txt.chars() {
                let v = poly.lcnts.entry(c.to_string()).or_insert(0);
                *v += 1;
            }
        } else if idx>1 {
            // pair matching rule
            let pair : Vec<&str> = txt.split(" -> ").collect();
            if pair.len()!=2 { println!("invalid line: {}", txt); return; }
            rules.entry(pair[0].to_string()).or_insert(pair[1].to_string());
        }
    }
    // populate initial pair counts in poly
    for i in 0..init.len()-1 {
        let p = &init[i..i+2];
        if rules.contains_key(p) {
            let v = poly.pcnts.entry(p.to_string()).or_insert(0);
            *v += 1;
        }
    }
    // take 40 steps
    for s in 1..41 {
        step(&mut poly, &rules);
        println!("step: {} pcnts: {:?} lcnts: {:?}", s, poly.pcnts, poly.lcnts);
    }
    let mut lcc : u64 = u64::MAX;
    let mut mcc : u64 = 0;
    for v in poly.lcnts.values() {
        if *v<lcc { lcc=*v; }
        if *v>mcc { mcc=*v; }
    }
    println!("lcc: {}, mcc: {} diff: {}", lcc, mcc, mcc-lcc);
}