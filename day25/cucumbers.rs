use std::io::{self,BufRead};

#[derive(Debug,Copy,Clone,PartialEq)]
enum Cucumber {
    None,
    East,
    South,
}

fn step(map : &Vec<Vec<Cucumber>>, moved : &mut usize) -> Vec<Vec<Cucumber>> {
    let mut mv : usize = 0;
    let mut xmap = map.clone();
    // here be the rules..
    // East facing move first, if they have a space to move into (including wrap-around)
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let nx = (x+1)%map[y].len();
            if Cucumber::East==map[y][x] {
                if Cucumber::None==map[y][nx] { xmap[y][x] = Cucumber::None; xmap[y][nx] = Cucumber::East; mv += 1; }
            }
        }
    }
    // South facing move next, as above..
    let mut ymap = xmap.clone();
    for y in 0..xmap.len() {
        let ny = (y+1)%xmap.len();
        for x in 0..xmap[y].len() {
            if Cucumber::South==xmap[y][x] {
                if Cucumber::None==xmap[ny][x] { ymap[y][x] = Cucumber::None; ymap[ny][x] = Cucumber::South; mv += 1; }
            }
        }
    }
    *moved = mv;
    return ymap;
}

fn print_map(step : usize, moved : usize, map : &Vec<Vec<Cucumber>>) {
    println!("step: {} moved: {}", step, moved);
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{}", match map[y][x] { Cucumber::None => '.', Cucumber::East => '>', Cucumber::South => 'v', });
        }
        println!("");
    }
    println!("");
}

fn main() {
    let mut map : Vec<Vec<Cucumber>> = Vec::new();
    for line in io::stdin().lock().lines() {
        // each line is a row in the map containing initial state
        let txt = line.unwrap();
        let mut row : Vec<Cucumber> = Vec::new();
        for c in txt.chars() {
            row.push(match c {
                '.' => Cucumber::None,
                '>' => Cucumber::East,
                'v' => Cucumber::South,
                _ => unreachable!(),
            });
        }
        map.push(row);
    }
    print_map(0, 0, &map);
    for s in 0.. {
        let mut moved : usize = 0;
        map = step(&map, &mut moved);
        print_map(s+1, moved, &map);
        if 0==moved { break; }
    }
}