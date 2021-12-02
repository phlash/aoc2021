use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() {
    let file = File::open("input").unwrap();
    let rdr = BufReader::new(file);
    let mut hoz : i32 = 0;
    let mut de1 : i32 = 0;
    let mut de2 : i32 = 0;
    let mut aim : i32 = 0;
    for (idx,line) in rdr.lines().enumerate() {
        // each line has '<command> <distance>'
        let txt = line.unwrap();
        let vals : Vec<&str> = txt.split_whitespace().collect();
        let cmd = vals[0];
        let dis : i32 = vals[1].parse().unwrap();
        match cmd {
            "forward" => { hoz = hoz+dis; de2 = de2+(aim*dis) },
            "down" => { de1 = de1+dis; aim = aim+dis }
            "up" => { de1 = de1-dis; aim = aim-dis },
            _ => {}
        }
    }
    println!("hoz:{} de1:{} de2:{} mu1:{} mu2:{}", hoz, de1, de2, hoz*de1, hoz*de2);
}
