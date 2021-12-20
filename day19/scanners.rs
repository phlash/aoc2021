use std::io::{self,BufRead};
use std::collections::HashMap;

// rotate a point to one of the 24 directions..
fn rotate(c: (isize,isize,isize), n : usize) -> (isize,isize,isize) {
    match n {
        0 => (c.0, c.1, c.2),
        1 => (c.0, c.2, -c.1),
        2 => (c.0, -c.1, -c.2),
        3 => (c.0, -c.2, c.1),
        4 => (c.1, c.0, -c.2),
        5 => (c.1, c.2, c.0),
        6 => (c.1, -c.0, c.2),
        7 => (c.1, -c.2, -c.0),
        8 => (c.2, c.0, c.1),
        9 => (c.2, c.1, -c.0),
        10 => (c.2, -c.0, -c.1),
        11 => (c.2, -c.1, c.0),
        12 => (-c.0, c.1, -c.2),
        13 => (-c.0, c.2, c.1),
        14 => (-c.0, -c.1, c.2),
        15 => (-c.0, -c.2, -c.1),
        16 => (-c.1, c.0, c.2),
        17 => (-c.1, c.2, -c.0),
        18 => (-c.1, -c.0, -c.2),
        19 => (-c.1, -c.2, c.0),
        20 => (-c.2, c.0, -c.1),
        21 => (-c.2, c.1, c.0),
        22 => (-c.2, -c.0, c.1),
        23 => (-c.2, -c.1, -c.0),
        _ => unreachable!(),
    }
}

// We need to exhaustively try all possible offsets (cartesian product of beacons and map entires) at all rotations
// until we get a maximum overlap of beacon positions - fugly, Fugly, FUGLY!
fn find_orientation(map : &HashMap<(isize,isize,isize),usize>, beacons : &Vec<(isize,isize,isize)>) -> (usize,usize,isize,isize,isize) {
    // try all rotations and offsets of beacons, until we maximize beacon alignment (defined as 12 or more)
    let mut max : usize = 0;
    let mut dir : usize = 0;
    let mut off : (isize,isize,isize) = (0,0,0);
    for n in 0..24 {
        // generate all the possible offsets from rotated beacons
        let rbeacons : Vec<(isize,isize,isize)> = beacons.iter().map(|&v| rotate(v, n)).collect();
        let mut offsets : Vec<(isize,isize,isize)> = Vec::new();
        for mk in map.keys() {
            for rb in &rbeacons {
                offsets.push( (mk.0-rb.0,mk.1-rb.1,mk.2-rb.2) );
            }
        }
        // try them..
        let mut mo : usize = 0;
        let mut mc : usize = 0;
        for o in 0..offsets.len() {
            mc = 0;
            for b in 0..rbeacons.len() {
                let bp = (rbeacons[b].0+offsets[o].0,rbeacons[b].1+offsets[o].1,rbeacons[b].2+offsets[o].2);
                if map.contains_key(&bp) {
                    mc += 1;
                }
            }
            if mc>=12 {
                mo = o;
                break;
            }
        }
        //println!("{} offsets: {} mc: {}", n, offsets.len(), mc);
        if mc>max {
            max = mc;
            dir = n;
            off = offsets[mo];
            if max>=12 { break; }
        }
    }
    // now we have orientation, and best offset values
    return (dir, max, off.0, off.1, off.2);
}

fn main() {
    let mut input : Vec<Vec<(isize,isize,isize)>> = Vec::new();
    let mut scanner : usize = 0;
    for line in io::stdin().lock().lines() {
        let txt = line.unwrap();
        // skip blanks
        if txt.len()<2 { continue; }
        // if line starts '---' it's a new scanner
        if "---"==&txt[0..3] { scanner += 1; continue; }
        // it's a co-ordinate triple
        let tc : Vec<&str> = txt.split(',').collect();
        if tc.len()!=3 { println!("invalid line: {}", txt); return; }
        let x = isize::from_str_radix(tc[0],10).unwrap();
        let y = isize::from_str_radix(tc[1],10).unwrap();
        let z = isize::from_str_radix(tc[2],10).unwrap();
        if input.len()<scanner { input.push(Vec::new()); }
        input[scanner-1].push((x,y,z));
    }
    // Now we merge each scanner in turn, assuming scanner 0 needs no re-orienting, into a single map
    let mut map : HashMap<(isize,isize,isize),usize> = HashMap::new();
    for coord in 0..input[0].len() {
        map.entry(input[0][coord]).or_insert(1);
        //println!("insert: {:?}: 1", input[0][coord]);
    }
    // keep trying to merge scans in, eventually we'll get them all :)
    let mut done : usize = 1;
    let mut spos : Vec<(isize,isize,isize)> = Vec::new();
    while done<input.len() {
        for scan in 1..input.len() {
            // skip if done
            if input[scan].len()==0 { continue; }
            // brute search orientation & offset
            let orient = find_orientation(&map, &input[scan]);
            println!("done: {}/{} scan: {} orient: {:?}", done, input.len(), scan, orient);
            if orient.1<12 { continue; }
            // merge beacons
            for b in 0..input[scan].len() {
                let mut rb = rotate(input[scan][b], orient.0);
                rb.0 += orient.2;
                rb.1 += orient.3;
                rb.2 += orient.4;
                let e = map.entry(rb).or_insert(0);
                *e += 1;
                //println!("merge: {:?}: {}", rb, *e);
            }
            // store scanner position (offset) for part2
            spos.push((orient.2,orient.3,orient.4));
            done += 1;
            input[scan].clear();
        }
    }
    println!("Part 1: beacons: {}", map.len());

    // Part 2: find largest Mahatten distance between any two scanners
    let mut maxm : isize = 0;
    for a in 0..spos.len() {
        for b in a+1..spos.len() {
            let d = (spos[a].0-spos[b].0).abs() +
                    (spos[a].1-spos[b].1).abs() +
                    (spos[a].2-spos[b].2).abs();
            if d>maxm { maxm = d; }
        }
    }
    println!("Part 2: maximum dist: {}", maxm);
}