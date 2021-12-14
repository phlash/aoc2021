use std::io::{self,BufRead};
use std::collections::HashMap;

fn fold(points : &mut HashMap<usize, HashMap<usize, bool>>, fold : isize) {
    let xfold : usize = -fold as usize;
    let yfold : usize = fold as usize;
    if fold>0 {
        // positive fold, y-changes
        // all y co-ordinates above the fold are folded: y => 2f-y
        let iy : Vec<usize> = points.keys().cloned().collect();
        for y in iy {
            if y>yfold {
                let ny = 2*yfold-y;
                let ix : Vec<usize> = points[&y].keys().cloned().collect();
                let row = points.entry(ny).or_insert(HashMap::new());
                for x in ix {
                    row.entry(x).or_insert(true);
                }
                points.remove(&y);
            }
        }
    } else {
        // negative fold, x-changes
        // all x co-ordinates above the fold are folded: x => 2f-x
        let iy : Vec<usize> = points.keys().cloned().collect();
        for y in iy {
            let ix : Vec<usize> = points[&y].keys().cloned().collect();
            for x in ix {
                if x>xfold {
                    let nx = 2*xfold-x;
                    let row = points.get_mut(&y).unwrap();
                    row.entry(nx).or_insert(true);
                    row.remove(&x);
                }
            }
        }
    }
}

fn main() {
    let mut points : HashMap<usize, HashMap<usize, bool>> = HashMap::new();
    let mut folds : Vec<isize> = Vec::new();
    for line in io::stdin().lock().lines() {
        let txt = line.unwrap();
        // if line contains ',', assume co-ordinates for point map
        if txt.contains(",") {
            let pair : Vec<&str> = txt.split(',').collect();
            if pair.len()!=2 { println!("invalid line: {}", txt); return; }
            let x = usize::from_str_radix(&pair[0],10).unwrap();
            let y = usize::from_str_radix(&pair[1],10).unwrap();
            // Y-first sparse co-ordinate map
            let row = points.entry(y).or_insert(HashMap::new());
            row.entry(x).or_insert(true);
        }
        // if line contains '=', assume fold instruction
        if txt.contains("=") {
            let pair : Vec<&str> = txt.split('=').collect();
            if pair.len()!=2 { println!("invalid line: {}", txt); return; }
            let d = match pair[0] {
                "fold along x" => -1,
                "fold along y" => 1,
                _ => { println!("unmatched fold: {}", pair[0]); return; }
            };
            let p = isize::from_str_radix(pair[1],10).unwrap();
            folds.push(d*p);
        }
    }
    println!("map: {:?}\nfolds: {:?}", points, folds);
    // Part 1: process the first fold, count the points.
    fold(&mut points, folds[0]);
    let mut sum : usize = 0;
    for y in points.keys() {
        sum += points[y].len();
    }
    println!("Part 1 points: {}", sum);
    // Complete folding..
    for f in 1..folds.len() {
        fold(&mut points, folds[f]);
    }
    // pretty print the map
    let mut sy : Vec<usize> = points.keys().cloned().collect();
    sy.sort();
    for y in sy {
        let row = &points[&y];
        let mut c : usize = 0;
        let mut x : usize = 0;
        while c<row.len() {
            if row.contains_key(&x) {
                c += 1;
                print!("#");
            } else {
                print!(" ");
            }
            x += 1;
        }
        println!("");
    }
}