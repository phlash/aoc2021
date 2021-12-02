use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() {
    let file = File::open("input").unwrap();
    let rdr = BufReader::new(file);
    let mut win : [i32; 3] = [0, 0, 0];
    let mut prv : i32 = 0;
    let mut prw : i32 = 0;
    let mut cnv : i32 = 0;
    let mut cnw : i32 = 0;
    for (idx,line) in rdr.lines().enumerate() {
        let val : i32 = line.unwrap().parse().unwrap();
        // populate the window
        win[idx%3] = val;
        // compare (if beyond first value)
        if idx>0 {
            if val>prv {
                cnv = cnv+1;
            }
        }
        // calculate sum
        let sum : i32 = win[0]+win[1]+win[2];
        // compare window (if beyond first three values)
        if idx>2 {
            if sum>prw {
                cnw = cnw+1
            }
        }
        prv = val;
        prw = sum;
    }
    println!("Increments: single:{} windowed:{}", cnv, cnw);
}
