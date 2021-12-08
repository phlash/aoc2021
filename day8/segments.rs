use std::fs::File;
use std::io::{BufRead,BufReader};

/* 7-segment display patterns, based on:
  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg

*/
const DIGITS : [[bool; 7]; 10] = [
    [true,true,true,false,true,true,true],//0
    [false,false,true,false,false,true,false],//1
    [true,false,true,true,true,false,true],//2
    [true,false,true,true,false,true,true],//3
    [false,true,true,true,false,true,false],//4
    [true,true,false,true,false,true,true],//5
    [true,true,false,true,true,true,true],//6
    [true,false,true,false,false,true,false],//7
    [true,true,true,true,true,true,true],//8
    [true,true,true,true,false,true,true],//9
];

fn find_digit(bits : &[bool;7]) -> usize {
    for dig in 0..DIGITS.len() {
        let mut vb : bool = true;
        for bit in 0..DIGITS[dig].len() {
            if bits[bit] != DIGITS[dig][bit] { vb = false; }
        }
        if vb { return dig; }
    }
    return 99;
}

fn check_valid(wires : &[usize;7], tests : &Vec<[bool;7]>) -> bool {
    // try each pattern as input, check we get a valid digit as output
    for test in 0..tests.len() {
        let mut out : [bool; 7] = [false; 7];
        // map test->out segments
        for w in 0..wires.len() {
            out[wires[w]] = tests[test][w];
        }
        // check against valid digits
        if find_digit(&out)>9 { return false; }
    }
    return true;
}

// Heap's algorithm to generate permutations
// https://en.wikipedia.org/wiki/Heap%27s_algorithm
fn generate(k : usize, a : &mut [usize;7], perms : &mut Vec<[usize;7]>) {
    if 1==k {
        perms.push(a.clone());
    } else {
        generate(k-1, a, perms);
        for i in 0..k-1 {
            if k&1>0 {
                // odd
                let t = a[0];
                a[0] = a[k-1];
                a[k-1] = t;
            } else {
                // even
                let t = a[i];
                a[i] = a[k-1];
                a[k-1] = t;
            }
            generate(k-1, a, perms);
        }
    }
}

fn map_to_bits(pat : &str) -> [bool;7] {
    let mut bits : [bool;7] = [false;7];
    for c in pat.chars() {
        match c {
            'a' => { bits[0]=true; },
            'b' => { bits[1]=true; },
            'c' => { bits[2]=true; },
            'd' => { bits[3]=true; },
            'e' => { bits[4]=true; },
            'f' => { bits[5]=true; },
            'g' => { bits[6]=true; },
            _ => { println!("invalid segment: {}", c); }
        }
    }
    return bits;
}

fn main() {
    //let file = File::open("test").unwrap();
    let file = File::open("input").unwrap();
    let rdr = BufReader::new(file);
    let mut perms : Vec<[usize;7]> = Vec::new();
    // generate all possible wiring permutations for Part 2, we use Heap's Algorithm
    generate(7, &mut [0,1,2,3,4,5,6], &mut perms);
    println!("generated: {} perms", perms.len());
    // counter for Part 1
    let mut cnt : usize = 0;
    // total for Part 2
    let mut total : usize = 0;
    for (idx,line) in rdr.lines().enumerate() {
        let txt = line.unwrap();
        // each line consists of: <unique patterns>{10} | <digit>{4}
        // where <unique pattern> & <digit> are segment lists: <a-g>{1,7}
        let pair :Vec<&str> = txt.split(" | ").collect();
        if pair.len()!=2 {
            println!("oops, invalid line: {}", txt);
            return;
        }
        let pats :Vec<&str> = pair[0].split_whitespace().collect();
        let digs :Vec<&str> = pair[1].split_whitespace().collect();

        // Part 2: deduce wiring of segments from unique patterns:
        // we try all permutations until only valid digits are produced by provided patterns
        // ..first convert patterns to input bits
        let mut tests : Vec<[bool;7]> = Vec::new();
        for pat in 0..pats.len() {
            let bits = map_to_bits(pats[pat]);
            tests.push(bits);
        }
        // ..now we exhaustively search permutations
        for perm in 0..perms.len() {
            if check_valid(&perms[perm], &tests) {
                // found one! decode related digits
                let mut sum : usize = 0;
                for dig in 0..digs.len() { // Part 1: count occurrances of unique digits (1,4,7,8)
                    match digs[dig].len() {
                        2 | 3 | 4 | 7 => { cnt += 1 },
                        _ => {},
                    }
                    let bits = map_to_bits(digs[dig]);
                    let mut out : [bool; 7] = [false; 7];
                    for b in 0..bits.len() {
                        out[perms[perm][b]] = bits[b];
                    }
                    let fnd = find_digit(&out);
                    if fnd>9 { println!("invalid bits: {:?}", out); }
                    sum *= 10;
                    sum += fnd;
                }
                total += sum;
                println!("Part 2: row: {} wires: {:?} sum: {}", idx, &perms[perm], sum);
                break;
            }
        }
    }
    println!("Part 1: cnt: {}", cnt);
    println!("Part 2: total: {}", total);
}
