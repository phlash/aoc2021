use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() {
    //let file = File::open("test").unwrap();
    let file = File::open("input").unwrap();
    let rdr = BufReader::new(file);
    // we push opening chars to the stack, pop closing chars and check for a match
    let mut stack : Vec<u8> = Vec::new();
    let mut part1 : usize = 0;
    let mut part2 : Vec<usize> = Vec::new();
    for line in rdr.lines() {
        let txt = line.unwrap();
        let bytes = txt.as_bytes();
        for p in 0..bytes.len() {
            let v = bytes[p];
            match v {
                b'(' | b'[' | b'{' | b'<' => stack.push(v),
                b')' | b']' | b'}' | b'>' => {
                    let o = stack.pop().unwrap();
                    let t = if b'('==o { b')' } else if b'['==o { b']' } else if b'{'==o { b'}' } else { b'>' };
                    if t != v {
                        println!("{} :{} - expected {}, but found {}", txt, p, t as char, v as char);
                        part1 += if b')'==v { 3 } else if b']'==v { 57 } else if b'}'==v { 1197 } else { 25137 };
                        stack.clear();
                        break;
                    }
                },
                _ => { println!("invalid char: {}", v); return; },
            }
        }
        // Part 2: complete any unfinished lines by popping values..
        if stack.len()>0 { println!("{}: incomplete", txt); }
        let mut score : usize = 0;
        while stack.len()>0 {
            score *= 5;
            let o = stack.pop().unwrap();
            score += if b'('==o { 1 } else if b'['==o { 2 } else if b'{'==o { 3 } else { 4 };
        }
        if score>0 { part2.push(score); }
    }
    part2.sort();
    println!("Part 1: score: {}", part1);
    println!("Part 2: score: {}", part2[part2.len()/2]);
}