use std::collections::HashMap;

// real game...
const p1start : usize = 4;
const p2start : usize = 5;

fn part1() {
    let mut die : usize = 100;
    let mut rolls : usize = 0;
    let mut scores : [usize; 2] = [0, 0];
    let mut positions : [usize; 2] = [p1start-1, p2start-1];
    let mut turn : usize = 0;
    while scores[0]<1000 && scores[1]<1000 {
        let mut mv : usize = 0;
        for _r in 0..3 { die = if die>=100 { 1 } else { die+1 }; mv += die; rolls += 1; }
        positions[turn] = (positions[turn]+mv) % 10;
        scores[turn] += positions[turn]+1;
        turn = 1-turn;
    }
    let score : usize;
    if scores[0]<1000 {
        println!("player 1 loses");
        score = scores[0];
    } else {
        println!("player 2 loses");
        score = scores[1];
    }
    println!("rolls: {} *losing: {}", rolls, score*rolls);
}

fn part2() {
    // a map of all possible states (player positions & scores), holding number of universes in which this state exists
    let mut state : HashMap<(usize, usize, usize, usize), u64> = HashMap::new();
    state.entry((p1start-1, 0, p2start-1, 0)).or_insert(1);
    // a map of universe split counts (permuatations that add up to the same) vs. die score after three rolls
    let mut splits : HashMap<usize, u64> = HashMap::new();
    splits.entry(3).or_insert(1);
    splits.entry(4).or_insert(3);
    splits.entry(5).or_insert(6);
    splits.entry(6).or_insert(7);
    splits.entry(7).or_insert(6);
    splits.entry(8).or_insert(3);
    splits.entry(9).or_insert(1);
    // keep iterating until we get winners for all game states
    let mut turn : bool = false;
    let mut wins : [u64; 2] = [0,0];
    loop {
        println!("state.len(): {}", state.len());
        let mut nstate = HashMap::new();
        // for all current player states
        for k in state.keys() {
            // for all possible die throws
            for d in splits.keys() {
                // move the player, increment score
                let pos = (if turn { k.2 } else { k.0 } + d) % 10;
                let score = if turn { k.3 } else { k.1 } + pos + 1;
                let univs = state[k] * splits[d];
                if score>20 {
                    // we have a winner in this number of universes
                    //println!("t:{} w:{},{} u:{}", turn, wins[0], wins[1], univs);
                    wins[if turn {1} else {0}] += univs;
                } else {
                    // update state if no winner yet
                    let nk = if turn { (k.0,k.1,pos,score) } else { (pos,score,k.2,k.3) };
                    *nstate.entry(nk).or_insert(0) += univs;
                }
            }
        }
        if nstate.is_empty() {
            println!("wins: p1={} p2={}", wins[0], wins[1]);
            return;
        }
        state = nstate;
        turn = !turn;
    }
}

fn main() {
    part1();
    part2();
}