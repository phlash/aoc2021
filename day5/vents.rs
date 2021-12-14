use std::io::{self,BufRead};

fn main() {
    let mut vents : [i32; 1000000] = [0; 1000000];     // by examination we see floor is 1000x1000 units
    for line in io::stdin().lock().lines() {
        let txt = line.unwrap();
        // each line is a co-ordinate pair: <x>,<y> -> <x>,<y>
        let pair : Vec<&str> = txt.split(" -> ").collect();
        if pair.len()!=2 {
            println!("unparsable pair: {}", txt);
            return;
        }
        let crd1 : Vec<&str> = pair[0].split(',').collect();
        let crd2 : Vec<&str> = pair[1].split(',').collect();
        if crd1.len()!=2 {
            println!("unparseable co-ords(0): {}", pair[0]);
        }
        if crd2.len()!=2 {
            println!("unparseable co-ords(1): {}", pair[1]);
        }
        let x1 = usize::from_str_radix(crd1[0],10).unwrap();
        let y1 = usize::from_str_radix(crd1[1],10).unwrap();
        let x2 = usize::from_str_radix(crd2[0],10).unwrap();
        let y2 = usize::from_str_radix(crd2[1],10).unwrap();
        // horz or vert (or broken!)?
        if x1==x2 {
            // vert - add to floor plan
            for y in if y2<y1 { y2..y1+1 } else { y1..y2+1 } {
                vents[1000*y+x1] += 1;
            }
        } else if y1==y2 {
            // horz - add to floor plan
            for x in if x2<x1 { x2..x1+1 } else { x1..x2+1 } {
                vents[1000*y1+x] += 1;
            }
        } else {
            // diagonal (45 degrees only we are assured) - verify, then add to floor plan
            let xrev = if x2<x1 { true } else { false };
            let yrev = if y2<y1 { true } else { false };
            let xd = if xrev { x1-x2 } else { x2-x1 };
            let yd = if yrev { y1-y2 } else { y2-y1 };
            if xd != yd { println!("invalid diagonal: {},{} -> {},{}", x1, y1, x2, y2); }
            let mut x: usize = x1;
            let mut y: usize = y1;
            while x!=x2 {
                vents[1000*y+x] += 1;
                x = if xrev { x-1 } else { x+1 };
                y = if yrev { y-1 } else { y+1 };
            }
            // inclusive range, so add one more point
            vents[1000*y+x] += 1;
        }
    }
    // loaded the vents - find all points of value 2 or more
    let mut count : i32 = 0;
    for n in 0..1000000 {
        if vents[n]>1 {
            count += 1;
        }
    }
    println!("hot spots: {}", count);
}