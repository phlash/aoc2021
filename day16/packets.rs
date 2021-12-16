use std::io::{self,BufRead};

fn op(typ : u8, sub : usize, cur : usize, val : usize) -> usize {
    // if first sub (sub==0), just the val
    if 0==sub { return val; }
    // apply operator based on typ between cur & val
    match typ {
        0x0 => {
            // sum value
            return cur + val;
        },
        0x01 => {
            // product value
            return cur * val;
        },
        0x02 => {
            // min value
            return if val<cur { val } else { cur };
        },
        0x03 => {
            // max value
            return if val>cur { val } else { cur };
        },
        0x05 => {
            // greater than
            return if cur>val { 1 } else { 0 };
        },
        0x06 => {
            // less than
            return if cur<val { 1 } else { 0 };
        },
        0x07 => {
            // equal to
            return if cur==val { 1 } else { 0 };
        },
        _ => { println!("invalid typ!"); },
    }
    return 0;
}

fn parse(packet : &Vec<bool>, pos : usize, vsum : &mut usize, pval : &mut usize) -> usize {
    // parse a packet, possibly recursively, from bit indexed by pos, return new pos
    let mut np : usize = pos;
    let mut ver : u8 = 0;
    for _i in 0..3 { ver <<= 1; ver += if packet[np] { 1 } else { 0 }; np += 1; }
    let mut typ : u8 = 0;
    for _i in 0..3 { typ <<= 1; typ += if packet[np] { 1 } else { 0 }; np += 1; }
    print!("ver={} typ={} ", ver, typ);
    // add version to sum
    *vsum += ver as usize;
    match typ {
        0x4 => {
            // absolute value, MIDI style in 4-bit chunks
            let mut val : usize = 0;
            let mut cont : bool = true;
            while cont {
                // read continuation bit
                cont = packet[np];
                np += 1;
                // read 4 value bits
                for _i in 0..4 { val <<= 1; val += if packet[np] { 1 } else { 0 }; np += 1; }
            }
            print!("abs={} ", val);
            // absolute packet value
            *pval = val;
        },
        _ => {
            // operator packet, check length type
            if packet[np] {
                // sub-packet count in next 11 bits
                let mut cnt : usize = 0;
                np += 1;
                for _i in 0..11 { cnt <<= 1; cnt += if packet[np] { 1 } else { 0 }; np += 1; }
                // recursively parse sub-packets
                print!("subs={} ", cnt);
                let mut oval : usize =  0;
                for s in 0..cnt {
                    print!("< ");
                    let mut sval : usize = 0;
                    np = parse(packet, np, vsum, &mut sval);
                    // apply typed operator
                    oval = op(typ, s, oval, sval);
                    print!("> ");
                }
                // return our calculated value
                *pval = oval;
            } else {
                // sub-packet bits in next 15 bits
                let mut cnt : usize = 0;
                np += 1;
                for _i in 0..15 { cnt <<= 1; cnt += if packet[np] { 1 } else { 0 }; np += 1; }
                // recursively parse sub-packets until no more bits
                print!("bits={} ", cnt);
                let mut oval : usize = 0;
                let fin : usize = np + cnt;
                let mut s : usize = 0;
                while np<fin {
                    print!("< ");
                    let mut sval : usize = 0;
                    np = parse(packet, np, vsum, &mut sval);
                    // apply typed operator
                    oval = op(typ, s, oval, sval);
                    print!("> ");
                    s += 1;
                }
                // return our calculated value
                *pval = oval;
            }
        },
    }
    return np;
}

fn main() {
    let mut packet : Vec<bool> = Vec::new();
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    // it's a big string of hex, that we want as a string of bits..
    for c in line.chars() {
        let h = u8::from_str_radix(&c.to_string(),16).unwrap();
        packet.push(h&0x8!=0);
        packet.push(h&0x4!=0);
        packet.push(h&0x2!=0);
        packet.push(h&0x1!=0);
    }
    // parse 'em cowboy!
    let mut sum : usize = 0;
    let mut val : usize = 0;
    parse(&packet, 0, &mut sum, &mut val);
    println!("\nPart 1: sum={}", sum);
    println!("Part 2: val={}", val);
}