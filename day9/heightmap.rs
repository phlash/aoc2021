use std::fs::File;
use std::io::{BufRead,BufReader};

fn basin_size(hmap : &Vec<Vec<usize>>, sx : usize, sy : usize) -> usize {
    // basically a floodfill algorithm, but terminated by map edge or height==9
    let mut size : usize = 0;

    // next y value, initially starting y
    let mut ny : usize = sy;
    // collected starting points (x values) on next row, initially starting x
    let mut starts : Vec<usize> = Vec::new();
    starts.push(sx);
    // while we have starting points..
    while starts.len()>0 {
        print!(" L^{}", starts.len());
        let mut nexts : Vec<usize> = Vec::new();
        while starts.len()>0 {
            let mut pst : bool = false;
            let mut rx : usize = starts.remove(0);
            // walk left to find edge
            while rx>0 && hmap[ny][rx-1]!=9 { rx -= 1; }
            // walk right to find edge, look for new starting points above
            while rx<hmap[ny].len() {
                // count basin points
                size += 1;
                print!(" [{},{}]", rx, ny);
                if ny>0 && hmap[ny-1][rx]!=9 {
                    // found a start point above..record it unless previous point was a start
                    if !pst { pst = true; nexts.push(rx); print!(" S@[{}]", rx); }
                }
                if ny>0 && hmap[ny-1][rx]==9 {
                    // found an edge above, clear previous start marker
                    pst = false;
                }
                // walk right
                rx += 1;
                // terminate if we find an edge
                if rx<hmap[ny].len() && hmap[ny][rx]==9 {
                    print!(" E@[{},{}]", rx, ny);
                    break;
                }
            }
            // remove any starting points we have passed..
            while starts.len()>0 && starts[0]<rx { starts.remove(0); }
        }
        // move up row
        if ny>0 { ny -= 1; }
        starts = nexts;
    }
    // repeat the walking right on initial row to find starting points below.. repeat
    // until at bottom or no more starting points
    ny = sy;
    starts.clear(); // should be uneccessary but..
    starts.push(sx);
    while starts.len()>0 {
        print!(" Lv{}", starts.len());
        let mut nexts : Vec<usize> = Vec::new();
        while starts.len()>0 {
            let mut pst : bool = false;
            let mut rx : usize = starts.remove(0);
            // walk left to find edge
            while rx>0 && hmap[ny][rx-1]!=9 { rx -= 1; }
            // walk right to find edge, look for starting points below
            while rx<hmap[ny].len() {
                // only count basin points if below initial row (we counted that already)
                if ny!=sy { print!(" [{},{}]", rx, ny); size += 1; }
                if ny<hmap.len()-1 && hmap[ny+1][rx]!=9 {
                    // found a start point..record it unless previous point was a start
                    if !pst { pst = true; nexts.push(rx); print!(" S@[{}]", rx); }
                }
                if ny<hmap.len()-1 && hmap[ny+1][rx]==9 {
                    // found an edge below, clear previous start marker
                    pst = false;
                }
                // walk right
                rx += 1;
                // terminate if we find an edge
                if rx<hmap[ny].len() && hmap[ny][rx]==9 {
                    print!(" E@[{},{}]", rx, ny);
                    break;
                }
            }
            // remove any starting points we have passed..
            while starts.len()>0 && starts[0]<rx { starts.remove(0); }
        }
        ny += 1;
        starts = nexts;
    }
    println!("");
    return size;
}

fn main() {
    //let file = File::open("test").unwrap();
    let file = File::open("input").unwrap();
    let rdr = BufReader::new(file);
    let mut hmap : Vec<Vec<usize>> = Vec::new();
    for line in rdr.lines() {
        let txt = line.unwrap();
        // each line is a row in the heightmap, values 0-9
        let mut r : Vec<usize> = Vec::new();
        for p in 0..txt.as_bytes().len() {
            let v = usize::from_str_radix(&txt[p..p+1],10).unwrap();
            r.push(v);
        }
        hmap.push(r);
    }
    // Part 1: find the low points, score them..
    let mut risk : usize = 0;
    // Part 2: measure the basin sizes, collect them..
    let mut basins : Vec<usize> = Vec::new();
    for y in 0..hmap.len() {
        for x in 0..hmap[y].len() {
            let v = hmap[y][x];
            // at a low point?
            let mut low : bool = true;
            // left
            if x>0 && hmap[y][x-1]<=v { low = false; }
            // right
            if x<hmap[y].len()-1 && hmap[y][x+1]<=v { low = false; }
            // above
            if y>0 && hmap[y-1][x]<=v { low = false; }
            // below
            if y<hmap.len()-1 && hmap[y+1][x]<=v { low = false; }
            // score the risk
            if low {
                risk += v+1;
                let b = basin_size(&hmap, x, y);
                basins.push(b);
                println!("low@ {},{} basin: {}", x, y, b);
            }
        }
    }
    println!("Part 1: risk: {}", risk);
    // find three largest basins, multiply them
    let mut total : usize = 1;
    for _b in 0..3 {
        let mut max : usize = 0;
        let mut idx : usize = 0;
        for i in 0..basins.len() {
            if basins[i]>max {
                max = basins[i];
                idx = i;
            }
        }
        basins.remove(idx);
        total *= max;
        println!("max: {}@{}", max, idx);
    }
    println!("Part 2: total: {}", total);
}