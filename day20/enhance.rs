use std::io::{self,BufRead};

fn enhance(map : &Vec<Vec<bool>>, lookup : &Vec<bool>, apply : usize) -> Vec<Vec<bool>> {
    // apply enhance algorithm to bits in map, extending on all sides by 1 pixel
    // NB: We can ignore the infinte size, even when lookup[0]='#' as we apply twice
    // and lookup[511]='.' which toggles the infinite background off again :=)
    // although we must assume new pixels are zero in first apply, one in next etc.
    // which amount to if bit zero is set in 'apply' counter, and only if lookup[0]=='#'.
    // returns fresh map
    let mut nmap = Vec::with_capacity(map.len()+2);
    let ob = if lookup[0] && apply&1>0 { true } else { false };
    //print!("x/y/i: [");
    for y in 0..nmap.capacity() {
        let mut row = Vec::with_capacity(map[0].len()+2);
        for x in 0..row.capacity() {
            // apply algorithm, inserting true/false as appropriate for outside bits
            let mut i : usize = 0;
            for dy in -1..2 {
                for dx in -1..2 {
                    i <<= 1;
                    let ly = (y as isize)+dy;
                    let lx = (x as isize)+dx;
                    let b = if lx>0 && ly>0 && lx<(row.capacity() as isize)-1 && ly<(nmap.capacity() as isize)-1 {
                        map[(ly-1) as usize][(lx-1) as usize]
                    } else {
                        ob
                    };
                    i += if b { 1 } else { 0 };
                }
            }
            //print!("{}/{}/{}, ", x, y, i);
            row.push(lookup[i]);
        }
        nmap.push(row);
    }
    //println!("");
    return nmap;
}

fn parse(txt : &str) -> Vec<bool> {
    let mut r : Vec<bool> = Vec::new();
    for c in txt.chars() {
        match c {
            '#' => { r.push(true); },
            '.' => { r.push(false); },
            _ => unreachable!(),
        }
    }
    return r;
}

fn print_map(map : &Vec<Vec<bool>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            match map[y][x] {
                false => print!("."),
                true => print!("#"),
            }
        }
        println!("");
    }
}

fn lit(map : &Vec<Vec<bool>>) -> usize {
    let mut cnt : usize = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] { cnt += 1; }
        }
    }
    return cnt;
}

fn main() {
    // TODO: read input to lookup and starting map,
    // Part 1: apply twice, print count.
    let mut map : Vec<Vec<bool>> = Vec::new();
    let mut lookup : Vec<bool> = Vec::new();
    for line in io::stdin().lock().lines() {
        let txt = line.unwrap();
        if txt.len()<2 { continue; }
        if lookup.len()==0 {
            lookup = parse(&txt);
        } else {
            map.push(parse(&txt));
        }
    }
    println!("lookup: {:?}\nmap:", lookup.len());
    print_map(&map);
    let mut p1 : usize = 0;
    for apply in 0..50 {
        map = enhance(&map, &lookup, apply);
        println!("apply: {}", apply);
        print_map(&map);
        if 1==apply { p1 = lit(&map); }
    }
    // Part 1: count lit pixels
    println!("Part 1: {}", p1);
    println!("Part 2: {}", lit(&map));
}