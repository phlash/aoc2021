fn simulate(sx : isize, sy : isize, target : ((isize,isize),(isize,isize)), hy : &mut isize, hvx : &mut isize, hvy : &mut isize) -> bool {
    // run simulation - stop when y<target lower edge or we hit
    let mut vx : isize = sx;
    let mut vy : isize = sy;
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut thy : isize = isize::MIN;
    let mut hit : bool = false;
    while !hit && y>=target.1.0 {
        x += vx;
        y += vy;
        vx -= if vx>0 { 1 } else { 0 };
        vy -= 1;
        if x>=target.0.0 && x<=target.0.1 && y>=target.1.0 && y<=target.1.1 { hit=true; }
        if y>thy {
            thy = y;
        }
    }
    if hit && thy>*hy {
        *hy = thy;
        *hvx = sx;
        *hvy = sy;
    }
    return hit;
}

fn main() {
    //let target = ((20,30),(-10,-5));
    let target = ((119,176),(-141,-84));
    // probe x-position reaches the triangle number 1+2+...+n where n is the initial x-velocity
    // - this constrains the minimum value for x-velocity since this must be >= nearest edge
    // - thus: n(n+1)/2 >= <min-x>
    // - we try increasing values for n until this is true.
    let mut mx : isize = 0;
    while mx*(mx+1)/2 < target.0.0 { mx += 1; }
    println!("min x-vel: {}", mx);
    // maximum x-velocity is far edge of target, or we exceed after one step
    let xx : isize = target.0.1;
    // minimum y-velocity is target lower edge, or we exceed after one step
    let my : isize = target.1.0;
    // by way of some algebra, we note that y position always returns to zero
    // at a particular step (2<n>+1 where n=y-velocity), and that the next step
    // must not exceed the lower edge of the target area, where:
    // step-size = <n>-2<n>-1 => -<n>-1, thus:
    let xy : isize = -target.1.0+1;

    // OK: we have minimums.. now we are looking for maximum y value attainable
    // - search increasing values for y-velocity until we miss the target
    // - increase x-velocity and repeat.
    // - this exhausts our target hit space - we record highest y-value
    // - during simulation and velocities that generated it
    let mut hy : isize = isize::MIN;
    let mut hvx : isize = 0;
    let mut hvy : isize = 0;
    let mut hits : usize = 0;
    for sx in mx..xx+1 {
        for sy in my..xy {
            if simulate(sx, sy, target, &mut hy, &mut hvx, &mut hvy) {
                println!("hit@({},{})", sx, sy);
                hits += 1;
            }
        }
    }
    println!("@({},{}) max-y: {}, hits: {}", hvx, hvy, hy, hits);
}