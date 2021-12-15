use std::io::{self,BufRead};
use std::collections::HashSet;

struct Node {
    vist : bool,
    cost : usize,
    prev : (usize, usize),
}

fn new_node(c : usize, p : (usize,usize)) -> Node {
    let n = Node {
        vist : false,
        cost : c,
        prev : p,
    };
    return n;
}

fn cost(map : &Vec<Vec<usize>>, visited : &mut Vec<Vec<Node>>, y : usize, x : usize, ny : usize, nx : usize) -> bool {
    // ignore visited nodes
    if visited[ny][nx].vist { return false; }
    // calc cost from current cost and map data (risk)
    let c = visited[y][x].cost + map[ny][nx];
    // update node cost if required (and path back)
    if c<visited[ny][nx].cost {
        visited[ny][nx].cost = c;
        visited[ny][nx].prev = (x,y);
    }
    return true;
}

fn next(tovisit : &mut HashSet<(usize,usize)>, visited : &Vec<Vec<Node>>) -> (usize,usize) {
    // search map for lowest cost node
    let mut lc : usize = usize::MAX;
    let mut ln : (usize,usize) = (0,0);
    for n in tovisit.iter() {
        if visited[n.1][n.0].cost < lc {
            lc = visited[n.1][n.0].cost;
            ln = *n;
        }
    }
    // remove from list and return
    tovisit.remove(&ln);
    return ln;
}

fn route(map : &Vec<Vec<usize>>, pfx : &str) {
    // Part 1: find lowest cost route from top left (0,0) to bottom right
    // looks like Djikstra's algorithm for route finding..
    // https://en.wikipedia.org/wiki/Dijkstra's_algorithm
    // we need a visited map with costs and previous node..
    let mut visited : Vec<Vec<Node>> = Vec::new();
    // initalise all costs to usize::MAX in visited map
    for y in 0..map.len() {
        let mut row : Vec<Node> = Vec::new();
        for _x in 0..map[y].len() {
            row.push(new_node(usize::MAX, (0,0)));
        }
        visited.push(row);
    }
    // while we haven't reached the destination..
    let mut x : usize = 0;
    let mut y : usize = 0;
    visited[0][0].cost = 0;
    let mut cnt : usize = 0;
    let mut tovisit : HashSet<(usize,usize)> = HashSet::new();
    while y!=map.len()-1 || x!=map[0].len()-1 {
        // mark current node as visited
        visited[y][x].vist = true;
        // calculate cost to each unvisited neighbour of current node (no diagnonals),
        // add to list of unvisited nodes
        if y>0 { if cost(&map, &mut visited, y, x, y-1, x) { tovisit.insert((x,y-1)); } }
        if y<map.len()-1 { if cost(&map, &mut visited, y, x, y+1, x) { tovisit.insert((x,y+1)); } }
        if x>0 { if cost(&map, &mut visited, y, x, y, x-1) { tovisit.insert((x-1,y)); } }
        if x<map[0].len()-1 { if cost(&map, &mut visited, y, x, y, x+1) { tovisit.insert((x+1,y)); } }
        // find lowest cost unvisited neighbour, make current
        let nn = next(&mut tovisit, &visited);
        x = nn.0;
        y = nn.1;
        // stats..
        cnt += 1;
        if cnt%1000==0 { println!("cnt: {} tovisit.len: {}", cnt, tovisit.len()); }
    }
    println!("{}: Lowest cost: {}", pfx, visited[y][x].cost);
}

fn main() {
    let mut part1 : Vec<Vec<usize>> = Vec::new();
    for line in io::stdin().lock().lines() {
        let txt = line.unwrap();
        let mut row : Vec<usize> = Vec::new();
        for c in txt.chars() {
            let r = usize::from_str_radix(&c.to_string(),10).unwrap();
            row.push(r);
        }
        part1.push(row);
    }
    route(&part1, "Part 1");
    // Part 2: increase map size, incrementing all values as we go... whee!
    let mut part2 : Vec<Vec<usize>> = Vec::new();
    for my in 0..5 {
        for y in 0..part1.len() {
            let mut row : Vec<usize> = Vec::new();
            for mx in 0..5 {
                for x in 0..part1[0].len() {
                    let v = (part1[y][x]+mx+my-1)%9+1;
                    row.push(v);
                }
            }
            part2.push(row);
        }
    }
    route(&part2, "Part 2");
}