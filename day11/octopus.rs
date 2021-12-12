use std::fs::File;
use std::io::{BufRead,BufReader};

fn printoct(octos : &[[usize;10];10]) {
    for y in 0..octos.len() {
        println!("{:?}", octos[y]);
    }
}

fn main() {
    //let file = File::open("test").unwrap();
    let file = File::open("input").unwrap();
    let mut octos : [[usize;10];10] = [[0;10];10];
    let rdr = BufReader::new(file);
    for (idx,line) in rdr.lines().enumerate() {
        let txt = line.unwrap();
        let bytes = txt.as_bytes();
        for p in 0..bytes.len() {
            octos[idx][p] = usize::from_str_radix(&txt[p..p+1],10).unwrap();
        }
    }
    printoct(&octos);
    // take steps until all flash together
    let mut total : usize = 0;
    for s in 0.. {
        // increment all energy levels
        for y in 0..octos.len() {
            for x in 0..octos[y].len() {
                octos[y][x] += 1;
            }
        }
        // find and propogate flashes until no new flashes occur
        let mut more : bool = true;
        let mut flash : [[bool;10];10] = [[false;10];10];
        while more {
            more = false;
            for y in 0..octos.len() {
                for x in 0..octos[y].len() {
                    if octos[y][x]>9 {
                        if !flash[y][x] {
                            flash[y][x] = true;
                            more = true;
                            if y>0 {
                                if x>0 { octos[y-1][x-1] += 1; }
                                octos[y-1][x] += 1;
                                if x<octos[y].len()-1 { octos[y-1][x+1] += 1; }
                            }
                            if x>0 { octos[y][x-1] += 1; }
                            if x<octos[y].len()-1 { octos[y][x+1] += 1; }
                            if y<octos.len()-1 {
                                if x>0 { octos[y+1][x-1] += 1; }
                                octos[y+1][x] += 1;
                                if x<octos[y].len()-1 { octos[y+1][x+1] += 1; }
                            }
                        }
                    }
                }
            }
        }
        // clear all flashed octopii, and count them
        let mut sum : usize = 0;
        for y in 0..octos.len() {
            for x in 0..octos[y].len() {
                if flash[y][x] {
                    flash[y][x] = false;
                    octos[y][x] = 0;
                    sum += 1;
                }
            }
        }
        total += sum;
        println!("step: {} sum: {}", s+1, sum);
        //printoct(&octos);
        // at 100th step, print part 1 total
        if 99==s { println!("Part 1: total: {}", total); }
        // when all flash together, stop and report step!
        if 100==sum {
            println!("All together: step: {}", s+1);
            break;
        }
    }
}