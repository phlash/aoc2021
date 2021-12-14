use std::io::{self,BufRead};
use std::convert::TryInto;

fn main() {
    //let wid : usize = 5;
    let wid : usize = 12;
    let mut inp : Vec<i32> = Vec::new();
    for line in io::stdin().lock().lines() {
        // each line has '<bit>*12'
        let txt = line.unwrap();
        let val = i32::from_str_radix(&txt,2).unwrap();
        inp.push(val);
    }
    // Part1: generate gamma & epsilon from most common (or least common) bits
    let tot : i32 = (inp.len()).try_into().unwrap();
    let cnts = bitcounts(&inp, wid);
    let mut gamma : i32 = 0;
    for i in 0..wid {
        gamma = gamma<<1;
        let n : usize = wid-1-i;
        println!("bit:{}={}", n, cnts[n]);
        if cnts[n]==tot/2 {
            println!("ambiguous bit!");
        } else if cnts[n]>tot/2 {
            gamma |= 1;
        }
    }
    let epsilon = !gamma & ((1<<wid)-1);
    println!("Part1: gamma: {:b}={}, epsilon: {:b}={}, mul: {}", gamma, gamma, epsilon, epsilon, gamma*epsilon);

    // Part2: filter values bit-by-bit that match most common (or least common) bit until only 1 remains
    let mut o2vals : Vec<i32> = inp.clone();
    let mut bit : usize = wid-1;
    while o2vals.len()>1 {
        let len : i32 = (o2vals.len()).try_into().unwrap();
        let cnts = bitcounts(&o2vals, wid);
        // most common bit at position 'bit'
        let mcb : i32 = if cnts[bit]*2>=len { 1<<bit } else { 0 };
        let msk : i32 = 1<<bit;
        println!("..o2vals.len:{}, cnts[bit]:{}, mcb:{:b}, msk:{:b}", o2vals.len(), cnts[bit], mcb, msk);
        // filter the list..
        o2vals = o2vals.into_iter().filter(|v| (v&msk)==mcb).collect();
        if 0==bit && o2vals.len()>1 {
            println!("filter failed to find a unique solution :=(");
            return;
        }
        if bit<6 {
            for val in &o2vals { println!("..o2val:{:b}", val); }
        }
         // shift the bit
        bit = if bit>0 { bit-1 } else { 0 };
    }
    println!("Part2: mcb value {:b}={}", o2vals[0], o2vals[0]);
    let mut covals : Vec<i32> = inp.clone();
    bit = wid-1;
    while covals.len()>1 {
        let len : i32 = (covals.len()).try_into().unwrap();
        let cnts = bitcounts(&covals, 12);
        // least common bit at position 'bit'
        let lcb : i32 = if cnts[bit]*2<len { 1<<bit } else { 0 };
        let msk : i32 = 1<<bit;
        println!("..covals.len:{}, cnts[bit]:{} lcb:{:b}, msk:{:b}", covals.len(), cnts[bit], lcb, msk);
        // filter the list..
        covals = covals.into_iter().filter(|v| (v&msk)==lcb).collect();
        if 0==bit && covals.len()>1 {
            println!("filter failed to find a unique solution :=(");
            return;
        }
        if bit<6 {
            for val in &covals { println!("..coval:{:b}", val); }
        }
        // shift the bit
        bit = if bit>0 { bit-1 } else { 0 };
    }
    println!("Part2: lcb value {:b}={}", covals[0], covals[0]);
    println!("Part2: mul:{}", o2vals[0]*covals[0]);
}

fn bitcounts(vals : &Vec<i32>, w: usize) -> Vec<i32> {
    let mut res : Vec<i32> = vec![0; w];
    for val in vals {
        for n in 0..w {
            let tst : i32 = 1<<n;
            if (val & tst) != 0 {
                res[n] += 1;
            }
        }
    }
    return res;
}