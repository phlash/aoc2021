use std::io::{self,BufRead};
use std::collections::{HashSet,HashMap};

struct Cave {
    name : String,
    is_big : bool,
    links : HashSet<String>,
}

fn new_cave(n : String) -> Cave {
    let cave = Cave {
        is_big: n.chars().next().unwrap().is_uppercase(),
        name  : n,
        links : HashSet::new(),
    };
    return cave;
}

// recursive route search.. we permit re-visiting 'big' (uppercase) caves
fn route(visited : &mut Vec<String>, has_two : bool, caves : &HashMap<String, Cave>, cave : &Cave) -> usize {
    // termination check
    if "end"==cave.name {
        // found a complete path, add one!
        return 1;
    }
    // small cave? check if we've visited before
    let mut can_two : bool = has_two;
    let mut has_push: bool = false;
    if !cave.is_big {
        if visited.contains(&cave.name) {
            // if this is 'start' or we've already made two visits elsewhere, bail
            if "start"==cave.name || !has_two { return 0; }
            can_two = false;
        } else {
            visited.push(cave.name.clone());
            has_push = true;
        }
    }
    // try all links
    let mut cnt : usize = 0;
    for link in &cave.links {
        cnt += route(visited, can_two, caves, caves.get(link).unwrap());
    }
    // pop from visited list..
    if has_push { visited.pop(); }
    return cnt;
}

fn main() {
    // our cave system to explore..
    let mut caves : HashMap<String, Cave> = HashMap::new();
    for line in io::stdin().lock().lines() {
        let txt = line.unwrap();
        let pair : Vec<&str> = txt.split('-').collect();
        if pair.len()!=2 { println!("Invalid line: {}", txt); return; }
        for p in 0..2 {
            // sometimes you find a nice way to do stuff!
            let cave = caves.entry(pair[p].to_string()).or_insert(new_cave(pair[p].to_string()));
            cave.links.insert(pair[1-p].to_string());
        }
    }
    println!("Loaded caves:");
    for (_n, cave) in &caves {
        println!("name: {} is_big: {} links: {:?}", cave.name, cave.is_big, cave.links);
    }

    // Part 1 - navigate caves to find all routes from 'start' to 'end' that only visit a small cave (lowercase) once
    let mut visited : Vec<String> = Vec::new();
    let start = caves.get("start").unwrap();
    println!("total: {}", route(&mut visited, true, &caves, &start))
}