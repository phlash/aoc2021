use std::io::{self,BufRead};

// we represent a cuboidal set of individual cubes by a co-ordinate pair, lowest x,y,z and highest x,y,z.
// we hold a list of all cuboids, then intersect each new cuboid, splitting any overlapping existing cuboid
// to retain smaller cuboids of 'on' cubes, finally we add the new cuboid if it is 'on'.
fn merge(cuboids : &Vec<(isize,isize,isize,isize,isize,isize)>, ncube : (isize,isize,isize,isize,isize,isize), on : bool) ->
    Vec<(isize,isize,isize,isize,isize,isize)> {
        println!("merging: {:?}", ncube);
        let mut ocubes : Vec<(isize,isize,isize,isize,isize,isize)> = Vec::new();
    for c in 0..cuboids.len() {
        // no overlap?
        let mut cb = cuboids[c];
        //println!("check: {:?}", cb);
        if  ncube.0>cb.3 || ncube.3<cb.0 || // x
            ncube.1>cb.4 || ncube.4<cb.1 || // y
            ncube.2>cb.5 || ncube.5<cb.2 {  // z
            ocubes.push(cb);
            continue;
        }
        // anything remaining below ncube.0 (low-x)
        if cb.0<ncube.0 {
            // split off as new cuboid, keep remainder
            ocubes.push((cb.0,cb.1,cb.2,ncube.0-1,cb.4,cb.5));
            cb = (ncube.0,cb.1,cb.2,cb.3,cb.4,cb.5);
            //println!("splitX-: {:?}", cb);
        }
        // anything remaining above ncube.3 (high-x)
        if cb.3>ncube.3 {
            ocubes.push((ncube.3+1,cb.1,cb.2,cb.3,cb.4,cb.5));
            cb = (cb.0,cb.1,cb.2,ncube.3,cb.4,cb.5);
            //println!("splitX+: {:?}", cb);
        }
        // repeat in y...
        if cb.1<ncube.1 {
            ocubes.push((cb.0,cb.1,cb.2,cb.3,ncube.1-1,cb.5));
            cb = (cb.0,ncube.1,cb.2,cb.3,cb.4,cb.5);
            //println!("splitY-: {:?}", cb);
        }
        if cb.4>ncube.4 {
            ocubes.push((cb.0,ncube.4+1,cb.2,cb.3,cb.4,cb.5));
            cb = (cb.0,cb.1,cb.2,cb.3,ncube.4,cb.5);
            //println!("splitY+: {:?}", cb);
        }
        // and finally in z..
        if cb.2<ncube.2 {
            ocubes.push((cb.0,cb.1,cb.2,cb.3,cb.4,ncube.2-1));
            cb = (cb.0,cb.1,ncube.2,cb.3,cb.4,cb.5);
            //println!("splitZ-: {:?}", cb);
        }
        if cb.5>ncube.5 {
            ocubes.push((cb.0,cb.1,ncube.5+1,cb.3,cb.4,cb.5));
            //cb = (cb.0,cb.1,cb.2,cb.3,cb.4,ncube.5);
            //println!("splitZ+: {:?}", cb);
        }
    }
    if on {
        // add new cuboid
        //println!("add: {:?}", ncube);
        ocubes.push(ncube);
    }
    return ocubes;
}

fn count(cubes : &Vec<(isize,isize,isize,isize,isize,isize)>) -> usize {
    let mut cnt : usize = 0;
    for c in 0..cubes.len() {
        let cb = cubes[c];
        cnt += ((cb.3-cb.0+1)*(cb.4-cb.1+1)*(cb.5-cb.2+1)) as usize;
    }
    return cnt;
}

fn main() {
    let mut cuboids : Vec<(isize,isize,isize,isize,isize,isize)> = Vec::new();
    for (lnum,line) in io::stdin().lock().lines().enumerate() {
        let txt = line.unwrap();
        // line format: on|off [<a>=<s>..<e>,]{3}
        let t1 : Vec<&str> = txt.split_whitespace().collect();
        if t1.len()!=2 { println!("invalid line: {}", txt); return; }
        let t2 : Vec<&str> = t1[1].split(',').collect();
        if t2.len()!=3 { println!("invalid co-ords: {}", t1[1]); return; }
        let ranges : Vec<(&str,isize,isize)> = t2.iter().map(
            |s| {
                let cv : Vec<&str> = s.split('=').collect();
                let se : Vec<&str> = cv[1].split("..").collect();
                return ( cv[0], isize::from_str_radix(se[0],10).unwrap(), isize::from_str_radix(se[1],10).unwrap() );
            }).collect();
        let on = match t1[0] {
            "on" => true,
            "off" => false,
            _ => unreachable!(),
        };
        if ranges.len()!=3 { println!("missing range?: {}", txt); return; }
        let mut sx : isize = 0;
        let mut sy : isize = 0;
        let mut sz : isize = 0;
        let mut ex : isize = 0;
        let mut ey : isize = 0;
        let mut ez : isize = 0;
        for i in 0..ranges.len() {
            let (s,e) = if ranges[i].1>ranges[i].2 { (ranges[i].2, ranges[i].1) } else { (ranges[i].1, ranges[i].2) };
            match ranges[i].0 {
                "x" => { sx = s; ex = e; },
                "y" => { sy = s; ey = e; },
                "z" => { sz = s; ez = e; },
                _ => unreachable!(),
            }
        }
        cuboids = merge(&cuboids, (sx, sy, sz, ex, ey, ez), on);
        if 19==lnum {
            println!("Part 1: count: {}", count(&cuboids));
        }
    }
    println!("Part2: count: {}", count(&cuboids));
}